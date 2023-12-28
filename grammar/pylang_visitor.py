#!/usr/bin/env python3
from grammar import PythonParserVisitor, PythonParser
import llvmlite.ir as ir
import llvmlite.binding as llvm
import argparse

class bcolors:
    HEADER = '\033[95m'
    OKBLUE = '\033[94m'
    OKCYAN = '\033[96m'
    OKGREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'

def log(msg):
    print(bcolors.WARNING + msg + bcolors.ENDC)

def print_tree(ctx, parser):
    from antlr4.tree.Trees import Trees

    import pprint
    pprint.pprint(Trees.toStringTree(ctx, None, parser))

class PyLangVisitor(PythonParserVisitor):
    def __init__(self, args: argparse.Namespace, parser=None):
        self.optimize = args.optimize
        self._parser = parser

    #def visitCompilationUnit(self, ctx: PyLangParser.CompilationUnitContext):
    def visitFile_input(self, ctx:PythonParser.File_inputContext):
        # global function table
        # functions[function-name] = function-pointer
        self.functions = {}
        # global variable table
        # variables[function-name][variable-name] = variable-pointer
        self.variables = {}
        self.current_function = ''

        # llvm init
        llvm.initialize()
        llvm.initialize_native_target()
        llvm.initialize_native_asmprinter()

        # set target
        target = llvm.Target.from_default_triple().create_target_machine()

        # define types
        self.i64 = ir.IntType(64)
        self.f64 = ir.FloatType()

        self.module = ir.Module(name='pylang_module')
        self.module.triple = target.triple
        # function prototype (external linkage implemented in builtin.c) for
        # void write(int64_t)
        ftype_write = ir.FunctionType(ir.VoidType(), [self.i64])
        self.fn_write = ir.Function(self.module, ftype_write, name='write')

        self.visitChildren(ctx)

        # generate code
        llvm_ir = str(self.module)

        print("DEBUG: ", llvm_ir)

        llvm_ir_parsed = llvm.parse_assembly(llvm_ir)

        # optimizer
        if self.optimize:
            pmb = llvm.create_pass_manager_builder()
            pmb.opt_level = 3
            pm = llvm.create_module_pass_manager()
            pmb.populate(pm)
            pm.run(llvm_ir_parsed)

        with open('build/out.ll', 'w') as f:
            f.write(str(llvm_ir_parsed))

        return None

    def visitFunction_def(self, ctx:PythonParser.Function_defContext):
        if ctx.function_def_raw() is not None:
            ctx_func = ctx.function_def_raw()
            name = ctx_func.NAME().getText()
            
            print("DEGUB: ", name)
            block_name = 'entry'
            self.current_function = name
            self.variables[self.current_function] = {}

            params = []
            paramnames = []
            if ctx_func.params() is not None:
                if ctx_func.params().parameters() is not None:
                    def_params = ctx_func.params().parameters().children
                    if def_params is not None:
                        print("func_params: ", def_params)
                        for paramdef in def_params:
                            params.append(self.i64)
                            paramnames.append(paramdef.param().NAME().getText())

            # register function
            ftype = ir.FunctionType(self.i64, params)
            func = ir.Function(self.module, ftype, name=name)
            entrybb = func.append_basic_block(name=block_name)
            retbb = ir.Block(entrybb, name='_ret')
            
            self.functions[name] = {
                'func': func,
                'entrybb': entrybb,
                'retbb': retbb
            }

            # make a block for func entry
            self.builder = ir.IRBuilder(entrybb)

            # define variables for the paramnames
            for paramname in paramnames:
                var = self.builder.alloca(self.i64, size=8, name=paramname)
                print("var def: ", name, var)
                self.variables[self.current_function][paramname] = var

            # create _ret variable
            var = self.builder.alloca(self.i64, size=8, name='_ret')
            self.variables[self.current_function]['_ret'] = var

            # store parameter values to the variables
            i = 0
            for paramname in paramnames:
                ptr = self.variables[self.current_function][paramname]
                value = func.args[i]
                self.builder.store(value, ptr)
                i += 1

            ret = self.visitChildren(ctx)

            # make a block for ret
            func.basic_blocks.append(retbb)
            self.builder = ir.IRBuilder(retbb)
            ptr = self.variables[self.current_function]['_ret']
            value = self.builder.load(ptr, name)
            self.builder.ret(value)

            # ret is always None
            return ret


    # Visit a parse tree produced by PythonParser#primary.
    def visitPrimary(self, ctx:PythonParser.PrimaryContext):
        
        # call func()
        if ctx.primary() is not None:
            func_name = ctx.primary().getText()
                    
            # call write stdlib
            if func_name == 'write':
                #print_tree(ctx, self._parser)
                args = []
                
                for arg in ctx.arguments().children:
                    name = self.visitChildren(arg)

                    if name in self.variables[self.current_function]:
                        ptr = self.variables[self.current_function][name]
                    
                        ret_load = self.builder.load(ptr, name)
                        args.append(ret_load)
                    else:
                        args.append(name)
                    
                ret = self.builder.call(self.fn_write, (args), name='write')
            
                return ret
            else:
                if ctx.arguments():
                    # call function    
                    #print("atom_expr args: ", self.visitChildren(ctx.arguments().args()))

                    args = []

                    if ctx.arguments().args():            
                        for arg in ctx.arguments().children:
                            #name = arg.getText()
                            name = self.visitChildren(arg)

                            if name in self.variables[self.current_function]:
                                ptr = self.variables[self.current_function][name]
                            
                                ret_load = self.builder.load(ptr, name)
                                args.append(ret_load)
                            else:
                                args.append(name)
                    
                    ret = self.builder.call(self.functions[func_name]['func'], args, name=func_name)
                            
                    print("args: ", args)
                    return ret
        
        return self.visitChildren(ctx)

    def visitAtom(self, ctx:PythonParser.AtomContext):
        # print("ATOM all: ", ctx.getText())

        if ctx.NUMBER() is not None:
            # print("ATOM number: ", ctx.NUMBER())
            return ir.Constant(self.i64, int(ctx.NUMBER().getText()))
        
        if ctx.NAME() is not None:
            name = ctx.NAME().getText()
            # print("ATOM name: ", name)
            if name in self.variables[self.current_function]:
                ptr = self.variables[self.current_function][name]
            
                ret = self.builder.load(ptr, name)
                return ret
            return ir.Constant(self.i64, name)
                
        return self.visitChildren(ctx)

    def visitReturn_stmt(self, ctx: PythonParser.Return_stmtContext):
        value = self.visit(ctx.star_expressions())
        # print("DEBUG return: ", value)
        
        #value = ir.Constant(self.i64, int(value))

        ptr = self.variables[self.current_function]['_ret']
        self.builder.store(value, ptr)
        self.builder.branch(self.functions[self.current_function]['retbb'])
        return None


    # Visit a parse tree produced by PythonParser#assignment.
    def visitAssignment(self, ctx:PythonParser.AssignmentContext):
        print_tree(ctx,self._parser)
        if ctx.children[2].start.text == '[':
            name = ctx.star_targets()[0].getText()
            array_type = ir.ArrayType(self.i64, 3)
            var = self.builder.alloca(array_type, name=name)
            self.variables[self.current_function][name] = var   
        else:   
            name = ctx.star_targets()[0].getText()
            var = self.builder.alloca(self.i64, size=8, name=name)
            self.variables[self.current_function][name] = var
        
        value = self.visit(ctx.children[2])
        ptr = self.variables[self.current_function][name]
        self.builder.store(value, ptr)

    # Visit a parse tree produced by PythonParser#sum.
    def visitSum(self, ctx:PythonParser.SumContext):  
        
        if ctx.PLUS() is not None:
            lhs = self.visit(ctx.children[0])
            rhs = self.visit(ctx.children[2])
            log("PLUS: ");print();print(ctx.getText())
            ret = self.builder.add(lhs, rhs, name='add_tmp')
            print(ret)
            return ret
        
        if ctx.MINUS() is not None:
            lhs = self.visit(ctx.children[0])
            rhs = self.visit(ctx.children[2])
            log("MINUS: ");print();print(ctx.getText())
            ret = self.builder.sub(lhs, rhs, name='sub_tmp')
            print(ret)
            return ret
        
        return self.visitChildren(ctx)
    
    # Visit a parse tree produced by PythonParser#if_stmt.
    def visitIf_stmt(self, ctx:PythonParser.If_stmtContext):
        cond = self.visit(ctx.named_expression())
        with self.builder.if_else(cond) as (then, otherwise):
            with then:
                self.visit(ctx.block())
            with otherwise:
                if ctx.else_block() is not None:
                    self.visit(ctx.else_block())


    # Visit a parse tree produced by PythonParser#elif_stmt.
    def visitElif_stmt(self, ctx:PythonParser.Elif_stmtContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by PythonParser#else_block.
    def visitElse_block(self, ctx:PythonParser.Else_blockContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by PythonParser#comparison.
    def visitComparison(self, ctx:PythonParser.ComparisonContext):
        if ctx.compare_op_bitwise_or_pair():
            if ctx.compare_op_bitwise_or_pair(0).gt_bitwise_or():
                log("Cond > ");print(ctx.getText())
                # print(
                #     self.visit(ctx.bitwise_or()), 
                #     self.visit(ctx.compare_op_bitwise_or_pair(0).gt_bitwise_or())
                # )
                return self.builder.icmp_signed('>',
                    self.visit(ctx.bitwise_or()),
                    self.visit(ctx.compare_op_bitwise_or_pair(0).gt_bitwise_or()), 'gt')

            if ctx.compare_op_bitwise_or_pair(0).lt_bitwise_or():
                log("Cond < ");print(ctx.getText())
                # print(
                #     self.visit(ctx.bitwise_or()), 
                #     self.visit(ctx.compare_op_bitwise_or_pair(0).gt_bitwise_or())
                # )
                return self.builder.icmp_signed('<',
                    self.visit(ctx.bitwise_or()),
                    self.visit(ctx.compare_op_bitwise_or_pair(0).lt_bitwise_or()), 'lt')

            if ctx.compare_op_bitwise_or_pair(0).eq_bitwise_or():
                log("Cond == ");print(ctx.getText())
                # print(
                #     self.visit(ctx.bitwise_or()), 
                #     self.visit(ctx.compare_op_bitwise_or_pair(0).gt_bitwise_or())
                # )
                return self.builder.icmp_signed('==',
                    self.visit(ctx.bitwise_or()),
                    self.visit(ctx.compare_op_bitwise_or_pair(0).eq_bitwise_or()), 'eq')

            if ctx.compare_op_bitwise_or_pair(0).ne_bitwise_or():
                log("Cond != ");print(ctx.getText())
                # print(
                #     self.visit(ctx.bitwise_or()), 
                #     self.visit(ctx.compare_op_bitwise_or_pair(0).gt_bitwise_or())
                # )
                return self.builder.icmp_signed('!=',
                    self.visit(ctx.bitwise_or()),
                    self.visit(ctx.compare_op_bitwise_or_pair(0).eq_bitwise_or()), 'ne')


        return self.visitChildren(ctx)


    # Visit a parse tree produced by PythonParser#named_expression.
    def visitNamed_expression(self, ctx:PythonParser.Named_expressionContext):
        return self.visitChildren(ctx)
    
        # Visit a parse tree produced by PythonParser#for_stmt.
    def visitFor_stmt(self, ctx:PythonParser.For_stmtContext):
        print_tree(ctx, self._parser)
        return self.visitChildren(ctx)

    # Visit a parse tree produced by PythonParser#list.
    def visitList(self, ctx:PythonParser.ListContext):
        array_example = [3,5,8]
        array_type = ir.ArrayType(self.i64, 3) #According to documentation, the second argument has to be an Python Integer. It can't be ir.Constant(i32, 3) for example.
        return ir.Constant(array_type, array_example)
        #return ir.Constant(self.i64, int(555777))
    
    # def visitCond(self, ctx: PyLangParser.CondContext):
    #     if len(ctx.children) == 1:
    #         # expr
    #         return self.visit(ctx.children[0])
    #     else:
    #         # expr cond expr
    #         if ctx.children[1].getText() == '==':
    #             return self.builder.icmp_signed(
    #                 '==', self.visit(
    #                     ctx.expr(0)), self.visit(
    #                     ctx.expr(1)), 'eq')
    #         elif ctx.children[1].getText() == '!=':
    #             return self.builder.icmp_signed(
    #                 '!=', self.visit(
    #                     ctx.expr(0)), self.visit(
    #                     ctx.expr(1)), 'ne')
    #         elif ctx.children[1].getText() == '<=':
    #             return self.builder.icmp_signed(
    #                 '<=', self.visit(
    #                     ctx.expr(0)), self.visit(
    #                     ctx.expr(1)), 'le')
    #         elif ctx.children[1].getText() == '<':
    #             return self.builder.icmp_signed(
    #                 '<', self.visit(
    #                     ctx.expr(0)), self.visit(
    #                     ctx.expr(1)), 'lt')
    #         elif ctx.children[1].getText() == '>=':
    #             return self.builder.icmp_signed(
    #                 '>=', self.visit(
    #                     ctx.expr(0)), self.visit(
    #                     ctx.expr(1)), 'ge')
    #         elif ctx.children[1].getText() == '>':
    #             return self.builder.icmp_signed(
    #                 '>', self.visit(
    #                     ctx.expr(0)), self.visit(
    #                     ctx.expr(1)), 'gt')

    # def visitUnaryExpr(self, ctx: PyLangParser.UnaryExprContext):
    #     ret = self.visit(ctx.children[1])
    #     if ctx.children[0].getText() == '-':
    #         ret = self.builder.mul(
    #             ret, ir.Constant(
    #                 self.i64, -1), name='mul_tmp')
    #     return ret

    # def visitMulDivExpr(self, ctx: PyLangParser.MulDivExprContext):
    #     lhs = self.visit(ctx.expr(0))
    #     rhs = self.visit(ctx.expr(1))
    #     if ctx.children[1].getText() == '*':
    #         ret = self.builder.mul(lhs, rhs, name='mul_tmp')
    #     else:
    #         ret = self.builder.sdiv(lhs, rhs, name='div_tmp')
    #     return ret


