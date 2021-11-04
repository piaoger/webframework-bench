## BENCHMARK REPORT
### warp
```text
Running 10s test @ http://127.0.0.1:8081/user
  4 threads and 128 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.15ms  345.70us  11.29ms   77.69%
    Req/Sec    24.84k     2.19k   38.07k    78.00%
  Latency Distribution
     50%    1.15ms
     75%    1.28ms
     90%    1.50ms
     99%    2.26ms
  989473 requests in 10.02s, 154.76MB read
Requests/sec:  98790.95
Transfer/sec:     15.45MB
```
### actix
```text
Running 10s test @ http://127.0.0.1:8082/user
  4 threads and 128 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.38ms    1.54ms  53.98ms   95.52%
    Req/Sec    23.83k     4.83k   45.99k    71.25%
  Latency Distribution
     50%    1.19ms
     75%    1.38ms
     90%    1.86ms
     99%    6.12ms
  949448 requests in 10.02s, 148.50MB read
Requests/sec:  94774.51
Transfer/sec:     14.82MB
```
### axum
```text
Running 10s test @ http://127.0.0.1:8083/user
  4 threads and 128 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.34ms    1.23ms  38.24ms   96.32%
    Req/Sec    23.16k     3.95k   40.24k    76.25%
  Latency Distribution
     50%    1.19ms
     75%    1.40ms
     90%    1.85ms
     99%    5.04ms
  922803 requests in 10.04s, 144.33MB read
Requests/sec:  91944.52
Transfer/sec:     14.38MB
```
### poem
```text
Running 10s test @ http://127.0.0.1:8084/user
  4 threads and 128 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.22ms  518.52us  19.03ms   79.83%
    Req/Sec    23.83k     2.81k   34.70k    74.75%
  Latency Distribution
     50%    1.16ms
     75%    1.38ms
     90%    1.77ms
     99%    3.03ms
  948738 requests in 10.02s, 148.39MB read
Requests/sec:  94710.41
Transfer/sec:     14.81MB
```
### net_http
```text
Running 10s test @ http://127.0.0.1:8091/user
  4 threads and 128 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.96ms    2.48ms  47.20ms   88.73%
    Req/Sec    20.62k     7.42k   36.78k    64.25%
  Latency Distribution
     50%    1.06ms
     75%    2.43ms
     90%    4.74ms
     99%   10.84ms
  824123 requests in 10.07s, 129.68MB read
Requests/sec:  81873.23
Transfer/sec:     12.88MB
```
