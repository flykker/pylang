from numba import njit

@njit
def test3(i):
    a = 1+1
    print(a)

test3(10)

# Get the llvm IR
llvm = test3.inspect_llvm()
with open('numba.ll','w') as f:
    f.write(next(iter(llvm.values())))

# Get the assembly code
#print(test3.inspect_asm())

# from numba.pycc import CC

# cc = CC('my_module')
# # Uncomment the following line to print out the compilation steps
# cc.verbose = True

# @cc.export('square', 'f8(f8)')
# def square(a):
#     return a ** 2


#cc.compile()