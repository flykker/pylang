def fib(n):
    if n < 2:
        return n
    else:
        return fib(n-1) + fib(n-2)
    
    return 0

# def pytest(n):
#     b = [111,222,333,444,555]
#     c = b[0]
#     b[7] = 777
#     print(b[7])
#     print()
#     for ind in b:
#         if ind == 555:
#             print(ind)
#     print()
#     print(333)
#     return 0

#2 Comments
def main():
    a = 40
    f = fib(a)
    print("Fib test: ")
    print(f)
    print()
    
    s = 'Test !'
    #b = "Test2 !"
    print(s)

    return 0
