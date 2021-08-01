## BENCHMARK REPORT
### actix
```text
Running 10s test @ http://127.0.0.1:8082/user
  4 threads and 128 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.35ms    2.90ms 125.82ms   99.07%
    Req/Sec    24.14k     3.69k   35.75k    72.75%
  Latency Distribution
     50%    1.17ms
     75%    1.35ms
     90%    1.61ms
     99%    4.09ms
  961057 requests in 10.01s, 150.31MB read
Requests/sec:  95999.93
Transfer/sec:     15.01MB
```
### axum
```text
Running 10s test @ http://127.0.0.1:8083/user
  4 threads and 128 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.21ms  383.61us   5.58ms   75.65%
    Req/Sec    24.38k     2.39k   31.51k    74.50%
  Latency Distribution
     50%    1.17ms
     75%    1.37ms
     90%    1.67ms
     99%    2.48ms
  970601 requests in 10.01s, 151.80MB read
Requests/sec:  97007.37
Transfer/sec:     15.17MB
```
### warp
```text
Running 10s test @ http://127.0.0.1:8081/user
  4 threads and 128 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.38ms    1.54ms  49.55ms   97.45%
    Req/Sec    23.64k     5.35k   59.99k    86.34%
  Latency Distribution
     50%    1.18ms
     75%    1.38ms
     90%    1.78ms
     99%    6.35ms
  913944 requests in 9.75s, 142.94MB read
  Socket errors: connect 0, read 0, write 0, timeout 115
Requests/sec:  93778.88
Transfer/sec:     14.67MB
```
### net_http
```text
Running 10s test @ http://127.0.0.1:8091/user
  4 threads and 128 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.30ms  798.99us  16.15ms   79.61%
    Req/Sec    24.67k     2.51k   61.72k    88.28%
  Latency Distribution
     50%    1.18ms
     75%    1.40ms
     90%    2.23ms
     99%    4.36ms
  984445 requests in 10.10s, 152.09MB read
Requests/sec:  97441.12
Transfer/sec:     15.05MB
```
