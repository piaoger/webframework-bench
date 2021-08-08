## BENCHMARK REPORT
### actix
```text
Running 10s test @ http://127.0.0.1:8082/user
  8 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.22ms    1.67ms  35.11ms   93.78%
    Req/Sec    37.57k     5.00k   56.18k    76.88%
  Latency Distribution
     50%  742.00us
     75%    1.10ms
     90%    1.74ms
     99%    9.89ms
  2990632 requests in 10.04s, 459.19MB read
Requests/sec: 297836.80
Transfer/sec:     45.73MB
```
### axum
```text
Running 10s test @ http://127.0.0.1:8083/user
  8 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.24ms    4.99ms 200.63ms   99.76%
    Req/Sec    52.32k     3.20k   68.71k    90.12%
  Latency Distribution
     50%    0.98ms
     75%    1.24ms
     90%    1.59ms
     99%    2.77ms
  4163841 requests in 10.10s, 639.32MB read
Requests/sec: 412253.40
Transfer/sec:     63.30MB
```
### warp
```text
Running 10s test @ http://127.0.0.1:8081/user
  8 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.02ms    3.01ms 200.48ms   99.78%
    Req/Sec    54.59k     6.66k   72.10k    86.49%
  Latency Distribution
     50%    0.88ms
     75%    1.13ms
     90%    1.53ms
     99%    2.90ms
  4301734 requests in 10.01s, 660.49MB read
Requests/sec: 429766.33
Transfer/sec:     65.99MB
```
### net_http
```text
Running 10s test @ http://127.0.0.1:8091/user
  8 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     8.31ms   12.10ms 204.93ms   86.53%
    Req/Sec    14.70k     2.49k   22.99k    68.99%
  Latency Distribution
     50%    2.44ms
     75%   11.01ms
     90%   25.17ms
     99%   52.72ms
  1163094 requests in 10.09s, 178.58MB read
Requests/sec: 115250.47
Transfer/sec:     17.70MB
```
