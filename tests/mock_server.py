#!/usr/bin/env python3
from http.server import BaseHTTPRequestHandler, HTTPServer


class MockHandler(BaseHTTPRequestHandler):
    def do_GET(self) -> None:
        if self.path in ("/WLAN", "/Security/Commissioning"):
            self.send_response(200)
        else:
            self.send_response(404)
        self.end_headers()

    def do_PUT(self) -> None:
        if self.path in ("/WLAN", "/Security/Commissioning"):
            self.send_response(200)
        else:
            self.send_response(404)
        self.end_headers()


def main() -> None:
    server = HTTPServer(("127.0.0.1", 80), MockHandler)
    print("Listening on http://127.0.0.1:80")
    server.serve_forever()


if __name__ == "__main__":
    main()
