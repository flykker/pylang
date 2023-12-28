# Generated from grammar/SoLang.g4 by ANTLR 4.13.1
# encoding: utf-8
from antlr4 import *
from io import StringIO
import sys
if sys.version_info[1] > 5:
	from typing import TextIO
else:
	from typing.io import TextIO

def serializedATN():
    return [
        4,1,28,176,2,0,7,0,2,1,7,1,2,2,7,2,2,3,7,3,2,4,7,4,2,5,7,5,2,6,7,
        6,2,7,7,7,2,8,7,8,2,9,7,9,2,10,7,10,2,11,7,11,2,12,7,12,1,0,4,0,
        28,8,0,11,0,12,0,29,1,1,1,1,1,1,1,1,3,1,36,8,1,1,1,1,1,1,1,1,2,1,
        2,4,2,43,8,2,11,2,12,2,44,1,2,1,2,1,3,1,3,1,3,1,3,1,3,1,3,1,3,1,
        3,1,3,1,3,1,3,1,3,1,3,1,3,1,3,1,3,1,3,1,3,1,3,1,3,1,3,1,3,3,3,71,
        8,3,1,4,1,4,1,4,1,4,1,4,1,4,3,4,79,8,4,1,5,5,5,82,8,5,10,5,12,5,
        85,9,5,1,5,1,5,1,5,1,6,1,6,1,6,1,6,1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,
        7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,
        7,1,7,3,7,119,8,7,1,8,1,8,1,8,1,8,1,8,1,8,1,8,1,8,1,8,1,8,1,8,3,
        8,132,8,8,1,8,1,8,3,8,136,8,8,1,8,1,8,1,8,1,8,1,8,1,8,5,8,144,8,
        8,10,8,12,8,147,9,8,1,9,1,9,1,9,1,9,1,9,1,9,5,9,155,8,9,10,9,12,
        9,158,9,9,1,10,1,10,1,10,1,11,1,11,1,11,1,11,1,11,1,11,5,11,169,
        8,11,10,11,12,11,172,9,11,1,12,1,12,1,12,0,3,16,18,22,13,0,2,4,6,
        8,10,12,14,16,18,20,22,24,0,2,1,0,18,19,1,0,20,21,187,0,27,1,0,0,
        0,2,31,1,0,0,0,4,40,1,0,0,0,6,70,1,0,0,0,8,72,1,0,0,0,10,83,1,0,
        0,0,12,89,1,0,0,0,14,118,1,0,0,0,16,135,1,0,0,0,18,148,1,0,0,0,20,
        159,1,0,0,0,22,162,1,0,0,0,24,173,1,0,0,0,26,28,3,2,1,0,27,26,1,
        0,0,0,28,29,1,0,0,0,29,27,1,0,0,0,29,30,1,0,0,0,30,1,1,0,0,0,31,
        32,5,1,0,0,32,33,5,23,0,0,33,35,5,2,0,0,34,36,3,18,9,0,35,34,1,0,
        0,0,35,36,1,0,0,0,36,37,1,0,0,0,37,38,5,3,0,0,38,39,3,4,2,0,39,3,
        1,0,0,0,40,42,5,4,0,0,41,43,3,6,3,0,42,41,1,0,0,0,43,44,1,0,0,0,
        44,42,1,0,0,0,44,45,1,0,0,0,45,46,1,0,0,0,46,47,5,5,0,0,47,5,1,0,
        0,0,48,49,3,16,8,0,49,50,5,6,0,0,50,71,1,0,0,0,51,52,5,1,0,0,52,
        53,5,23,0,0,53,71,5,6,0,0,54,55,5,23,0,0,55,56,5,7,0,0,56,57,3,16,
        8,0,57,58,5,6,0,0,58,71,1,0,0,0,59,71,3,8,4,0,60,61,5,8,0,0,61,62,
        5,2,0,0,62,63,3,16,8,0,63,64,5,3,0,0,64,65,5,6,0,0,65,71,1,0,0,0,
        66,67,5,9,0,0,67,68,3,16,8,0,68,69,5,6,0,0,69,71,1,0,0,0,70,48,1,
        0,0,0,70,51,1,0,0,0,70,54,1,0,0,0,70,59,1,0,0,0,70,60,1,0,0,0,70,
        66,1,0,0,0,71,7,1,0,0,0,72,73,5,10,0,0,73,74,5,2,0,0,74,75,3,14,
        7,0,75,76,5,3,0,0,76,78,3,4,2,0,77,79,3,10,5,0,78,77,1,0,0,0,78,
        79,1,0,0,0,79,9,1,0,0,0,80,82,3,12,6,0,81,80,1,0,0,0,82,85,1,0,0,
        0,83,81,1,0,0,0,83,84,1,0,0,0,84,86,1,0,0,0,85,83,1,0,0,0,86,87,
        5,11,0,0,87,88,3,4,2,0,88,11,1,0,0,0,89,90,5,11,0,0,90,91,5,10,0,
        0,91,92,3,4,2,0,92,13,1,0,0,0,93,94,3,16,8,0,94,95,5,12,0,0,95,96,
        3,16,8,0,96,119,1,0,0,0,97,98,3,16,8,0,98,99,5,13,0,0,99,100,3,16,
        8,0,100,119,1,0,0,0,101,102,3,16,8,0,102,103,5,14,0,0,103,104,3,
        16,8,0,104,119,1,0,0,0,105,106,3,16,8,0,106,107,5,15,0,0,107,108,
        3,16,8,0,108,119,1,0,0,0,109,110,3,16,8,0,110,111,5,16,0,0,111,112,
        3,16,8,0,112,119,1,0,0,0,113,114,3,16,8,0,114,115,5,17,0,0,115,116,
        3,16,8,0,116,119,1,0,0,0,117,119,3,16,8,0,118,93,1,0,0,0,118,97,
        1,0,0,0,118,101,1,0,0,0,118,105,1,0,0,0,118,109,1,0,0,0,118,113,
        1,0,0,0,118,117,1,0,0,0,119,15,1,0,0,0,120,121,6,8,-1,0,121,122,
        7,0,0,0,122,136,3,16,8,7,123,124,5,2,0,0,124,125,3,16,8,0,125,126,
        5,3,0,0,126,136,1,0,0,0,127,136,5,24,0,0,128,129,5,23,0,0,129,131,
        5,2,0,0,130,132,3,22,11,0,131,130,1,0,0,0,131,132,1,0,0,0,132,133,
        1,0,0,0,133,136,5,3,0,0,134,136,5,23,0,0,135,120,1,0,0,0,135,123,
        1,0,0,0,135,127,1,0,0,0,135,128,1,0,0,0,135,134,1,0,0,0,136,145,
        1,0,0,0,137,138,10,6,0,0,138,139,7,1,0,0,139,144,3,16,8,7,140,141,
        10,5,0,0,141,142,7,0,0,0,142,144,3,16,8,6,143,137,1,0,0,0,143,140,
        1,0,0,0,144,147,1,0,0,0,145,143,1,0,0,0,145,146,1,0,0,0,146,17,1,
        0,0,0,147,145,1,0,0,0,148,149,6,9,-1,0,149,150,3,20,10,0,150,156,
        1,0,0,0,151,152,10,1,0,0,152,153,5,22,0,0,153,155,3,20,10,0,154,
        151,1,0,0,0,155,158,1,0,0,0,156,154,1,0,0,0,156,157,1,0,0,0,157,
        19,1,0,0,0,158,156,1,0,0,0,159,160,5,1,0,0,160,161,5,23,0,0,161,
        21,1,0,0,0,162,163,6,11,-1,0,163,164,3,24,12,0,164,170,1,0,0,0,165,
        166,10,1,0,0,166,167,5,22,0,0,167,169,3,24,12,0,168,165,1,0,0,0,
        169,172,1,0,0,0,170,168,1,0,0,0,170,171,1,0,0,0,171,23,1,0,0,0,172,
        170,1,0,0,0,173,174,3,16,8,0,174,25,1,0,0,0,13,29,35,44,70,78,83,
        118,131,135,143,145,156,170
    ]

class SoLangParser ( Parser ):

    grammarFileName = "SoLang.g4"

    atn = ATNDeserializer().deserialize(serializedATN())

    decisionsToDFA = [ DFA(ds, i) for i, ds in enumerate(atn.decisionToState) ]

    sharedContextCache = PredictionContextCache()

    literalNames = [ "<INVALID>", "'int'", "'('", "')'", "'{'", "'}'", "';'", 
                     "'='", "'write'", "'return'", "'if'", "'else'", "'=='", 
                     "'!='", "'<='", "'<'", "'>='", "'>'", "'+'", "'-'", 
                     "'*'", "'/'", "','" ]

    symbolicNames = [ "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                      "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                      "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                      "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                      "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                      "<INVALID>", "<INVALID>", "<INVALID>", "Ident", "Number", 
                      "Newline", "Whitespace", "BlockComment", "LineComment" ]

    RULE_compilationUnit = 0
    RULE_function = 1
    RULE_block = 2
    RULE_stmt = 3
    RULE_if_stmt = 4
    RULE_else_block = 5
    RULE_elseif_block = 6
    RULE_cond = 7
    RULE_expr = 8
    RULE_paramdefs = 9
    RULE_paramdef = 10
    RULE_params = 11
    RULE_param = 12

    ruleNames =  [ "compilationUnit", "function", "block", "stmt", "if_stmt", 
                   "else_block", "elseif_block", "cond", "expr", "paramdefs", 
                   "paramdef", "params", "param" ]

    EOF = Token.EOF
    T__0=1
    T__1=2
    T__2=3
    T__3=4
    T__4=5
    T__5=6
    T__6=7
    T__7=8
    T__8=9
    T__9=10
    T__10=11
    T__11=12
    T__12=13
    T__13=14
    T__14=15
    T__15=16
    T__16=17
    T__17=18
    T__18=19
    T__19=20
    T__20=21
    T__21=22
    Ident=23
    Number=24
    Newline=25
    Whitespace=26
    BlockComment=27
    LineComment=28

    def __init__(self, input:TokenStream, output:TextIO = sys.stdout):
        super().__init__(input, output)
        self.checkVersion("4.13.1")
        self._interp = ParserATNSimulator(self, self.atn, self.decisionsToDFA, self.sharedContextCache)
        self._predicates = None




    class CompilationUnitContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def function(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(SoLangParser.FunctionContext)
            else:
                return self.getTypedRuleContext(SoLangParser.FunctionContext,i)


        def getRuleIndex(self):
            return SoLangParser.RULE_compilationUnit

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitCompilationUnit" ):
                return visitor.visitCompilationUnit(self)
            else:
                return visitor.visitChildren(self)




    def compilationUnit(self):

        localctx = SoLangParser.CompilationUnitContext(self, self._ctx, self.state)
        self.enterRule(localctx, 0, self.RULE_compilationUnit)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 27 
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            while True:
                self.state = 26
                self.function()
                self.state = 29 
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                if not (_la==1):
                    break

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class FunctionContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def Ident(self):
            return self.getToken(SoLangParser.Ident, 0)

        def block(self):
            return self.getTypedRuleContext(SoLangParser.BlockContext,0)


        def paramdefs(self):
            return self.getTypedRuleContext(SoLangParser.ParamdefsContext,0)


        def getRuleIndex(self):
            return SoLangParser.RULE_function

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitFunction" ):
                return visitor.visitFunction(self)
            else:
                return visitor.visitChildren(self)




    def function(self):

        localctx = SoLangParser.FunctionContext(self, self._ctx, self.state)
        self.enterRule(localctx, 2, self.RULE_function)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 31
            self.match(SoLangParser.T__0)
            self.state = 32
            self.match(SoLangParser.Ident)
            self.state = 33
            self.match(SoLangParser.T__1)
            self.state = 35
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            if _la==1:
                self.state = 34
                self.paramdefs(0)


            self.state = 37
            self.match(SoLangParser.T__2)
            self.state = 38
            self.block()
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class BlockContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def stmt(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(SoLangParser.StmtContext)
            else:
                return self.getTypedRuleContext(SoLangParser.StmtContext,i)


        def getRuleIndex(self):
            return SoLangParser.RULE_block

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitBlock" ):
                return visitor.visitBlock(self)
            else:
                return visitor.visitChildren(self)




    def block(self):

        localctx = SoLangParser.BlockContext(self, self._ctx, self.state)
        self.enterRule(localctx, 4, self.RULE_block)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 40
            self.match(SoLangParser.T__3)
            self.state = 42 
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            while True:
                self.state = 41
                self.stmt()
                self.state = 44 
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                if not ((((_la) & ~0x3f) == 0 and ((1 << _la) & 25954054) != 0)):
                    break

            self.state = 46
            self.match(SoLangParser.T__4)
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class StmtContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser


        def getRuleIndex(self):
            return SoLangParser.RULE_stmt

     
        def copyFrom(self, ctx:ParserRuleContext):
            super().copyFrom(ctx)



    class ExprStmtContext(StmtContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SoLangParser.StmtContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def expr(self):
            return self.getTypedRuleContext(SoLangParser.ExprContext,0)


        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitExprStmt" ):
                return visitor.visitExprStmt(self)
            else:
                return visitor.visitChildren(self)


    class IfStmtContext(StmtContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SoLangParser.StmtContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def if_stmt(self):
            return self.getTypedRuleContext(SoLangParser.If_stmtContext,0)


        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitIfStmt" ):
                return visitor.visitIfStmt(self)
            else:
                return visitor.visitChildren(self)


    class AsgnStmtContext(StmtContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SoLangParser.StmtContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def Ident(self):
            return self.getToken(SoLangParser.Ident, 0)
        def expr(self):
            return self.getTypedRuleContext(SoLangParser.ExprContext,0)


        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitAsgnStmt" ):
                return visitor.visitAsgnStmt(self)
            else:
                return visitor.visitChildren(self)


    class WriteStmtContext(StmtContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SoLangParser.StmtContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def expr(self):
            return self.getTypedRuleContext(SoLangParser.ExprContext,0)


        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitWriteStmt" ):
                return visitor.visitWriteStmt(self)
            else:
                return visitor.visitChildren(self)


    class ReturnStmtContext(StmtContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SoLangParser.StmtContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def expr(self):
            return self.getTypedRuleContext(SoLangParser.ExprContext,0)


        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitReturnStmt" ):
                return visitor.visitReturnStmt(self)
            else:
                return visitor.visitChildren(self)


    class VariableDefinitionStmtContext(StmtContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SoLangParser.StmtContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def Ident(self):
            return self.getToken(SoLangParser.Ident, 0)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitVariableDefinitionStmt" ):
                return visitor.visitVariableDefinitionStmt(self)
            else:
                return visitor.visitChildren(self)



    def stmt(self):

        localctx = SoLangParser.StmtContext(self, self._ctx, self.state)
        self.enterRule(localctx, 6, self.RULE_stmt)
        try:
            self.state = 70
            self._errHandler.sync(self)
            la_ = self._interp.adaptivePredict(self._input,3,self._ctx)
            if la_ == 1:
                localctx = SoLangParser.ExprStmtContext(self, localctx)
                self.enterOuterAlt(localctx, 1)
                self.state = 48
                self.expr(0)
                self.state = 49
                self.match(SoLangParser.T__5)
                pass

            elif la_ == 2:
                localctx = SoLangParser.VariableDefinitionStmtContext(self, localctx)
                self.enterOuterAlt(localctx, 2)
                self.state = 51
                self.match(SoLangParser.T__0)
                self.state = 52
                self.match(SoLangParser.Ident)
                self.state = 53
                self.match(SoLangParser.T__5)
                pass

            elif la_ == 3:
                localctx = SoLangParser.AsgnStmtContext(self, localctx)
                self.enterOuterAlt(localctx, 3)
                self.state = 54
                self.match(SoLangParser.Ident)
                self.state = 55
                self.match(SoLangParser.T__6)
                self.state = 56
                self.expr(0)
                self.state = 57
                self.match(SoLangParser.T__5)
                pass

            elif la_ == 4:
                localctx = SoLangParser.IfStmtContext(self, localctx)
                self.enterOuterAlt(localctx, 4)
                self.state = 59
                self.if_stmt()
                pass

            elif la_ == 5:
                localctx = SoLangParser.WriteStmtContext(self, localctx)
                self.enterOuterAlt(localctx, 5)
                self.state = 60
                self.match(SoLangParser.T__7)
                self.state = 61
                self.match(SoLangParser.T__1)
                self.state = 62
                self.expr(0)
                self.state = 63
                self.match(SoLangParser.T__2)
                self.state = 64
                self.match(SoLangParser.T__5)
                pass

            elif la_ == 6:
                localctx = SoLangParser.ReturnStmtContext(self, localctx)
                self.enterOuterAlt(localctx, 6)
                self.state = 66
                self.match(SoLangParser.T__8)
                self.state = 67
                self.expr(0)
                self.state = 68
                self.match(SoLangParser.T__5)
                pass


        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class If_stmtContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def cond(self):
            return self.getTypedRuleContext(SoLangParser.CondContext,0)


        def block(self):
            return self.getTypedRuleContext(SoLangParser.BlockContext,0)


        def else_block(self):
            return self.getTypedRuleContext(SoLangParser.Else_blockContext,0)


        def getRuleIndex(self):
            return SoLangParser.RULE_if_stmt

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitIf_stmt" ):
                return visitor.visitIf_stmt(self)
            else:
                return visitor.visitChildren(self)




    def if_stmt(self):

        localctx = SoLangParser.If_stmtContext(self, self._ctx, self.state)
        self.enterRule(localctx, 8, self.RULE_if_stmt)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 72
            self.match(SoLangParser.T__9)
            self.state = 73
            self.match(SoLangParser.T__1)
            self.state = 74
            self.cond()
            self.state = 75
            self.match(SoLangParser.T__2)
            self.state = 76
            self.block()
            self.state = 78
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            if _la==11:
                self.state = 77
                self.else_block()


        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class Else_blockContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def block(self):
            return self.getTypedRuleContext(SoLangParser.BlockContext,0)


        def elseif_block(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(SoLangParser.Elseif_blockContext)
            else:
                return self.getTypedRuleContext(SoLangParser.Elseif_blockContext,i)


        def getRuleIndex(self):
            return SoLangParser.RULE_else_block

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitElse_block" ):
                return visitor.visitElse_block(self)
            else:
                return visitor.visitChildren(self)




    def else_block(self):

        localctx = SoLangParser.Else_blockContext(self, self._ctx, self.state)
        self.enterRule(localctx, 10, self.RULE_else_block)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 83
            self._errHandler.sync(self)
            _alt = self._interp.adaptivePredict(self._input,5,self._ctx)
            while _alt!=2 and _alt!=ATN.INVALID_ALT_NUMBER:
                if _alt==1:
                    self.state = 80
                    self.elseif_block() 
                self.state = 85
                self._errHandler.sync(self)
                _alt = self._interp.adaptivePredict(self._input,5,self._ctx)

            self.state = 86
            self.match(SoLangParser.T__10)
            self.state = 87
            self.block()
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class Elseif_blockContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def block(self):
            return self.getTypedRuleContext(SoLangParser.BlockContext,0)


        def getRuleIndex(self):
            return SoLangParser.RULE_elseif_block

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitElseif_block" ):
                return visitor.visitElseif_block(self)
            else:
                return visitor.visitChildren(self)




    def elseif_block(self):

        localctx = SoLangParser.Elseif_blockContext(self, self._ctx, self.state)
        self.enterRule(localctx, 12, self.RULE_elseif_block)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 89
            self.match(SoLangParser.T__10)
            self.state = 90
            self.match(SoLangParser.T__9)
            self.state = 91
            self.block()
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class CondContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def expr(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(SoLangParser.ExprContext)
            else:
                return self.getTypedRuleContext(SoLangParser.ExprContext,i)


        def getRuleIndex(self):
            return SoLangParser.RULE_cond

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitCond" ):
                return visitor.visitCond(self)
            else:
                return visitor.visitChildren(self)




    def cond(self):

        localctx = SoLangParser.CondContext(self, self._ctx, self.state)
        self.enterRule(localctx, 14, self.RULE_cond)
        try:
            self.state = 118
            self._errHandler.sync(self)
            la_ = self._interp.adaptivePredict(self._input,6,self._ctx)
            if la_ == 1:
                self.enterOuterAlt(localctx, 1)
                self.state = 93
                self.expr(0)
                self.state = 94
                self.match(SoLangParser.T__11)
                self.state = 95
                self.expr(0)
                pass

            elif la_ == 2:
                self.enterOuterAlt(localctx, 2)
                self.state = 97
                self.expr(0)
                self.state = 98
                self.match(SoLangParser.T__12)
                self.state = 99
                self.expr(0)
                pass

            elif la_ == 3:
                self.enterOuterAlt(localctx, 3)
                self.state = 101
                self.expr(0)
                self.state = 102
                self.match(SoLangParser.T__13)
                self.state = 103
                self.expr(0)
                pass

            elif la_ == 4:
                self.enterOuterAlt(localctx, 4)
                self.state = 105
                self.expr(0)
                self.state = 106
                self.match(SoLangParser.T__14)
                self.state = 107
                self.expr(0)
                pass

            elif la_ == 5:
                self.enterOuterAlt(localctx, 5)
                self.state = 109
                self.expr(0)
                self.state = 110
                self.match(SoLangParser.T__15)
                self.state = 111
                self.expr(0)
                pass

            elif la_ == 6:
                self.enterOuterAlt(localctx, 6)
                self.state = 113
                self.expr(0)
                self.state = 114
                self.match(SoLangParser.T__16)
                self.state = 115
                self.expr(0)
                pass

            elif la_ == 7:
                self.enterOuterAlt(localctx, 7)
                self.state = 117
                self.expr(0)
                pass


        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class ExprContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser


        def getRuleIndex(self):
            return SoLangParser.RULE_expr

     
        def copyFrom(self, ctx:ParserRuleContext):
            super().copyFrom(ctx)


    class IdentExprContext(ExprContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SoLangParser.ExprContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def Ident(self):
            return self.getToken(SoLangParser.Ident, 0)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitIdentExpr" ):
                return visitor.visitIdentExpr(self)
            else:
                return visitor.visitChildren(self)


    class ParExprContext(ExprContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SoLangParser.ExprContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def expr(self):
            return self.getTypedRuleContext(SoLangParser.ExprContext,0)


        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitParExpr" ):
                return visitor.visitParExpr(self)
            else:
                return visitor.visitChildren(self)


    class UnaryExprContext(ExprContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SoLangParser.ExprContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def expr(self):
            return self.getTypedRuleContext(SoLangParser.ExprContext,0)


        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitUnaryExpr" ):
                return visitor.visitUnaryExpr(self)
            else:
                return visitor.visitChildren(self)


    class AddSubExprContext(ExprContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SoLangParser.ExprContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def expr(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(SoLangParser.ExprContext)
            else:
                return self.getTypedRuleContext(SoLangParser.ExprContext,i)


        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitAddSubExpr" ):
                return visitor.visitAddSubExpr(self)
            else:
                return visitor.visitChildren(self)


    class NumberExprContext(ExprContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SoLangParser.ExprContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def Number(self):
            return self.getToken(SoLangParser.Number, 0)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitNumberExpr" ):
                return visitor.visitNumberExpr(self)
            else:
                return visitor.visitChildren(self)


    class FunctionCallExprContext(ExprContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SoLangParser.ExprContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def Ident(self):
            return self.getToken(SoLangParser.Ident, 0)
        def params(self):
            return self.getTypedRuleContext(SoLangParser.ParamsContext,0)


        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitFunctionCallExpr" ):
                return visitor.visitFunctionCallExpr(self)
            else:
                return visitor.visitChildren(self)


    class MulDivExprContext(ExprContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SoLangParser.ExprContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def expr(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(SoLangParser.ExprContext)
            else:
                return self.getTypedRuleContext(SoLangParser.ExprContext,i)


        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitMulDivExpr" ):
                return visitor.visitMulDivExpr(self)
            else:
                return visitor.visitChildren(self)



    def expr(self, _p:int=0):
        _parentctx = self._ctx
        _parentState = self.state
        localctx = SoLangParser.ExprContext(self, self._ctx, _parentState)
        _prevctx = localctx
        _startState = 16
        self.enterRecursionRule(localctx, 16, self.RULE_expr, _p)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 135
            self._errHandler.sync(self)
            la_ = self._interp.adaptivePredict(self._input,8,self._ctx)
            if la_ == 1:
                localctx = SoLangParser.UnaryExprContext(self, localctx)
                self._ctx = localctx
                _prevctx = localctx

                self.state = 121
                _la = self._input.LA(1)
                if not(_la==18 or _la==19):
                    self._errHandler.recoverInline(self)
                else:
                    self._errHandler.reportMatch(self)
                    self.consume()
                self.state = 122
                self.expr(7)
                pass

            elif la_ == 2:
                localctx = SoLangParser.ParExprContext(self, localctx)
                self._ctx = localctx
                _prevctx = localctx
                self.state = 123
                self.match(SoLangParser.T__1)
                self.state = 124
                self.expr(0)
                self.state = 125
                self.match(SoLangParser.T__2)
                pass

            elif la_ == 3:
                localctx = SoLangParser.NumberExprContext(self, localctx)
                self._ctx = localctx
                _prevctx = localctx
                self.state = 127
                self.match(SoLangParser.Number)
                pass

            elif la_ == 4:
                localctx = SoLangParser.FunctionCallExprContext(self, localctx)
                self._ctx = localctx
                _prevctx = localctx
                self.state = 128
                self.match(SoLangParser.Ident)
                self.state = 129
                self.match(SoLangParser.T__1)
                self.state = 131
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                if (((_la) & ~0x3f) == 0 and ((1 << _la) & 25952260) != 0):
                    self.state = 130
                    self.params(0)


                self.state = 133
                self.match(SoLangParser.T__2)
                pass

            elif la_ == 5:
                localctx = SoLangParser.IdentExprContext(self, localctx)
                self._ctx = localctx
                _prevctx = localctx
                self.state = 134
                self.match(SoLangParser.Ident)
                pass


            self._ctx.stop = self._input.LT(-1)
            self.state = 145
            self._errHandler.sync(self)
            _alt = self._interp.adaptivePredict(self._input,10,self._ctx)
            while _alt!=2 and _alt!=ATN.INVALID_ALT_NUMBER:
                if _alt==1:
                    if self._parseListeners is not None:
                        self.triggerExitRuleEvent()
                    _prevctx = localctx
                    self.state = 143
                    self._errHandler.sync(self)
                    la_ = self._interp.adaptivePredict(self._input,9,self._ctx)
                    if la_ == 1:
                        localctx = SoLangParser.MulDivExprContext(self, SoLangParser.ExprContext(self, _parentctx, _parentState))
                        self.pushNewRecursionContext(localctx, _startState, self.RULE_expr)
                        self.state = 137
                        if not self.precpred(self._ctx, 6):
                            from antlr4.error.Errors import FailedPredicateException
                            raise FailedPredicateException(self, "self.precpred(self._ctx, 6)")
                        self.state = 138
                        _la = self._input.LA(1)
                        if not(_la==20 or _la==21):
                            self._errHandler.recoverInline(self)
                        else:
                            self._errHandler.reportMatch(self)
                            self.consume()
                        self.state = 139
                        self.expr(7)
                        pass

                    elif la_ == 2:
                        localctx = SoLangParser.AddSubExprContext(self, SoLangParser.ExprContext(self, _parentctx, _parentState))
                        self.pushNewRecursionContext(localctx, _startState, self.RULE_expr)
                        self.state = 140
                        if not self.precpred(self._ctx, 5):
                            from antlr4.error.Errors import FailedPredicateException
                            raise FailedPredicateException(self, "self.precpred(self._ctx, 5)")
                        self.state = 141
                        _la = self._input.LA(1)
                        if not(_la==18 or _la==19):
                            self._errHandler.recoverInline(self)
                        else:
                            self._errHandler.reportMatch(self)
                            self.consume()
                        self.state = 142
                        self.expr(6)
                        pass

             
                self.state = 147
                self._errHandler.sync(self)
                _alt = self._interp.adaptivePredict(self._input,10,self._ctx)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.unrollRecursionContexts(_parentctx)
        return localctx


    class ParamdefsContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def paramdef(self):
            return self.getTypedRuleContext(SoLangParser.ParamdefContext,0)


        def paramdefs(self):
            return self.getTypedRuleContext(SoLangParser.ParamdefsContext,0)


        def getRuleIndex(self):
            return SoLangParser.RULE_paramdefs

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitParamdefs" ):
                return visitor.visitParamdefs(self)
            else:
                return visitor.visitChildren(self)



    def paramdefs(self, _p:int=0):
        _parentctx = self._ctx
        _parentState = self.state
        localctx = SoLangParser.ParamdefsContext(self, self._ctx, _parentState)
        _prevctx = localctx
        _startState = 18
        self.enterRecursionRule(localctx, 18, self.RULE_paramdefs, _p)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 149
            self.paramdef()
            self._ctx.stop = self._input.LT(-1)
            self.state = 156
            self._errHandler.sync(self)
            _alt = self._interp.adaptivePredict(self._input,11,self._ctx)
            while _alt!=2 and _alt!=ATN.INVALID_ALT_NUMBER:
                if _alt==1:
                    if self._parseListeners is not None:
                        self.triggerExitRuleEvent()
                    _prevctx = localctx
                    localctx = SoLangParser.ParamdefsContext(self, _parentctx, _parentState)
                    self.pushNewRecursionContext(localctx, _startState, self.RULE_paramdefs)
                    self.state = 151
                    if not self.precpred(self._ctx, 1):
                        from antlr4.error.Errors import FailedPredicateException
                        raise FailedPredicateException(self, "self.precpred(self._ctx, 1)")
                    self.state = 152
                    self.match(SoLangParser.T__21)
                    self.state = 153
                    self.paramdef() 
                self.state = 158
                self._errHandler.sync(self)
                _alt = self._interp.adaptivePredict(self._input,11,self._ctx)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.unrollRecursionContexts(_parentctx)
        return localctx


    class ParamdefContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def Ident(self):
            return self.getToken(SoLangParser.Ident, 0)

        def getRuleIndex(self):
            return SoLangParser.RULE_paramdef

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitParamdef" ):
                return visitor.visitParamdef(self)
            else:
                return visitor.visitChildren(self)




    def paramdef(self):

        localctx = SoLangParser.ParamdefContext(self, self._ctx, self.state)
        self.enterRule(localctx, 20, self.RULE_paramdef)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 159
            self.match(SoLangParser.T__0)
            self.state = 160
            self.match(SoLangParser.Ident)
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class ParamsContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def param(self):
            return self.getTypedRuleContext(SoLangParser.ParamContext,0)


        def params(self):
            return self.getTypedRuleContext(SoLangParser.ParamsContext,0)


        def getRuleIndex(self):
            return SoLangParser.RULE_params

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitParams" ):
                return visitor.visitParams(self)
            else:
                return visitor.visitChildren(self)



    def params(self, _p:int=0):
        _parentctx = self._ctx
        _parentState = self.state
        localctx = SoLangParser.ParamsContext(self, self._ctx, _parentState)
        _prevctx = localctx
        _startState = 22
        self.enterRecursionRule(localctx, 22, self.RULE_params, _p)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 163
            self.param()
            self._ctx.stop = self._input.LT(-1)
            self.state = 170
            self._errHandler.sync(self)
            _alt = self._interp.adaptivePredict(self._input,12,self._ctx)
            while _alt!=2 and _alt!=ATN.INVALID_ALT_NUMBER:
                if _alt==1:
                    if self._parseListeners is not None:
                        self.triggerExitRuleEvent()
                    _prevctx = localctx
                    localctx = SoLangParser.ParamsContext(self, _parentctx, _parentState)
                    self.pushNewRecursionContext(localctx, _startState, self.RULE_params)
                    self.state = 165
                    if not self.precpred(self._ctx, 1):
                        from antlr4.error.Errors import FailedPredicateException
                        raise FailedPredicateException(self, "self.precpred(self._ctx, 1)")
                    self.state = 166
                    self.match(SoLangParser.T__21)
                    self.state = 167
                    self.param() 
                self.state = 172
                self._errHandler.sync(self)
                _alt = self._interp.adaptivePredict(self._input,12,self._ctx)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.unrollRecursionContexts(_parentctx)
        return localctx


    class ParamContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def expr(self):
            return self.getTypedRuleContext(SoLangParser.ExprContext,0)


        def getRuleIndex(self):
            return SoLangParser.RULE_param

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitParam" ):
                return visitor.visitParam(self)
            else:
                return visitor.visitChildren(self)




    def param(self):

        localctx = SoLangParser.ParamContext(self, self._ctx, self.state)
        self.enterRule(localctx, 24, self.RULE_param)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 173
            self.expr(0)
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx



    def sempred(self, localctx:RuleContext, ruleIndex:int, predIndex:int):
        if self._predicates == None:
            self._predicates = dict()
        self._predicates[8] = self.expr_sempred
        self._predicates[9] = self.paramdefs_sempred
        self._predicates[11] = self.params_sempred
        pred = self._predicates.get(ruleIndex, None)
        if pred is None:
            raise Exception("No predicate with index:" + str(ruleIndex))
        else:
            return pred(localctx, predIndex)

    def expr_sempred(self, localctx:ExprContext, predIndex:int):
            if predIndex == 0:
                return self.precpred(self._ctx, 6)
         

            if predIndex == 1:
                return self.precpred(self._ctx, 5)
         

    def paramdefs_sempred(self, localctx:ParamdefsContext, predIndex:int):
            if predIndex == 2:
                return self.precpred(self._ctx, 1)
         

    def params_sempred(self, localctx:ParamsContext, predIndex:int):
            if predIndex == 3:
                return self.precpred(self._ctx, 1)
         




