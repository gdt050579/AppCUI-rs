import http.server
import socketserver

PORT = 8080

class CORSRequestHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
        self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
        self.send_header('Access-Control-Allow-Origin', '*')
        super().end_headers()

    def guess_type(self, path):
        if path.endswith('.wasm'):
            return 'application/wasm'
        if path.endswith('.js'):
            return 'application/javascript'
        return super().guess_type(path)

if __name__ == "__main__":
    with socketserver.TCPServer(("", PORT), CORSRequestHandler) as httpd:
        print(f"Serving at http://localhost:{PORT}")
        print("Press Ctrl+C to stop")
        httpd.serve_forever()

