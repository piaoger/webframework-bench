## BENCHMARK REPORT
### warp
```text
Running 10s test @ http://127.0.0.1:8081/user
  4 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   415.37us  238.25us  14.28ms   91.22%
    Req/Sec    76.69k     7.25k   85.91k    68.00%
  Latency Distribution
     50%  363.00us
     75%  541.00us
     90%  626.00us
     99%    0.91ms
  3052490 requests in 10.07s, 457.04MB read
Requests/sec: 303121.78
Transfer/sec:     45.39MB
```
### actix
```text
Running 10s test @ http://127.0.0.1:8082/user
  4 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   692.99us    1.71ms  73.90ms   99.54%
    Req/Sec    42.50k     4.54k   53.19k    74.00%
  Latency Distribution
     50%  581.00us
     75%  673.00us
     90%  834.00us
     99%    1.41ms
  1691341 requests in 10.06s, 253.24MB read
Requests/sec: 168137.92
Transfer/sec:     25.17MB
```
### net_http
```text
Running 10s test @ http://127.0.0.1:8083/user
  4 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.61ms    3.41ms  91.59ms   86.04%
    Req/Sec    23.47k     1.43k   32.47k    88.00%
  Latency Distribution
     50%    0.88ms
     75%    3.69ms
     90%    7.24ms
     99%   12.64ms
  934876 requests in 10.09s, 139.98MB read
Requests/sec:  92651.26
Transfer/sec:     13.87MB
```
