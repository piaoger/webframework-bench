## BENCHMARK REPORT
### actix
```text
Running 10s test @ http://127.0.0.1:8082/user
  4 threads and 128 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.27ms    1.73ms 100.00ms   98.36%
    Req/Sec    24.43k     3.39k   42.54k    73.00%
  Latency Distribution
     50%    1.16ms
     75%    1.33ms
     90%    1.59ms
     99%    3.65ms
  973280 requests in 10.02s, 149.44MB read
Requests/sec:  97169.99
Transfer/sec:     14.92MB
```
### axum
```text
Running 10s test @ http://127.0.0.1:8083/user
  4 threads and 128 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.20ms  395.29us   4.94ms   74.79%
    Req/Sec    24.44k     2.14k   30.76k    74.26%
  Latency Distribution
     50%    1.16ms
     75%    1.38ms
     90%    1.69ms
     99%    2.47ms
  982478 requests in 10.10s, 150.85MB read
Requests/sec:  97239.62
Transfer/sec:     14.93MB
```
### warp
```text
Running 10s test @ http://127.0.0.1:8081/user
  4 threads and 128 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.13ms  298.62us   3.22ms   74.09%
    Req/Sec    25.11k     2.19k   56.56k    85.04%
  Latency Distribution
     50%    1.13ms
     75%    1.28ms
     90%    1.48ms
     99%    1.96ms
  1002191 requests in 10.10s, 153.88MB read
Requests/sec:  99207.83
Transfer/sec:     15.23MB
```
### net_http
```text
Running 10s test @ http://127.0.0.1:8091/user
  4 threads and 128 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.57ms    1.48ms  22.46ms   87.55%
    Req/Sec    22.18k     5.37k   37.19k    72.00%
  Latency Distribution
     50%    1.13ms
     75%    1.87ms
     90%    3.43ms
     99%    7.51ms
  884661 requests in 10.04s, 135.83MB read
Requests/sec:  88100.30
Transfer/sec:     13.53MB
```
