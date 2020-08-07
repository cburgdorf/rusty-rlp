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

### How to run the Python tests

`python -m pytest python_tests.py `

Or ` cargo build --release && cp target/release/librusty_rlp.so target/release/rusty_rlp.so && python -m pytest python_tests.py` to compile and test in one step.

### Caveats

The code is in very bad shape and we aren't yet passing all the tests. Also `encode_raw` isn't
yet implemented.

