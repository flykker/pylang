ANTLR4=java -jar /usr/local/lib/antlr-4.8-complete.jar

GRAMMAR_PY=build/grammar/SoLangLexer.py \
build/grammar/SoLang.interp \
build/grammar/SoLang.tokens \
build/grammar/SoLangLexer.interp \
build/grammar/SoLangLexer.tokens \
build/grammar/SoLangParser.py

all: $(GRAMMAR_PY) build/builtin.ll

run: all
	python main.py

clean:
	[ -d build ] && rm -rf build/*
	[ -d __pycache__ ] && rm -rf __pycache__

tmp: all
	echo "* running test 1"
	echo "int add(){return 2*3;} int main(){write(1); write(add()); return 0;}" | python main.py
	llvm-link build/out.ll build/builtin.ll -S -o build/linked.ll
	echo "* running linked.ll by lli (inetrpreter)"
	lli build/linked.ll

test: all
	echo "* running test 1"
	echo "int main(){write(-3+1*2);return 0;}" | python main.py
	llvm-link build/out.ll build/builtin.ll -S -o build/linked.ll
	echo "* running linked.ll by lli (inetrpreter)"
	lli build/linked.ll
	echo "* running test 2"
	echo "int main(){write(1+2*(3+4));return 0;}" | python main.py
	llvm-link build/out.ll build/builtin.ll -S -o build/linked.ll
	llc build/linked.ll -o build/linked.s
	clang build/linked.s -o build/linked
	echo "* running native linked"
	build/linked
	lli build/linked.ll
	echo "* running test 3"
	echo "int main(){write(3*(3+5)/4);write(1);return 0;}" | python main.py
	llvm-link build/out.ll build/builtin.ll -S -o build/linked.ll
	echo "* running linked.ll by lli (inetrpreter)"
	lli build/linked.ll
	echo "* running test 4"
	echo "int add(){return 2*3;} int main(){write(1); write(add()); return 0;}" | python main.py
	llvm-link build/out.ll build/builtin.ll -S -o build/linked.ll
	echo "* running linked.ll by lli (inetrpreter)"
	lli build/linked.ll

# generation rules
$(GRAMMAR_PY): grammar/SoLang.g4
	[ -d build ] || mkdir build
	$(ANTLR4) grammar/SoLang.g4 -no-listener -visitor -o build

build/builtin.ll: builtin.c
	[ -d build ] || mkdir build
	clang -emit-llvm -S -O -o build/builtin.ll builtin.c

