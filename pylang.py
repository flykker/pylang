#!/usr/bin/env python3
import grammar
import antlr4
import sys
import argparse


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description='SoLang compiler')
    parser.add_argument('-O', dest='optimize', action='store_true',
                        default=False,
                        help='Optimize genrated IR')
    parser.add_argument('-C', dest='compile', action='store_true',
                        default=False,
                        help='Compile IR')
    return parser.parse_args()


def tokenize(source: str) -> antlr4.CommonTokenStream:
    lexer = grammar.PythonLexer(antlr4.InputStream(source))
    return antlr4.CommonTokenStream(lexer)


def visit(
        args: argparse.Namespace,
        token_stream: antlr4.CommonTokenStream) -> None:
    parser = grammar.PythonParser(token_stream)
    
    visitor = grammar.PyLangVisitor(args, parser=parser)

    # visit AST nodes
    visitor.visitFile_input(parser.file_input())


def main():
    args = parse_args()

    try:
        file = open("tests/main.py","r")
        source =  file.read()
    except:
        source = ''.join(sys.stdin.readlines())
    token_stream = tokenize(source)

    # from antlr4.tree.Trees import Trees
    # parser = grammar.PythonParser(token_stream)
    
    # tree = parser.file_input()
    # import pprint
    # pprint.pprint(Trees.toStringTree(tree, None, parser))

    visit(args, token_stream)

if __name__ == '__main__':
    main()
