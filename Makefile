ANTLR4=java -jar antlr-4.13.1-complete.jar

ANTLR4_PY=build/grammar/PyLangLexer.py \
build/grammar/PyLang.interp \
build/grammar/PyLang.tokens \
build/grammar/PyLangLexer.interp \
build/grammar/PyLangLexer.tokens \
build/grammar/PyLangParser.py

pylang: genrules build/builtin.ll

clean:
	[ -d build ] && rm -rf build/*
	[ -d __pycache__ ] && rm -rf __pycache__

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

genrules: grammar/PyLang.g4
	[ -d build ] || mkdir build
	$(ANTLR4) grammar/PyLang.g4 -no-listener -visitor -o build

build/builtin.ll: builtin.c
	[ -d build ] || mkdir build
	clang-12 -emit-llvm -S -O -o build/builtin.ll builtin.c

