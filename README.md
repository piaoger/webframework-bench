## web framework benchamark

Benchmark to get the performance of http frameworks

## benchmarks


### binary size

we can run "strip" to get smaller binary size for rust apps.


```text
actix-bench:
   4.6 MB
   3.3 MB

warp-bench:
   2.7 MB
   2 MB

net-http-bench:
   6.6MB
   N/A
```

- Result

```text
net/http(go) > actix(rust) > warp(rust)
```

### performance

- Benchmark tool ([wrk](https://github.com/wg/wrk))


```txt
brew install wrk
apt get wrk
```

```txt
wrk --latency -t4 -c200 -d8s http://127.0.0.1:8081/user
wrk --latency -t4 -c200 -d8s http://127.0.0.1:8082/user
wrk --latency -t4 -c200 -d8s http://127.0.0.1:8083/user
```


- Result

```
# macosx
warp(rust) > actix(rust) > net/http(go)
```

- Todo

Add hyper(rust), deno, express(node.js) into benchmark

## refs

- ["Which programming language is fastest?"](https://benchmarksgame-team.pages.debian.net/benchmarksgame/index.html)