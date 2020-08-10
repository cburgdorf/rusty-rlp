import timeit
import random
from typing import Sequence, Tuple

import rlp
from eth_utils import encode_hex, decode_hex
from rlp import encode, decode
from rlp.sedes import raw, big_endian_int
from rlp.codec import encode_raw

from target.release import rusty_rlp


def get_decoded_samples(sample_count: int) -> Sequence[Tuple[bytes, bytes, bytes, bytes]]:
    for _ in range(sample_count):
        yield [
            bytes([random.getrandbits(8) for _ in range(0, 8)]),
            bytes([random.getrandbits(8) for _ in range(0, 32)]),
            bytes([random.getrandbits(8) for _ in range(0, 1)]),
            bytes([random.getrandbits(8) for _ in range(0, 16)]),
        ]


def check_correctness():
    for sample in get_decoded_samples(100):
        pyrlp_bytes = encode_raw(sample)
        rustyrlp_bytes = rusty_rlp.encode_raw(sample)

        assert pyrlp_bytes == rustyrlp_bytes

        pyrlp_decoded = decode(pyrlp_bytes)
        rustyrlp_decoded = rusty_rlp.decode_raw(pyrlp_bytes)

        assert pyrlp_decoded == rustyrlp_decoded


def bench_pyrlp_roundtrip():
    for sample in get_decoded_samples(100):
        rlp_bytes = encode_raw(sample)
        decoded = decode(rlp_bytes)
        assert decoded == sample


def bench_rustyrlp_roundtrip():
    for sample in get_decoded_samples(100):
        rlp_bytes = rusty_rlp.encode_raw(sample)
        decoded = rusty_rlp.decode_raw(rlp_bytes)
        assert decoded == sample
