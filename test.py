
class HttpServer:
    fd = 0
    routers = {}

    def __init___(self, routers):
        self.fd = 0
        self.routers = routers

    def run(self, host, port):
        self.fd = socket(2, 1, 0)

        err = bind(self.fd, host, port)
        if err < 0:
            print("Address {host}:{port} already in use !!!\n")
            exit(1)

        listen(self.fd, 10)
        print(f"Running on port {port} ...\n")

        while 1:
            conn = accept(self.fd)
            data = recv(conn, 1024)

            if len(data) > 0:
                content = self.routers["/health"]()
                length = len(content)
                response = f"HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{content}\n"
                send(conn, response)
            close(conn)

        close(self.fd)


class Router:
    routers = {}
    
    def __init__(self):
        self.routers = {}

    def add_route(self, path: str, endpoint):
        self.routers[path] = endpoint

        print(f"Route registered {path} !!!\n")


class FastPy:
    def __init__(self):
        self.router = Router()
        self.app = HttpServer(self.router.routers)

    def run(self, host: str, port: int):
        self.app.run("0.0.0.0", 8080)

    def post(self, path: str):
        def decorator(func):
            self.router.add_route(path, func)
            return func
        return decorator

app = FastPy()

@app.post("/health")
def health() -> str:
    print("Log:Main Health is OK !\n")
    return "{'health':'ok'}"

def main():
    print("Run app ...\n")
    app.run("0.0.0.0", 8080)

