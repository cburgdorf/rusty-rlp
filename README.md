### How to run the benchmark

1. Create a virtualenv
2. Install `rlp`
3. From the root directory of the library, run:

```
cargo build --release && cp target/release/librusty_rlp.so target/release/rusty_rlp.so
``` 

4. Then run any of these from the root directory of the library:

```
# Benchmark rusty_rlp
python -m timeit -n 300 -u msec  -s'import benchmark' 'benchmark.bench_rustyrlp_roundtrip()'

# Benchmark pyrlp
python -m timeit -n 300 -u msec  -s'import benchmark' 'benchmark.bench_pyrlp_roundtrip()'
```

### Results

```
$ python -m timeit -n 300 -u msec  -s'import benchmark' 'benchmark.bench_rustyrlp_roundtrip()'
300 loops, best of 5: 0.698 msec per loop

$ python -m timeit -n 300 -u msec  -s'import benchmark' 'benchmark.bench_pyrlp_roundtrip()'
300 loops, best of 5: 3.07 msec per loop
```

These numbers ☝️ suggest that `rusty_rlp` runs more than 4x faster compared to pyrlp.

### Caveats

The API isn't yet fully aligned. E.g. we aren't returning actual Python `bytes` types yet.
We return a vector of bytes instead but this can be fixed. I don't expect it to have much impact (if any) on the performance though.

