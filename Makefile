ANTLR4=java -jar antlr-4.13.1-complete.jar


GRAMMAR_PY=build/grammar/SoLangLexer.py \
build/grammar/SoLang.interp \
build/grammar/SoLang.tokens \
build/grammar/SoLangLexer.interp \
build/grammar/SoLangLexer.tokens \
build/grammar/SoLangParser.py

ANTLR4_PY=build/grammar/PyLangLexer.py \
build/grammar/PyLang.interp \
build/grammar/PyLang.tokens \
build/grammar/PyLangLexer.interp \
build/grammar/PyLangLexer.tokens \
build/grammar/PyLangParser.py

all: $(GRAMMAR_PY) build/builtin.ll

pylang: genrules build/builtin.ll

run: all
	./main.py

clean:
	[ -d build ] && rm -rf build/*
	[ -d __pycache__ ] && rm -rf __pycache__

tmp: all
	echo "* running tmp test"
	echo "int main(){int x;x=1;write(0);if (x==2) {write(10);} else {write(20);} write(1); write (2);return 0;}" | ./main.py
	llvm-link-12 build/out.ll build/builtin.ll -S -o build/linked.ll
	echo "* running linked.ll by lli (inetrpreter)"
	lli build/linked.ll

run-pytest:
	echo "Running test pylang"
	cat tests/fib.py | ./main.py -O
	llvm-link-12 build/out.ll build/builtin.ll -S -o build/linked.ll
	lli build/linked.ll

pytest:
	echo "Running test pylang"
	cat tests/fib.py | ./main.py -O
	llvm-link-12 build/out.ll build/builtin.ll -S -o build/linked.ll
#	lli build/linked.ll
	llc-12 build/linked.ll -o build/linked.s
	clang-12 build/linked.s -o build/linked
	echo "Run binary python ..."
	./build/linked

test: 
	echo "* running test 1"
	cat tests/if.solang | ./main.py
	llvm-link-12 build/out.ll build/builtin.ll -S -o build/linked.ll
	lli build/linked.ll
	echo
	echo "* running test 2"
	cat tests/fib.solang | ./main_.py -O
	llvm-link-12 build/out.ll build/builtin.ll -S -o build/linked.ll
	llc-12 build/linked.ll -o build/linked.s
	clang-12 build/linked.s -o build/linked
	echo "Run binary python ..."
	./build/linked

# generation rules
$(GRAMMAR_PY): grammar/SoLang.g4
	[ -d build ] || mkdir build
	$(ANTLR4) grammar/SoLang.g4 -no-listener -visitor -o build

genrules: grammar/PyLang.g4
	[ -d build ] || mkdir build
	$(ANTLR4) grammar/PyLang.g4 -no-listener -visitor -o build

build/builtin.ll: builtin.c
	[ -d build ] || mkdir build
	clang-12 -emit-llvm -S -O -o build/builtin.ll builtin.c

