# Benchmark on generating address

Here's the result of benchmarking generate address on both iota.rs and iota.c. To generate a address it will do several hashing and type conversion. It's a ideal metrics to see how much time it will be consumed in both language. Both libraries being tested are compiled with release build with AVX2 optimization option enabled.

## x86_64 with AVX2

![](https://i.imgur.com/NthFHxT.png)
![](https://i.imgur.com/LuMfyCC.png)

## armv7 (raspberry pi)
And here are the results on raspberry pi 4:

![](https://i.imgur.com/ht5X01p.png)
![](https://i.imgur.com/Fxwicfh.png)

As we can see that both Rust and C are almost the same which means they can produce same binary with very little overhead. It leaves developers to make further optimization and fine tuning.