class HttpServer:
    fd: int = 0
    routers: dict[str, Callable] = {}

    def __init__(self, routers: dict[str, Callable]):
        self.fd = 0
        self.routers = routers

    def run(self, host: str, port: int) -> int:
        self.fd = socket_sys(2, 1, 0)

        err: int = bind(self.fd, host, port)
        if err < 0:
            print(f"Address {host}:{port} already in use !!!\n")
            exit(1)

        listen(self.fd, 10)
        print(f"Running on port {port} ...\n")

        while 1:
            conn: int = accept_sys(self.fd)
            
            pid: int = fork_sys()
            if pid == 0:
                buf: ptr = alloc_sys(1032)
                data_n: int = recv_sys(conn, buf, 1024)

                if data_n > 0:
                    content: str = self.routers["/"]()
                    length: int = len(content)
                    response: str = f"HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{content}"
                    send(conn, response)
                close(conn)
                exit(0)
        close_sys(self.fd)


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