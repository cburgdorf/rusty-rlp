# Rusty-RLP

Rapid fast RLP serialization / deserialization for Python.

As the name suggest, `rusty-rlp` is written in Rust and based on the battle-tested, MIT-licensed
`rlp` implementation that is also used by [OpenEthereum](https://github.com/openethereum/openethereum).

## Usage

```python
from rusty_rlp import decode_raw, encode_raw

encoded = encode_raw(b'some_string')
decoded = decode_raw(encoded)
```

## Running the tests

The library exposes a pure Python API and all tests are implemented in [`python_tests.py`](https://github.com/cburgdorf/rusty-rlp/blob/master/python_tests.py)

Run the tests:

```
make test
```

**Note:** There do exists some tests implemented in Rust that can be run with `cargo tests` but those do not test *this* library but the underlying `rlp` library instead. They exist merely to prove some assumptions about the underlying library.


## Benchmarks

We provide some benchmarks against [`pyrlp`](https://github.com/ethereum/pyrlp).

|           |    Encoding (msec/loop) |  Decoding (msec/loop) |
|-----------|:-----------------------:|----------------------:|
| PyRLP     |  0.016                  |   0.014               |
| rusty-rlp |  0.00173 (~9x)          |   0.00595 (~2.35x)    |


### Running the benchmarks

```
make benchmark
```
