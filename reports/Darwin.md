## BENCHMARK REPORT
### actix
```text
Running 10s test @ http://127.0.0.1:8082/user
  4 threads and 128 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     4.43ms    5.82ms  95.55ms   94.47%
    Req/Sec     8.31k     1.81k   14.48k    74.87%
  Latency Distribution
     50%    3.36ms
     75%    4.15ms
     90%    6.22ms
     99%   32.45ms
  332649 requests in 10.10s, 51.08MB read
Requests/sec:  32939.53
Transfer/sec:      5.06MB
```
### axum
```text
Running 10s test @ http://127.0.0.1:8083/user
  4 threads and 128 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     3.77ms    1.38ms  13.88ms   71.45%
    Req/Sec     7.78k     1.10k   11.61k    73.50%
  Latency Distribution
     50%    3.62ms
     75%    4.52ms
     90%    5.56ms
     99%    7.87ms
  311138 requests in 10.06s, 47.77MB read
Requests/sec:  30938.82
Transfer/sec:      4.75MB
```
### warp
```text
Running 10s test @ http://127.0.0.1:8081/user
  4 threads and 128 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     3.25ms    1.12ms   9.55ms   71.21%
    Req/Sec     8.63k     1.37k   13.69k    77.50%
  Latency Distribution
     50%    3.12ms
     75%    3.87ms
     90%    4.68ms
     99%    6.61ms
  345471 requests in 10.06s, 53.04MB read
Requests/sec:  34335.83
Transfer/sec:      5.27MB
```
### net_http
```text
Running 10s test @ http://127.0.0.1:8091/user
  4 threads and 128 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     5.65ms    6.84ms  96.21ms   88.94%
    Req/Sec     6.91k     2.32k   14.34k    69.47%
  Latency Distribution
     50%    3.26ms
     75%    6.99ms
     90%   13.18ms
     99%   33.59ms
  275414 requests in 10.09s, 42.29MB read
Requests/sec:  27287.19
Transfer/sec:      4.19MB
```
