# from http.server import BaseHTTPRequestHandler, HTTPServer
# from urllib.parse import parse_qs

# class RequestHandler(BaseHTTPRequestHandler):
#     def do_GET(self):
#         self.protocol_version = "HTTP/1.1"
#         self.send_response(200)
#         self.end_headers()

#         if '?q=' in self.path:
#             q = parse_qs(self.path[2:])["q"]
#             with open('log.txt','a') as f:
#                 for cookie in q:
#                     f.write(str(cookie)+'\n')

#         return

def run():
    server = ('', 8000)
    httpd = HTTPServer(server, RequestHandler)
    httpd.serve_forever()

# run()