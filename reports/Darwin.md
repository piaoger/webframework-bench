## BENCHMARK REPORT
### warp
```text
Running 10s test @ http://127.0.0.1:8081/user
  4 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.01ms    1.16ms  32.56ms   95.46%
    Req/Sec    24.15k     3.21k   29.82k    87.25%
  Latency Distribution
     50%    1.90ms
     75%    2.10ms
     90%    2.42ms
     99%    5.49ms
  961891 requests in 10.02s, 144.02MB read
  Socket errors: connect 0, read 62, write 0, timeout 0
Requests/sec:  96019.81
Transfer/sec:     14.38MB
```
### actix
```text
Running 10s test @ http://127.0.0.1:8082/user
  4 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.14ms    0.89ms  23.70ms   91.81%
    Req/Sec    22.68k     2.64k   32.48k    81.75%
  Latency Distribution
     50%    2.11ms
     75%    2.26ms
     90%    2.48ms
     99%    5.16ms
  904042 requests in 10.02s, 135.36MB read
  Socket errors: connect 0, read 52, write 0, timeout 0
Requests/sec:  90195.03
Transfer/sec:     13.50MB
```
### net_http
```text
Running 10s test @ http://127.0.0.1:8083/user
  4 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.64ms    2.59ms  48.55ms   88.04%
    Req/Sec    20.58k     4.91k   36.07k    70.50%
  Latency Distribution
     50%    1.87ms
     75%    3.27ms
     90%    5.66ms
     99%   11.58ms
  821471 requests in 10.05s, 123.00MB read
  Socket errors: connect 0, read 59, write 0, timeout 0
Requests/sec:  81698.57
Transfer/sec:     12.23MB
```
