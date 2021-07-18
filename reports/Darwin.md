## BENCHMARK REPORT
### warp
```text
Running 10s test @ http://127.0.0.1:8081/user
  4 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.83ms  338.34us   7.20ms   76.49%
    Req/Sec    26.12k     1.81k   30.77k    71.50%
  Latency Distribution
     50%    1.87ms
     75%    2.01ms
     90%    2.17ms
     99%    2.64ms
  1039916 requests in 10.01s, 155.70MB read
  Socket errors: connect 0, read 50, write 0, timeout 0
Requests/sec: 103910.75
Transfer/sec:     15.56MB
```
### actix
```text
Running 10s test @ http://127.0.0.1:8082/user
  4 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.09ms  417.50us  15.87ms   87.97%
    Req/Sec    23.56k     1.76k   32.39k    79.95%
  Latency Distribution
     50%    2.12ms
     75%    2.21ms
     90%    2.31ms
     99%    3.29ms
  946816 requests in 10.10s, 141.76MB read
  Socket errors: connect 0, read 51, write 0, timeout 0
Requests/sec:  93706.21
Transfer/sec:     14.03MB
```
### net_http
```text
Running 10s test @ http://127.0.0.1:8083/user
  4 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.72ms    2.91ms  48.90ms   88.37%
    Req/Sec    21.33k     5.52k   35.08k    71.25%
  Latency Distribution
     50%    1.75ms
     75%    3.39ms
     90%    6.07ms
     99%   14.24ms
  851511 requests in 10.05s, 127.49MB read
  Socket errors: connect 0, read 60, write 0, timeout 0
Requests/sec:  84717.36
Transfer/sec:     12.68MB
```
