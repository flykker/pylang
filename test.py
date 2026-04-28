
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

app = Router()

@app.post("/health")
def health():
    print("Health is OK !\n")

def main():
    print("Run app ...\n")
    health()
