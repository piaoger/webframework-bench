## BENCHMARK REPORT
### actix
```text
Running 10s test @ http://127.0.0.1:8081/user
  4 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   681.24us    1.22ms  57.98ms   99.42%
    Req/Sec    41.92k     4.79k   54.33k    78.25%
  Latency Distribution
     50%  590.00us
     75%  685.00us
     90%  841.00us
     99%    1.45ms
  1668387 requests in 10.07s, 249.80MB read
Requests/sec: 165719.18
Transfer/sec:     24.81MB
```
### warp
```text
Running 10s test @ http://127.0.0.1:8082/user
  4 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   435.48us  604.32us  28.75ms   99.03%
    Req/Sec    75.52k     9.01k   87.50k    75.50%
  Latency Distribution
     50%  375.00us
     75%  537.00us
     90%  627.00us
     99%    1.03ms
  3004672 requests in 10.06s, 449.88MB read
Requests/sec: 298700.71
Transfer/sec:     44.72MB
```
### net_http
```text
Running 10s test @ http://127.0.0.1:8083/user
  4 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.51ms    2.98ms  42.43ms   84.20%
    Req/Sec    23.85k     1.00k   26.17k    80.00%
  Latency Distribution
     50%    0.86ms
     75%    3.60ms
     90%    7.12ms
     99%   11.79ms
  949147 requests in 10.06s, 142.11MB read
Requests/sec:  94339.20
Transfer/sec:     14.13MB
```
