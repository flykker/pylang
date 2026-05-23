class HttpServer:
    fd: int = 0
    routers: dict[str, Callable] = {}

    def __init__(self, routers: dict[str, Callable]):
        self.fd = 0
        self.routers = routers

    def worker(self, listen_fd: int):
        efd: int = epoll_create1(0)
        ev: epoll_event = epoll_event(0x1, listen_fd)
        epoll_ctl(efd, 1, listen_fd, struct_ptr(ev))

        events: epoll_event = epoll_event(64)

        while 1:
            n: int = epoll_wait(efd, events, 64, -1)
            i: int = 0
            while i < n:
                ev: epoll_event = events[i]
                conn: int = ev.data
                if conn == listen_fd:
                    conn = accept4(listen_fd, 2048)  # SOCK_NONBLOCK
                    if conn > 0:
                        conn_ev: epoll_event = epoll_event(0x1, conn)
                        epoll_ctl(efd, 1, conn, struct_ptr(conn_ev))
                else:
                    buf: ptr = alloc_sys(1032)
                    nread: int = recv_sys(conn, buf, 1024)
                    if nread > 0:
                        content: str = self.routers["/"]()
                        length: int = len(content)
                        response: str = f"HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{content}"
                        send(conn, response)
                    close(conn)
                i = i + 1

    def run(self, host: str, port: int) -> int:
        self.fd = socket(2, 1 | 2048, 0)  # SOCK_STREAM | SOCK_NONBLOCK

        err: int = bind(self.fd, host, port)
        if err < 0:
            print(f"Address {host}:{port} already in use !!!\n")
            exit(1)

        listen(self.fd, 1024)
        print(f"Running on port {port} ...\n")

        i: int = 0
        while i < 4:
            pid: int = fork()
            if pid == 0:
                self.worker(self.fd)
                return 0
            i = i + 1

        while 1:
            waitpid(-1, 0)


class Router:
    routers: dict[str, Callable] = {}
    
    def __init__(self):
        self.routers: dict[str, Callable] = {}

    def add_route(self, path: str, endpoint: Callable):
        self.routers[path] = endpoint
        print(f"Route registered {path} !!!\n")


class FastPy:
    def __init__(self):
        self.router: Router = Router()
        self.app: HttpServer = HttpServer(self.router.routers)

    def run(self, host: str, port: int):
        self.app.run("0.0.0.0", 8080)

    def post(self, path: str) -> Callable:
        def decorator(func: Callable) -> Callable:
            self.router.add_route(path, func)
            return func
        return decorator

    def get(self, path: str) -> Callable:
        def decorator(func: Callable) -> Callable:
            self.router.add_route(path, func)
            return func
        return decorator

app: FastPy = FastPy()

@app.post("/health")
def health() -> str:
    print("Log:Main Health is OK !\n")
    return "{'health':'ok'}"

@app.get("/")
def hello() -> str:
    msg: str = embed("index.html")
    return msg

def main():
    print("Run app ...\n")
    app.run("0.0.0.0", 8080)