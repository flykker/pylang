
class HttpServer:
    fd = 0

    def __init___(self):
        self.fd = 0

    def run(self, host, port):
        self.fd = socket(2, 1, 0)

        err = bind(self.fd, host, port)
        if err < 0:
            print("Address already in use !!!\n")
            exit(1)

        listen(self.fd, 10)
        print(f"Running on port {port} ...\n")

        while 1:
            conn = accept(self.fd)
            data = recv(conn, 1024)

            if len(data) > 0:
                response = "HTTP/1.1 200 OK\r\nContent-Length: 11\r\n\r\nHello World"
                send(conn, response)
            close(conn)

        close(self.fd)

class Router:
    routers = []
    
    def __init__(self):
        pass

    def add_route(self, path, endpoint):
        print("Route registered !!!\n")

    def post(self, path: str):
        def decorator(func):
            self.add_route(path, func)
            return func
        return decorator

router = Router()

@router.post("/health")
def health():
    print("Health is OK !\n")

def main():
    app = HttpServer()
    app.run("0.0.0.0", 8080)

    print("Run app ...\n")
    health()
