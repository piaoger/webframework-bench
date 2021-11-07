## BENCHMARK REPORT
### warp
```text
Running 10s test @ http://127.0.0.1:8081/user
  8 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     0.94ms    0.89ms  43.15ms   95.12%
    Req/Sec    56.65k     5.65k   73.62k    78.88%
  Latency Distribution
     50%    0.85ms
     75%    1.10ms
     90%    1.45ms
     99%    2.91ms
  4509106 requests in 10.01s, 705.24MB read
Requests/sec: 450281.87
Transfer/sec:     70.43MB
```
### actix
```text
Running 10s test @ http://127.0.0.1:8082/user
  8 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.14ms    1.82ms 199.38ms   95.40%
    Req/Sec    36.67k     4.49k   49.86k    73.23%
  Latency Distribution
     50%  751.00us
     75%    0.95ms
     90%    1.62ms
     99%    9.28ms
  2889967 requests in 10.03s, 452.00MB read
Requests/sec: 288189.05
Transfer/sec:     45.07MB
```
### axum
```text
Running 10s test @ http://127.0.0.1:8083/user
  8 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.13ms    4.27ms 199.97ms   99.76%
    Req/Sec    54.94k     4.27k   70.61k    82.50%
  Latency Distribution
     50%    0.90ms
     75%    1.16ms
     90%    1.54ms
     99%    2.97ms
  4373558 requests in 10.07s, 684.04MB read
Requests/sec: 434356.27
Transfer/sec:     67.93MB
```
### poem
```text
Running 10s test @ http://127.0.0.1:8084/user
  8 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.06ms    3.21ms 200.69ms   99.77%
    Req/Sec    54.05k     6.02k   73.83k    89.28%
  Latency Distribution
     50%    0.90ms
     75%    1.15ms
     90%    1.53ms
     99%    2.89ms
  4264227 requests in 10.06s, 666.94MB read
Requests/sec: 424068.57
Transfer/sec:     66.33MB
```
### net_http
```text
Running 10s test @ http://127.0.0.1:8091/user
  8 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     8.80ms   13.22ms 214.90ms   86.55%
    Req/Sec    14.84k     2.67k   31.53k    69.54%
  Latency Distribution
     50%    2.37ms
     75%   11.42ms
     90%   27.32ms
     99%   56.76ms
  1172342 requests in 10.10s, 184.48MB read
Requests/sec: 116074.72
Transfer/sec:     18.27MB
```
