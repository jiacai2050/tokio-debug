## How to run
- First start server with `make serve`.
  - It will start a simple HTTP server, which take 50ms to finish any requests.
- Then run tokio demo with `RUST_LOG=info cargo run --release`
  - Change log level to `trace` to see how `reqwest` make requests.

Log below is what I get on my macbook, every requests should cost server 50ms in theory.
```
    Finished release [optimized] target(s) in 0.05s
     Running `target/release/foo`
Start running...
CPU tasks started...
IO tasks started...
[2023-08-06T14:19:56.527Z INFO  foo] io-0 begin
[2023-08-06T14:19:56.527Z INFO  foo] cpu-0 begin
[2023-08-06T14:19:56.527Z DEBUG reqwest::connect] starting new connection: http://127.0.0.1:8080/
[2023-08-06T14:19:56.527Z DEBUG reqwest::connect] starting new connection: http://127.0.0.1:8080/
[2023-08-06T14:19:56.528Z TRACE reqwest::connect::verbose] 8170baa2 write (vectored): b"GET /cpu-0 HTTP/1.1\r\naccept: */*\r\nhost: 127.0.0.1:8080\r\n\r\n"
[2023-08-06T14:19:56.528Z TRACE reqwest::connect::verbose] 0250728a write (vectored): b"GET /io-0 HTTP/1.1\r\naccept: */*\r\nhost: 127.0.0.1:8080\r\n\r\n"
[2023-08-06T14:19:56.538Z INFO  foo] io-1 begin
[2023-08-06T14:19:56.538Z DEBUG reqwest::connect] starting new connection: http://127.0.0.1:8080/
[2023-08-06T14:19:56.538Z TRACE reqwest::connect::verbose] 63391838 write (vectored): b"GET /io-1 HTTP/1.1\r\naccept: */*\r\nhost: 127.0.0.1:8080\r\n\r\n"
[2023-08-06T14:19:56.539Z INFO  foo] cpu-1 begin
[2023-08-06T14:19:56.539Z DEBUG reqwest::connect] starting new connection: http://127.0.0.1:8080/
[2023-08-06T14:19:56.539Z TRACE reqwest::connect::verbose] c29312cf write (vectored): b"GET /cpu-1 HTTP/1.1\r\naccept: */*\r\nhost: 127.0.0.1:8080\r\n\r\n"
[2023-08-06T14:19:56.549Z INFO  foo] io-2 begin
[2023-08-06T14:19:56.549Z DEBUG reqwest::connect] starting new connection: http://127.0.0.1:8080/
[2023-08-06T14:19:56.549Z TRACE reqwest::connect::verbose] 573aadce write (vectored): b"GET /io-2 HTTP/1.1\r\naccept: */*\r\nhost: 127.0.0.1:8080\r\n\r\n"
[2023-08-06T14:19:56.552Z INFO  foo] cpu-2 begin
[2023-08-06T14:19:56.552Z DEBUG reqwest::connect] starting new connection: http://127.0.0.1:8080/
[2023-08-06T14:19:56.552Z TRACE reqwest::connect::verbose] 0d2ed280 write (vectored): b"GET /cpu-2 HTTP/1.1\r\naccept: */*\r\nhost: 127.0.0.1:8080\r\n\r\n"
[2023-08-06T14:19:56.560Z INFO  foo] io-3 begin
[2023-08-06T14:19:56.560Z DEBUG reqwest::connect] starting new connection: http://127.0.0.1:8080/
[2023-08-06T14:19:56.560Z TRACE reqwest::connect::verbose] 3bd70f04 write (vectored): b"GET /io-3 HTTP/1.1\r\naccept: */*\r\nhost: 127.0.0.1:8080\r\n\r\n"
[2023-08-06T14:19:56.564Z INFO  foo] cpu-3 begin
[2023-08-06T14:19:56.564Z DEBUG reqwest::connect] starting new connection: http://127.0.0.1:8080/
[2023-08-06T14:19:56.565Z TRACE reqwest::connect::verbose] 8f81117c write (vectored): b"GET /cpu-3 HTTP/1.1\r\naccept: */*\r\nhost: 127.0.0.1:8080\r\n\r\n"
[2023-08-06T14:19:56.571Z INFO  foo] io-4 begin
[2023-08-06T14:19:56.571Z DEBUG reqwest::connect] starting new connection: http://127.0.0.1:8080/
[2023-08-06T14:19:56.571Z TRACE reqwest::connect::verbose] cdd2b23e write (vectored): b"GET /io-4 HTTP/1.1\r\naccept: */*\r\nhost: 127.0.0.1:8080\r\n\r\n"
[2023-08-06T14:19:56.577Z INFO  foo] cpu-4 begin
[2023-08-06T14:19:56.577Z DEBUG reqwest::connect] starting new connection: http://127.0.0.1:8080/
[2023-08-06T14:19:56.577Z TRACE reqwest::connect::verbose] 61e078ed write (vectored): b"GET /cpu-4 HTTP/1.1\r\naccept: */*\r\nhost: 127.0.0.1:8080\r\n\r\n"
[2023-08-06T14:19:56.578Z TRACE reqwest::connect::verbose] 0250728a read: b"HTTP/1.1 200 OK\r\nDate: Sun, 06 Aug 2023 14:19:56 GMT\r\nContent-Length: 14\r\nContent-Type: text/plain; charset=utf-8\r\n\r\nHello, \"/io-0\""
[2023-08-06T14:19:56.578Z TRACE reqwest::connect::verbose] 8170baa2 read: b"HTTP/1.1 200 OK\r\nDate: Sun, 06 Aug 2023 14:19:56 GMT\r\nContent-Length: 15\r\nContent-Type: text/plain; charset=utf-8\r\n\r\nHello, \"/cpu-0\""
[2023-08-06T14:19:56.578Z INFO  foo] io-0 cost:51.623167ms
[2023-08-06T14:19:56.578Z INFO  foo] cpu-0 cost:51.618542ms
[2023-08-06T14:19:56.581Z INFO  foo] io-5 begin
[2023-08-06T14:19:56.589Z INFO  foo] cpu-5 begin
[2023-08-06T14:19:56.589Z TRACE reqwest::connect::verbose] 0250728a write (vectored): b"GET /cpu-5 HTTP/1.1\r\naccept: */*\r\nhost: 127.0.0.1:8080\r\n\r\n"
[2023-08-06T14:19:56.590Z TRACE reqwest::connect::verbose] 63391838 read: b"HTTP/1.1 200 OK\r\nDate: Sun, 06 Aug 2023 14:19:56 GMT\r\nContent-Length: 14\r\nContent-Type: text/plain; charset=utf-8\r\n\r\nHello, \"/io-1\""
[2023-08-06T14:19:56.590Z INFO  foo] io-1 cost:51.527625ms
[2023-08-06T14:19:56.590Z TRACE reqwest::connect::verbose] c29312cf read: b"HTTP/1.1 200 OK\r\nDate: Sun, 06 Aug 2023 14:19:56 GMT\r\nContent-Length: 15\r\nContent-Type: text/plain; charset=utf-8\r\n\r\nHello, \"/cpu-1\""
[2023-08-06T14:19:56.590Z INFO  foo] cpu-1 cost:50.408375ms
[2023-08-06T14:19:56.592Z INFO  foo] io-6 begin
[2023-08-06T14:19:56.601Z TRACE reqwest::connect::verbose] 573aadce read: b"HTTP/1.1 200 OK\r\nDate: Sun, 06 Aug 2023 14:19:56 GMT\r\nContent-Length: 14\r\nContent-Type: text/plain; charset=utf-8\r\n\r\nHello, \"/io-2\""
[2023-08-06T14:19:56.601Z INFO  foo] io-2 cost:51.476542ms
[2023-08-06T14:19:56.602Z INFO  foo] io-7 begin
[2023-08-06T14:19:56.602Z TRACE reqwest::connect::verbose] 573aadce write (vectored): b"GET /io-7 HTTP/1.1\r\naccept: */*\r\nhost: 127.0.0.1:8080\r\n\r\n"
[2023-08-06T14:19:56.612Z TRACE reqwest::connect::verbose] 3bd70f04 read: b"HTTP/1.1 200 OK\r\nDate: Sun, 06 Aug 2023 14:19:56 GMT\r\nContent-Length: 14\r\nContent-Type: text/plain; charset=utf-8\r\n\r\nHello, \"/io-3\""
[2023-08-06T14:19:56.612Z INFO  foo] io-3 cost:51.598083ms
[2023-08-06T14:19:56.612Z INFO  foo] io-8 begin
[2023-08-06T14:19:56.612Z TRACE reqwest::connect::verbose] 3bd70f04 write (vectored): b"GET /io-8 HTTP/1.1\r\naccept: */*\r\nhost: 127.0.0.1:8080\r\n\r\n"
[2023-08-06T14:19:56.622Z TRACE reqwest::connect::verbose] cdd2b23e read: b"HTTP/1.1 200 OK\r\nDate: Sun, 06 Aug 2023 14:19:56 GMT\r\nContent-Length: 14\r\nContent-Type: text/plain; charset=utf-8\r\n\r\nHello, \"/io-4\""
[2023-08-06T14:19:56.622Z INFO  foo] io-4 cost:51.323834ms
[2023-08-06T14:19:56.623Z INFO  foo] io-9 begin
[2023-08-06T14:19:56.623Z TRACE reqwest::connect::verbose] cdd2b23e write (vectored): b"GET /io-9 HTTP/1.1\r\naccept: */*\r\nhost: 127.0.0.1:8080\r\n\r\n"
[2023-08-06T14:19:56.641Z TRACE reqwest::connect::verbose] 0250728a read: b"HTTP/1.1 200 OK\r\nDate: Sun, 06 Aug 2023 14:19:56 GMT\r\nContent-Length: 15\r\nContent-Type: text/plain; charset=utf-8\r\n\r\nHello, \"/cpu-5\""
[2023-08-06T14:19:56.654Z TRACE reqwest::connect::verbose] 573aadce read: b"HTTP/1.1 200 OK\r\nDate: Sun, 06 Aug 2023 14:19:56 GMT\r\nContent-Length: 14\r\nContent-Type: text/plain; charset=utf-8\r\n\r\nHello, \"/io-7\""
[2023-08-06T14:19:56.654Z INFO  foo] io-7 cost:51.344333ms
[2023-08-06T14:19:56.664Z TRACE reqwest::connect::verbose] 3bd70f04 read: b"HTTP/1.1 200 OK\r\nDate: Sun, 06 Aug 2023 14:19:56 GMT\r\nContent-Length: 14\r\nContent-Type: text/plain; charset=utf-8\r\n\r\nHello, \"/io-8\""
[2023-08-06T14:19:56.664Z INFO  foo] io-8 cost:51.205083ms
[2023-08-06T14:19:56.674Z TRACE reqwest::connect::verbose] cdd2b23e read: b"HTTP/1.1 200 OK\r\nDate: Sun, 06 Aug 2023 14:19:56 GMT\r\nContent-Length: 14\r\nContent-Type: text/plain; charset=utf-8\r\n\r\nHello, \"/io-9\""
[2023-08-06T14:19:56.674Z INFO  foo] io-9 cost:51.243542ms
[2023-08-06T14:19:56.784Z TRACE reqwest::connect::verbose] 8170baa2 write (vectored): b"GET /io-5 HTTP/1.1\r\naccept: */*\r\nhost: 127.0.0.1:8080\r\n\r\n"
[2023-08-06T14:19:56.784Z INFO  foo] cpu-6 begin
[2023-08-06T14:19:56.784Z INFO  foo] cpu-7 begin
[2023-08-06T14:19:56.784Z INFO  foo] cpu-8 begin
[2023-08-06T14:19:56.784Z INFO  foo] cpu-9 begin
[2023-08-06T14:19:56.784Z TRACE reqwest::connect::verbose] cdd2b23e write (vectored): b"GET /cpu-6 HTTP/1.1\r\naccept: */*\r\nhost: 127.0.0.1:8080\r\n\r\n"
[2023-08-06T14:19:56.784Z TRACE reqwest::connect::verbose] 63391838 write (vectored): b"GET /cpu-9 HTTP/1.1\r\naccept: */*\r\nhost: 127.0.0.1:8080\r\n\r\n"
[2023-08-06T14:19:56.784Z INFO  foo] cpu-5 cost:194.677375ms
[2023-08-06T14:19:56.784Z TRACE reqwest::connect::verbose] 3bd70f04 write (vectored): b"GET /cpu-7 HTTP/1.1\r\naccept: */*\r\nhost: 127.0.0.1:8080\r\n\r\n"
[2023-08-06T14:19:56.784Z TRACE reqwest::connect::verbose] 573aadce write (vectored): b"GET /cpu-8 HTTP/1.1\r\naccept: */*\r\nhost: 127.0.0.1:8080\r\n\r\n"
[2023-08-06T14:19:56.795Z TRACE reqwest::connect::verbose] c29312cf write (vectored): b"GET /io-6 HTTP/1.1\r\naccept: */*\r\nhost: 127.0.0.1:8080\r\n\r\n"
[2023-08-06T14:19:56.795Z TRACE reqwest::connect::verbose] 61e078ed read: b"HTTP/1.1 200 OK\r\nDate: Sun, 06 Aug 2023 14:19:56 GMT\r\nContent-Length: 15\r\nContent-Type: text/plain; charset=utf-8\r\n\r\nHello, \"/cpu-4\""
[2023-08-06T14:19:56.795Z INFO  foo] cpu-4 cost:218.081041ms
[2023-08-06T14:19:56.835Z TRACE reqwest::connect::verbose] 573aadce read: b"HTTP/1.1 200 OK\r\nDate: Sun, 06 Aug 2023 14:19:56 GMT\r\nContent-Length: 15\r\nContent-Type: text/plain; charset=utf-8\r\n\r\nHello, \"/cpu-8\""
[2023-08-06T14:19:56.835Z TRACE reqwest::connect::verbose] 63391838 read: b"HTTP/1.1 200 OK\r\nDate: Sun, 06 Aug 2023 14:19:56 GMT\r\nContent-Length: 15\r\nContent-Type: text/plain; charset=utf-8\r\n\r\nHello, \"/cpu-9\""
[2023-08-06T14:19:56.835Z TRACE reqwest::connect::verbose] cdd2b23e read: b"HTTP/1.1 200 OK\r\nDate: Sun, 06 Aug 2023 14:19:56 GMT\r\nContent-Length: 15\r\nContent-Type: text/plain; charset=utf-8\r\n\r\nHello, \"/cpu-6\""
[2023-08-06T14:19:56.835Z TRACE reqwest::connect::verbose] 3bd70f04 read: b"HTTP/1.1 200 OK\r\nDate: Sun, 06 Aug 2023 14:19:56 GMT\r\nContent-Length: 15\r\nContent-Type: text/plain; charset=utf-8\r\n\r\nHello, \"/cpu-7\""
[2023-08-06T14:19:56.986Z INFO  foo] cpu-8 cost:201.642583ms
[2023-08-06T14:19:56.997Z TRACE reqwest::connect::verbose] 0d2ed280 read: b"HTTP/1.1 200 OK\r\nDate: Sun, 06 Aug 2023 14:19:56 GMT\r\nContent-Length: 15\r\nContent-Type: text/plain; charset=utf-8\r\n\r\nHello, \"/cpu-2\""
[2023-08-06T14:19:56.997Z INFO  foo] cpu-2 cost:445.243ms
[2023-08-06T14:19:57.191Z INFO  foo] cpu-9 cost:406.872125ms
[2023-08-06T14:19:57.202Z TRACE reqwest::connect::verbose] 8f81117c read: b"HTTP/1.1 200 OK\r\nDate: Sun, 06 Aug 2023 14:19:56 GMT\r\nContent-Length: 15\r\nContent-Type: text/plain; charset=utf-8\r\n\r\nHello, \"/cpu-3\""
[2023-08-06T14:19:57.202Z INFO  foo] cpu-3 cost:637.910834ms
[2023-08-06T14:19:57.396Z INFO  foo] cpu-6 cost:612.408708ms
[2023-08-06T14:19:57.407Z INFO  foo] cpu-7 cost:623.538875ms
[2023-08-06T14:19:57.601Z TRACE reqwest::connect::verbose] c29312cf read: b"HTTP/1.1 200 OK\r\nDate: Sun, 06 Aug 2023 14:19:56 GMT\r\nContent-Length: 14\r\nContent-Type: text/plain; charset=utf-8\r\n\r\nHello, \"/io-6\""
[2023-08-06T14:19:57.601Z TRACE reqwest::connect::verbose] 8170baa2 read: b"HTTP/1.1 200 OK\r\nDate: Sun, 06 Aug 2023 14:19:56 GMT\r\nContent-Length: 14\r\nContent-Type: text/plain; charset=utf-8\r\n\r\nHello, \"/io-5\""
[2023-08-06T14:19:57.602Z INFO  foo] io-6 cost:1.009949333s
[2023-08-06T14:19:57.602Z INFO  foo] io-5 cost:1.02032875s

```
