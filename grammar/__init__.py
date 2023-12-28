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

try:
    from SoLangLexer import SoLangLexer
    from SoLangParser import SoLangParser
    from SoLangVisitor import SoLangVisitor
    from .my_visitor import MyVisitor
except:
    pass

from PythonLexer import PythonLexer
from PythonParser import PythonParser
from PythonParserVisitor import PythonParserVisitor
from .pylang_visitor import PyLangVisitor

