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

#### Encoding Performance

```
$ python -m timeit -n 300 -u msec  -s'import benchmark' 'benchmark.bench_pyrlp_encoding()'
300 loops, best of 5: 0.0267 msec per loop

$ python -m timeit -n 300 -u msec  -s'import benchmark' 'benchmark.bench_rustyrlp_encoding()'
300 loops, best of 5: 0.00374 msec per loop

```

These numbers ☝️ suggest that `rusty_rlp` encodes roughly 7.5 faster compared to pyrlp.


### Decoding Performance

```
$ python -m timeit -n 300 -u msec  -s'import benchmark' 'benchmark.bench_pyrlp_decoding()'
300 loops, best of 5: 0.0241 msec per loop

$ python -m timeit -n 300 -u msec  -s'import benchmark' 'benchmark.bench_rustyrlp_decoding()'
300 loops, best of 5: 0.011 msec per loop

```

These numbers ☝️ suggest that `rusty_rlp` decodes roughly 2.2 faster compared to pyrlp.

### Roundtrip Performance

```
$ python -m timeit -n 300 -u msec  -s'import benchmark' 'benchmark.bench_pyrlp_roundtrip()'
300 loops, best of 5: 2.08 msec per loop

$ python -m timeit -n 300 -u msec  -s'import benchmark' 'benchmark.bench_rustyrlp_roundtrip()'
300 loops, best of 5: 1.09 msec per loop

```

These numbers ☝️ suggest that the rountrip performance of `rusty_rlp` is about 2x faster than pyrlp.
(Not exactly sure why it isn't higher given the results for encoding/decoding in isolation).


### How to run the Python tests

`python -m pytest python_tests.py `

Or ` cargo build --release && cp target/release/librusty_rlp.so target/release/rusty_rlp.so && python -m pytest python_tests.py` to compile and test in one step.

### Caveats

The code is in very bad shape and we aren't yet passing all the tests. Also `encode_raw` isn't
yet implemented.

