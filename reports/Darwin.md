## BENCHMARK REPORT
### actix
```text
Running 10s test @ http://127.0.0.1:8082/user
  4 threads and 128 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.27ms  492.44us  12.81ms   86.65%
    Req/Sec    23.63k     2.91k   33.61k    72.50%
  Latency Distribution
     50%    1.25ms
     75%    1.39ms
     90%    1.58ms
     99%    3.36ms
  940897 requests in 10.01s, 147.16MB read
Requests/sec:  93962.68
Transfer/sec:     14.70MB
```
### axum
```text
Running 10s test @ http://127.0.0.1:8083/user
  4 threads and 128 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.17ms  345.80us   6.41ms   77.70%
    Req/Sec    25.61k     2.48k   31.42k    75.50%
  Latency Distribution
     50%    1.14ms
     75%    1.31ms
     90%    1.55ms
     99%    2.38ms
  1029665 requests in 10.10s, 161.04MB read
Requests/sec: 101911.63
Transfer/sec:     15.94MB
```
### warp
```text
Running 10s test @ http://127.0.0.1:8081/user
  4 threads and 128 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.12ms  255.75us  10.53ms   86.79%
    Req/Sec    27.59k     2.73k   77.54k    98.50%
  Latency Distribution
     50%    1.14ms
     75%    1.22ms
     90%    1.30ms
     99%    1.53ms
  1100979 requests in 10.10s, 172.20MB read
Requests/sec: 109000.33
Transfer/sec:     17.05MB
```
### net_http
```text
Running 10s test @ http://127.0.0.1:8091/user
  4 threads and 128 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.80ms    2.00ms  26.84ms   88.32%
    Req/Sec    21.28k     6.25k   35.87k    69.75%
  Latency Distribution
     50%    1.12ms
     75%    2.18ms
     90%    4.15ms
     99%   10.23ms
  850223 requests in 10.08s, 133.79MB read
Requests/sec:  84373.58
Transfer/sec:     13.28MB
```
