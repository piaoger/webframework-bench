## BENCHMARK REPORT
### actix
```text
Running 10s test @ http://127.0.0.1:8082/user
  8 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.22ms    1.71ms  44.11ms   93.88%
    Req/Sec    38.05k     4.46k   51.65k    74.49%
  Latency Distribution
     50%  727.00us
     75%    1.09ms
     90%    1.71ms
     99%    9.94ms
  2998374 requests in 10.02s, 460.38MB read
Requests/sec: 299301.41
Transfer/sec:     45.96MB
```
### axum
```text
Running 10s test @ http://127.0.0.1:8083/user
  8 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.16ms    3.80ms 199.76ms   99.80%
    Req/Sec    52.39k     3.05k   75.80k    84.99%
  Latency Distribution
     50%    0.97ms
     75%    1.22ms
     90%    1.57ms
     99%    2.75ms
  4133171 requests in 10.06s, 634.61MB read
Requests/sec: 410996.98
Transfer/sec:     63.11MB
```
### warp
```text
Running 10s test @ http://127.0.0.1:8081/user
  8 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.05ms    3.42ms 200.60ms   99.76%
    Req/Sec    54.89k     6.11k   73.23k    86.88%
  Latency Distribution
     50%    0.87ms
     75%    1.13ms
     90%    1.54ms
     99%    3.04ms
  4368407 requests in 10.09s, 670.73MB read
Requests/sec: 433151.38
Transfer/sec:     66.51MB
```
### net_http
```text
Running 10s test @ http://127.0.0.1:8091/user
  8 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     8.57ms   13.15ms 244.26ms   87.05%
    Req/Sec    14.84k     2.59k   21.27k    66.04%
  Latency Distribution
     50%    2.42ms
     75%   11.02ms
     90%   26.08ms
     99%   55.58ms
  1179046 requests in 10.07s, 181.03MB read
Requests/sec: 117044.75
Transfer/sec:     17.97MB
```
