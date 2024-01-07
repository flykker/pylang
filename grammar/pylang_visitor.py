from grammar import PythonParserVisitor, PythonParser
import llvmlite.ir as ir
import llvmlite.binding as llvm
import argparse
import ast
from antlr4.xpath.XPath import XPath
from ctypes import c_void_p, c_int64, CFUNCTYPE

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

def lib_builtin(module):
    voidptr_ty = ir.IntType(8).as_pointer()

    fmt = "%s\0"
    c_fmt = ir.Constant(ir.ArrayType(ir.IntType(8), len(fmt)), bytearray(fmt.encode("utf8")))
    global_fmt = ir.GlobalVariable(module, c_fmt.type, name="fstr")
    global_fmt.linkage = 'internal'
    global_fmt.global_constant = True
    global_fmt.initializer = c_fmt

    return global_fmt


class PyLangVisitor(PythonParserVisitor):
    def __init__(self, args: argparse.Namespace, parser=None):
        self.optimize = args.optimize
        self.compile = args.compile
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
        self.voidptr_ty = ir.IntType(8).as_pointer()
        self.i8 = ir.IntType(8)
        self.i32 = ir.IntType(32)
        self.i64 = ir.IntType(64)
        self.f64 = ir.FloatType()

        self.module = ir.Module(name='pylang_module')
        self.module.triple = target.triple
 
        printf_ty = ir.FunctionType(self.i32, [self.voidptr_ty])
        self.printf = ir.Function(self.module, printf_ty, name="printf")

        itostr_ty = ir.FunctionType(self.voidptr_ty, [self.i32, self.voidptr_ty, self.i32])
        self.itostr = ir.Function(self.module, itostr_ty, name="itostr")

        self.visitChildren(ctx)

        # generate code
        llvm_ir = str(self.module)

        print("DEBUG: ", llvm_ir)

        llvm_ir_parsed = llvm.parse_assembly(llvm_ir)
        llvm_ir_parsed.data_layout = ""

        # optimizer
        if self.optimize:
            pmb = llvm.create_pass_manager_builder()
            pmb.opt_level = 3
            pm = llvm.create_module_pass_manager()
            pmb.populate(pm)
            pm.run(llvm_ir_parsed)

        llvm_ir_parsed.data_layout = ""
        with open('build/app.ll', 'w') as f:
            f.write(str(llvm_ir_parsed))
        
        f_lib_mod = open("build/builtin.ll", 'r')
        llvm_ir_lib = llvm.parse_assembly(f_lib_mod.read())
        llvm_ir_lib.link_in(llvm_ir_parsed, preserve=True)

        if self.optimize:
            pmb = llvm.create_pass_manager_builder()
            pmb.opt_level = 3
            pm = llvm.create_module_pass_manager()
            pmb.populate(pm)
            pm.run(llvm_ir_lib)

        with open('build/linked.ll', 'w') as f:
            f.write(str(llvm_ir_lib))
        
        # engine = llvm.create_mcjit_compiler(llvm_ir_parsed, target)
        # engine.finalize_object()
        # engine.run_static_constructors()
        # get_func_main = engine.get_function_address("main")
        # func_main = CFUNCTYPE(c_int64)(get_func_main)        
        # if not self.compile:
        #     func_main()
            

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
            
            if func_name == 'print':
                
                arg = "\n\0"
                if ctx.arguments() is not None:
                    name = ctx.arguments().getText()

                    if name in self.variables[self.current_function]:
                        ptr = self.variables[self.current_function][name]    
                        int_arg = self.builder.load(ptr, name)

                        int_arg = self.builder.trunc(int_arg, self.i32)
                        char_buf = self.builder.alloca(ir.ArrayType(ir.IntType(8), 33))
                        buffer = self.builder.bitcast(char_buf, self.voidptr_ty)
                        base = ir.Constant(self.i32, 10)
                        #v = ir.Constant(self.i32, 44556677)
                        ret_char = self.builder.call(self.itostr, [int_arg, buffer, base])
                        ret = self.builder.call(self.printf, [ret_char])
                        return ret
                    else:
                        arg = ctx.arguments().getText() + '\0'            

                        c_str_val = ir.Constant(ir.ArrayType(ir.IntType(8), len(arg)), bytearray(arg.encode("utf8")))
                        c_str = self.builder.alloca(c_str_val.type)
                        self.builder.store(c_str_val, c_str)
                        char_buf = self.builder.bitcast(c_str, self.voidptr_ty)
                        ret = self.builder.call(self.printf, [char_buf])
                        return ret
                        
                c_str_val = ir.Constant(ir.ArrayType(ir.IntType(8), len(arg)), bytearray(arg.encode("utf8")))
                c_str = self.builder.alloca(c_str_val.type)
                self.builder.store(c_str_val, c_str)
                char_buf = self.builder.bitcast(c_str, self.voidptr_ty)
                ret = self.builder.call(self.printf, [char_buf])
                return ret
            
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
            
            if ctx.slices():
                # Parse array get,set element
                log('slices: ');print(ctx.primary().getText());print(ctx.slices().getText())
                int64_0 = ir.Constant(self.i64, 0)
                index = self.visit(ctx.slices())
                
                name = ctx.primary().getText()

                if name in self.variables[self.current_function]:
                    ptr = self.variables[self.current_function][name]
                
                address = self.builder.gep(ptr, [int64_0, index['slices']])

                return self.builder.load(address)
        
        return self.visitChildren(ctx)

    # Visit a parse tree produced by PythonParser#t_primary.
    def visitT_primary(self, ctx:PythonParser.T_primaryContext):
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

        ptr = self.variables[self.current_function]['_ret']
        self.builder.store(value, ptr)
        self.builder.branch(self.functions[self.current_function]['retbb'])
        return None #self.visitChildren(ctx)


    # Visit a parse tree produced by PythonParser#assignment.
    def visitAssignment(self, ctx:PythonParser.AssignmentContext):
        #print_tree(ctx,self._parser)
        slices = XPath(ctx.parser, '').findAll(ctx.star_targets(0),'//slices',ctx.parser)
        value = None

        if ctx.children[2].start.text == '[':
            name = ctx.star_targets()[0].getText()
            #array_type = ir.ArrayType(self.i64, 3)
            value = self.visit(ctx.children[2])
            var = self.builder.alloca(value.type, name=name)
            self.variables[self.current_function][name] = var
            ptr = self.variables[self.current_function][name]
            self.builder.store(value, ptr)     
        elif slices:
            name = XPath(ctx.parser, '').findAll(ctx,'//t_primary',ctx.parser)[0].getText()
            index = self.visit(slices[0])
            
            int64_0 = ir.Constant(self.i64, 0)
            ptr = None
            if name in self.variables[self.current_function]:
                ptr = self.variables[self.current_function][name]
            
            array_address = self.builder.gep(ptr, [int64_0, index['slices']])
            value = self.visit(ctx.children[2])
            self.builder.store(value, array_address)
        else:   
            name = ctx.star_targets()[0].getText()
            value = self.visit(ctx.children[2])

            ptr = None
            if name in self.variables[self.current_function]:
                ptr = self.variables[self.current_function][name]
            else:
                var = self.builder.alloca(self.i64, size=8, name=name)
                self.variables[self.current_function][name] = var
                ptr = self.variables[self.current_function][name]
            
            self.builder.store(value, ptr)

        #return self.visitChildren(ctx)



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

            if ctx.compare_op_bitwise_or_pair(0).gte_bitwise_or():
                log("Cond >= ");print(ctx.getText())
                return self.builder.icmp_signed('>=',
                    self.visit(ctx.bitwise_or()),
                    self.visit(ctx.compare_op_bitwise_or_pair(0).gte_bitwise_or()), 'gte')


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

    # Visit a parse tree produced by PythonParser#slices.
    def visitSlices(self, ctx:PythonParser.SlicesContext):
        log("slices2: ")
        value = self.visitChildren(ctx)
        print("slice value: ", value)
        return {"slices": value}

    # Visit a parse tree produced by PythonParser#named_expression.
    def visitNamed_expression(self, ctx:PythonParser.Named_expressionContext):
        return self.visitChildren(ctx)
    
        # Visit a parse tree produced by PythonParser#for_stmt.
    def visitFor_stmt(self, ctx:PythonParser.For_stmtContext):
        print_tree(ctx, self._parser)
        
        i64_0 = ir.Constant(self.i64,0)
        i64_1 = ir.Constant(self.i64,1)

        name = ctx.star_targets().getText()
        iter = self.visit(ctx.star_expressions())
        iter_len =ir.Constant(self.i64, iter.type.count)
        
        ptr = self.builder.alloca(iter.type) #allocate memory
        self.builder.store(iter, ptr)

        #add variable 'array' to the symbol table
        symbol_table = {"array":ptr}

        #
        for_body_block = self.builder.append_basic_block("for.body")
        for_after_block = self.builder.append_basic_block("for.after")

        #initiailize i = 0
        #for i in ...
        i_ptr = self.builder.alloca(self.i64)
        i_value = i64_0
        self.builder.store(i_value, i_ptr) #store the value 0 to the address allocated
        
        var = self.builder.alloca(self.i64, name=name)
        self.variables[self.current_function][name] = var
        ptr = self.variables[self.current_function][name]
             
        symbol_table["i"] = i_ptr

        #does the initial i <3; Since i = 0, this is trivial

        current_i_value = self.builder.load(symbol_table["i"])
        cond_head = self.builder.icmp_signed('<', current_i_value, iter_len, name="lt") #returns boolean, which is ir.IntType(1)

        #branches depending on whether cond_head is true or false
        self.builder.cbranch(cond_head, for_body_block, for_after_block)
        self.builder.position_at_start(for_body_block)

        # for.body
        current_i_value = self.builder.load(symbol_table["i"]) #gets value of i (0,1 or 2)
        array_i_pointer = self.builder.gep(symbol_table["array"], [i64_0,current_i_value]) #accesses array[i]
        array_i_value = self.builder.load(array_i_pointer)
        self.builder.store(array_i_value, ptr)
        self.visit(ctx.block())

        
        # new_array_i_value = self.builder.add(array_i_value, i64_1, name="add") #array[i] + 1
        # self.builder.store(new_array_i_value, array_i_pointer) #store the new value of array[i]

        #i++
        new_i_value = self.builder.add(current_i_value, i64_1, name="add_i")
        self.builder.store(new_i_value, symbol_table["i"]) #store the new value of i at the i pointer

        #compare i < 3
        cond_body = self.builder.icmp_signed('<', new_i_value, iter_len, name="lt")
        self.builder.cbranch(cond_body, for_body_block, for_after_block) #iterate again if true, leave if false

        # after
        self.builder.position_at_start(for_after_block)

        #return self.visitChildren(ctx)

    # Visit a parse tree produced by PythonParser#while_stmt.
    def visitWhile_stmt(self, ctx:PythonParser.While_stmtContext):
        cond = self.visit(ctx.named_expression())
        
        # Entry (block where that runs if the condition is true)
        while_loop_entry = self.builder.append_basic_block("while_loop_entry")

        # If the condition is not true it runs from here
        while_loop_end = self.builder.append_basic_block("while_loop_end")

        # Creating a condition branch
        #     condition
        #        / \
        # if true   if false
        #       /   \
        #      /     \
        # true block  false block
        self.builder.cbranch(cond, while_loop_entry, while_loop_end)

        # Setting the builder position-at-start
        self.builder.position_at_start(while_loop_entry)
        self.visit(ctx.block())
        
        cond = self.visit(ctx.named_expression())
        self.builder.cbranch(cond, while_loop_entry, while_loop_end)
        self.builder.position_at_start(while_loop_end)
        #return self.visitChildren(ctx)

    # Visit a parse tree produced by PythonParser#list.
    def visitList(self, ctx:PythonParser.ListContext):
        array = ast.literal_eval(ctx.getText())
        array_type = ir.ArrayType(self.i64, len(array))
        return ir.Constant(array_type, array)
    
    # Visit a parse tree produced by PythonParser#slices.
    # def visitSlices(self, ctx:PythonParser.SlicesContext):
    #     log("slices: ");print(ctx.getText())
    #     return ir.Constant(self.i64, int(555777))
    #     return self.visitChildren(ctx)

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

    # Visit a parse tree produced by PythonParser#term.
    def visitTerm(self, ctx:PythonParser.TermContext):
        if ctx.PERCENT() is not None:
            log('Term: ');print(ctx.getText())
            lhs = self.visit(ctx.term())
            rhs = self.visit(ctx.term())
            return self.builder.srem(lhs,rhs)
        
        if ctx.SLASH() is not None:
            log('Term: ');print(ctx.getText())
            lhs = self.visit(ctx.term())
            rhs = self.visit(ctx.term())
            return self.builder.sdiv(lhs,rhs)

        return self.visitChildren(ctx)

    # Visit a parse tree produced by PythonParser#strings.
    def visitStrings(self, ctx:PythonParser.StringsContext):
        log('Strings: ');print(ctx.getText())
        string = ctx.getText()[1:-1]

        return ir.Constant(ir.ArrayType(ir.IntType(8), len(string)), bytearray(string.encode("utf8")))
        #return self.visitChildren(ctx)