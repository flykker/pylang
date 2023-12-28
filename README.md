## About

* LLVM frontend (IR generation) Compiler Python language

## How to build to exec

* Run `make pytest` to build/linked

```sh
make pytest
```

## How to run JIT

* Run `make run-pytest` to see what happens

```sh
make run-pytest

```

## Example fibonacci

```python
# Comments
def print(number):
	# C function printf
    write(number)
    return 0

def fib(n):
    if n < 2:
        return n
    else:
        return fib(n-1) + fib(n-2)
    
    return 0

#2 Comments
def main():
    a = 40
	b = [1,2,3,4,5]
    print(fib(a))
    return 0

```



