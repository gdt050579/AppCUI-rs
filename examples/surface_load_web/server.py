import http.server
import socketserver
import os

class CustomHandler(http.server.SimpleHTTPRequestHandler):
    def send_head(self):
        path = self.translate_path(self.path)
        if os.path.isfile(path):
            f = open(path, 'rb')
            fs = os.fstat(f.fileno())
            self.send_response(200)
            if path.endswith('.js'):
                mime_type = "application/javascript"
            elif path.endswith('.wasm'):
                mime_type = "application/wasm"
            else:
                mime_type = "text/html"
            self.send_header("Content-Type", mime_type)
            self.send_header("Content-Length", str(fs.st_size))
            self.send_header("Cross-Origin-Opener-Policy", "same-origin")
            self.send_header("Cross-Origin-Embedder-Policy", "require-corp")
            self.end_headers()
            return f
        return super().send_head()

    def do_GET(self):
        f = self.send_head()
        if f:
            try:
                self.wfile.write(f.read())
            finally:
                f.close()

PORT = 4000
with socketserver.TCPServer(("", PORT), CustomHandler) as httpd:
    print(f"Serving on port {PORT}")
    httpd.serve_forever()