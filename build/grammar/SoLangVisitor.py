# Generated from grammar/SoLang.g4 by ANTLR 4.13.1
from antlr4 import *
if "." in __name__:
    from .SoLangParser import SoLangParser
else:
    from SoLangParser import SoLangParser

# This class defines a complete generic visitor for a parse tree produced by SoLangParser.

class SoLangVisitor(ParseTreeVisitor):

    # Visit a parse tree produced by SoLangParser#compilationUnit.
    def visitCompilationUnit(self, ctx:SoLangParser.CompilationUnitContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SoLangParser#function.
    def visitFunction(self, ctx:SoLangParser.FunctionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SoLangParser#block.
    def visitBlock(self, ctx:SoLangParser.BlockContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SoLangParser#exprStmt.
    def visitExprStmt(self, ctx:SoLangParser.ExprStmtContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SoLangParser#variableDefinitionStmt.
    def visitVariableDefinitionStmt(self, ctx:SoLangParser.VariableDefinitionStmtContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SoLangParser#asgnStmt.
    def visitAsgnStmt(self, ctx:SoLangParser.AsgnStmtContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SoLangParser#ifStmt.
    def visitIfStmt(self, ctx:SoLangParser.IfStmtContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SoLangParser#writeStmt.
    def visitWriteStmt(self, ctx:SoLangParser.WriteStmtContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SoLangParser#returnStmt.
    def visitReturnStmt(self, ctx:SoLangParser.ReturnStmtContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SoLangParser#if_stmt.
    def visitIf_stmt(self, ctx:SoLangParser.If_stmtContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SoLangParser#else_block.
    def visitElse_block(self, ctx:SoLangParser.Else_blockContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SoLangParser#elseif_block.
    def visitElseif_block(self, ctx:SoLangParser.Elseif_blockContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SoLangParser#cond.
    def visitCond(self, ctx:SoLangParser.CondContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SoLangParser#identExpr.
    def visitIdentExpr(self, ctx:SoLangParser.IdentExprContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SoLangParser#parExpr.
    def visitParExpr(self, ctx:SoLangParser.ParExprContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SoLangParser#unaryExpr.
    def visitUnaryExpr(self, ctx:SoLangParser.UnaryExprContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SoLangParser#addSubExpr.
    def visitAddSubExpr(self, ctx:SoLangParser.AddSubExprContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SoLangParser#numberExpr.
    def visitNumberExpr(self, ctx:SoLangParser.NumberExprContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SoLangParser#functionCallExpr.
    def visitFunctionCallExpr(self, ctx:SoLangParser.FunctionCallExprContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SoLangParser#mulDivExpr.
    def visitMulDivExpr(self, ctx:SoLangParser.MulDivExprContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SoLangParser#paramdefs.
    def visitParamdefs(self, ctx:SoLangParser.ParamdefsContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SoLangParser#paramdef.
    def visitParamdef(self, ctx:SoLangParser.ParamdefContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SoLangParser#params.
    def visitParams(self, ctx:SoLangParser.ParamsContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SoLangParser#param.
    def visitParam(self, ctx:SoLangParser.ParamContext):
        return self.visitChildren(ctx)



del SoLangParser