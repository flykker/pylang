ANTLR4=java -jar antlr-4.13.1-complete.jar

ANTLR4_PY=build/grammar/PyLangLexer.py \
build/grammar/PyLang.interp \
build/grammar/PyLang.tokens \
build/grammar/PyLangLexer.interp \
build/grammar/PyLangLexer.tokens \
build/grammar/PyLangParser.py

pylang: genrules lib

clean:
	[ -d build ] && rm -rf build/*
	[ -d __pycache__ ] && rm -rf __pycache__

run-jit:
	echo "Running test pylang"
	python3.10 ./main.py -O
#	llvm-link build/out.ll build/builtin.ll -S -o build/linked.ll
	lli --jit-kind=mcjit build/linked.ll

run:
	echo "Running test pylang"
	python3.10 ./main.py -O -C
#	llvm-link build/out.ll build/builtin.ll -S -o build/linked.ll
#	llc build/linked.ll -o build/linked.s
	llvm-as build/linked.ll -o build/linked.bc
	clang build/linked.bc -o build/app
	echo "Run binary python ..."
	./build/app

genrules: grammar/PyLang.g4
	[ -d build ] || mkdir build
	$(ANTLR4) grammar/PyLang.g4 -no-listener -visitor -o build

lib: builtin.c
	[ -d build ] || mkdir build
	clang -emit-llvm -S -O -o build/builtin.ll builtin.c

