import os
import sys

module_path = os.path.join(
    os.path.dirname(
        os.path.dirname(
            os.path.abspath(__file__))),
    'build',
    'grammar')
if module_path not in sys.path:
    sys.path.append(module_path)

from PythonLexer import PythonLexer
from PythonParser import PythonParser
from PythonParserVisitor import PythonParserVisitor
from .pylang_visitor import PyLangVisitor

