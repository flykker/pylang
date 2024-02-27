import lib

class sockaddr:
    sa_family: i16
    sa_data: array

class sockaddr_in:
    sin_family: u16
    sin_port: u16
    sin_addr: u32
    sin_zero: u64

@C
def i16(integer: int) -> i16:
    pass

@C
def i64(integer: int) -> i64:
    pass

@C
def inet_addr(ip: ptr[str]) -> int:
    pass

@C
def socket(domain: int, type: int, proto: int = 0) -> int:
    pass

@C
def bind(fd: int, addr: ptr[sockaddr], len: int) -> int:
    pass

@C
def accept(fd: int, addr: ptr[sockaddr], len: ptr[int]) -> int:
    pass

@C
def listen(fd: int, n: int) -> int:
    pass

@C
def htons(port: u16) -> u16:
    pass

@C
def sleep(seconds: int) -> int:
    pass

@C
def close(fd: int) -> int:
    pass
@C
def recv(fd: int, buf: ptr[str], len: i64, flags: int) -> int:
    pass

@C
def send(fd: int, buf: ptr[str], len: i64, flags: int) -> int:
    pass

@C
def exit(code: int) -> void:
    pass

@C
def bytearray(count: int) -> ptr[str]:
    pass

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

class Socket:
    sock: int

    def create(self):
        AF_INET = 2
        SOCK_STREAM = 1

        sock = socket(2, 1, 0)
        print(sock)
        print()

        if sock == -1:
            print("Error create socket")
            print()
            exit(0)
        
        sock_addr = sockaddr_in()
        sock_addr.sin_family = i16(2)
        sock_addr.sin_port = htons(i16(8000))
        sock_addr.sin_addr = inet_addr('127.0.0.1')
        sock_addr.sin_zero = i64(0)
        
        len = sizeof(sock_addr)

        b = bind(sock, sock_addr, len)
        if b < 0:
            print("Error bind socket")
            print()
            exit(0)
        
        print('Bind socket success')
        print()
        
        listen(sock, 10)
        
        flags = 0
        loop = 1
        buf_size = 1024
        output = bytearray(buf_size)
        #server = "HTTP/1.1 200 OK\nServer: Z-Server\n"
        server = ""
        
        # while loop == 1:
        newsockfd = accept(sock, sock_addr, len)
        recv_data = 1         

        # if newsockfd < 0:
        #     print("Error webserver accept")
        #     exit(0)
        
        print('Accept connect\n')

        recv_msg_size = recv(newsockfd, output, i64(buf_size), flags)
        #print(output)
        
        
        # send(newsockfd, server, 33, flags)
        send(newsockfd, server, i64(33), flags)
        close(newsockfd)
        close(sock)
        return 0

# comments
def main():
    AF_INET = 2
    SOCK_STREAM = 1

    sock = socket(2, 1, 0)
    print(sock)
    print()
    
    sock_addr = sockaddr_in()
    sock_addr.sin_family = i16(2)
    sock_addr.sin_port = htons(i16(8000))
    sock_addr.sin_addr = inet_addr('127.0.0.1')
    sock_addr.sin_zero = i64(0)
    
    len = sizeof(sock_addr)

    b = bind(sock, sock_addr, len)
    
    print('Bind socket success')
    print()
    
    listen(sock, 10)
    
    flags = 0
    loop = 1
    buf_size = 1024
    output = bytearray(buf_size)
    server = "HTTP/1.1 200 OK\nServer: Z-Server\nContent-type: text/html\n\n<html><body>Z-Server</body></html>"
    
    while loop == 1:
        newsockfd = accept(sock, sock_addr, len)      

        print('Accept connect\n')

        recv_msg_size = recv(newsockfd, output, i64(buf_size), flags)
        print(output)

        send(newsockfd, server, i64(96), flags)
        close(newsockfd)

    close(sock)
    return 0