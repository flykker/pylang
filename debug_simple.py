class HttpServer:
    fd: int = 0

    def run(self, host: str, port: int):
        self.fd = socket(2, 1, 0)
        bind(self.fd, host, port)
        listen(self.fd, 10)
        print("Listening\n")
        while 1:
            conn = accept(self.fd)
            send(conn, "OK\n")
            close(conn)
        close(self.fd)

def main():
    s = HttpServer()
    s.run("0.0.0.0", 8080)
