import http.server, scketserver
Handler = http.server.SimpleHTTPRequestHandler
Handler.extensions_map['.wasm'] = 'application/wasm'

port = 8888
with scketserver.TCPServer(("", port), Handler) as httpd:
    print(f"serving at port {port}")
    try:
        httpd.serve_forever()
    except:
        ...
    finally:
        httpd.server_close()