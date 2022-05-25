## web framework benchamark

Benchmark to get the performance of http frameworks.

They are also very simple examples to learn.


## benchmarks

### frameworks

actix-web: [github](https://github.com/actix/actix-web)

axum: [github](https://github.com/tokio-rs/axum)

warp: [github](https://github.com/seanmonstar/warp)

poem: [github](https://hub.fastgit.org/poem-web/poem)

net/http(go): [github](https://github.com/golang/go)

### binary size

we can run "strip" to get smaller binary size for rust apps.

| framework | size(release) | strip   |
| --------- | ------------- | ------- |
| actix-web | 4.6  MB       | 3.3  MB |
| axum      | 4    MB       | 3  MB   |
| warp      | 2.8  MB       | 2.1  MB |
| poem      | 2.9  MB       | 2.2  MB |
| net/http  | 6.6  MB       | N/A     |

- Result

Smaller is better.

```text
# macosx (file size)
net/http(go) > actix(rust) > axum(rust) > warp(rust)
```

### memory

| framework | real memory size | private memory size |
| --------- | ---------------- | ------------------- |
| actix-web | 8.1  MB          | 6.5  MB             |
| axum      | 5.8  MB          | 4.2  MB             |
| warp      | 5.1  MB          | 3.8  MB             |
| poem      | 5.1  MB          | 3.8  MB             |
| net/http  | 15.6 MB          | 10.3 MB             |

- Result

Smaller is better.

```text
# macosx (memory size)
net/http(go) > actix(rust) > axum(rust) >=< warp(rust)
```

### performance

- Benchmark tool ([wrk](https://github.com/wg/wrk))

```txt
brew install wrk
apt get wrk 

# no wrk package in ubuntu 20.14, have to build myself or use rewrk instead
```

```txt
wrk --latency -t4 -c128 -d10s http://127.0.0.1:8081/user
wrk --latency -t4 -c128 -d10s http://127.0.0.1:8082/user
wrk --latency -t4 -c128 -d10s http://127.0.0.1:8083/user
wrk --latency -t4 -c128 -d10s http://127.0.0.1:8084/user
wrk --latency -t4 -c128 -d10s http://127.0.0.1:8091/user
```

| framework | Transfer/sec | Requests/sec |
| --------- | ------------ | -------------|
| actix-web | 14.70 MB     | 93962.68     |
| axum      | 15.94 MB     | 101911.63    |
| warp      | 17.05 MB     | 109000.33    |
| net/http  | 13.28 MB     | 84373.58     |

- Result

Larger requests/sec is better.

```
# macosx (requests/sec)
warp(rust) >=< axum(rust) > actix(rust) > net/http(go)
```

## Todo

- Add hyper(rust), deno, express(node.js) into benchmark

- Add build time


## refs

- [Which programming language is fastest?](https://benchmarksgame-team.pages.debian.net/benchmarksgame/index.html)

- [Techempower web framework benchmarks](https://www.techempower.com/benchmarks/)
