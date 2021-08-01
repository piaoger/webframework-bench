## BENCHMARK REPORT
### actix
```text
Running 10s test @ http://127.0.0.1:8082/user
  4 threads and 128 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     7.42ms   20.85ms 251.03ms   92.05%
    Req/Sec    22.32k     7.52k   49.60k    73.09%
  Latency Distribution
     50%    1.15ms
     75%    1.74ms
     90%   17.80ms
     99%  110.96ms
  871654 requests in 10.03s, 133.84MB read
Requests/sec:  86871.14
Transfer/sec:     13.34MB
```
### axum
```text
Running 10s test @ http://127.0.0.1:8083/user
  4 threads and 128 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.22ms  415.12us   6.68ms   76.23%
    Req/Sec    24.21k     2.07k   30.79k    76.00%
  Latency Distribution
     50%    1.16ms
     75%    1.38ms
     90%    1.71ms
     99%    2.61ms
  963746 requests in 10.01s, 147.98MB read
Requests/sec:  96301.52
Transfer/sec:     14.79MB
```
### warp
```text
Running 10s test @ http://127.0.0.1:8081/user
  4 threads and 128 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.16ms  367.63us   5.81ms   76.87%
    Req/Sec    24.44k     2.54k   58.64k    84.29%
  Latency Distribution
     50%    1.13ms
     75%    1.31ms
     90%    1.58ms
     99%    2.38ms
  975586 requests in 10.10s, 149.79MB read
Requests/sec:  96579.72
Transfer/sec:     14.83MB
```
### net_http
```text
Running 10s test @ http://127.0.0.1:8091/user
  4 threads and 128 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.62ms    1.66ms  31.58ms   88.64%
    Req/Sec    22.31k     5.60k   38.56k    73.25%
  Latency Distribution
     50%    1.13ms
     75%    1.86ms
     90%    3.52ms
     99%    8.72ms
  890136 requests in 10.06s, 136.67MB read
Requests/sec:  88523.13
Transfer/sec:     13.59MB
```
