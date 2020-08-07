import timeit
import random
from typing import Sequence, Tuple

import rlp
from eth_utils import encode_hex, decode_hex
from rlp import encode, decode
from rlp.sedes import raw, big_endian_int

from target.release import rusty_rlp

UPPER_BOUND = 18_446_744_073_709_551_615

SAMPLE = (1, 2, 3, 4)


def get_decoded_samples(sample_count: int) -> Sequence[Tuple[int, int, int, int]]:
    for _ in range(sample_count):
        yield (
            random.randint(0, UPPER_BOUND),
            random.randint(0, UPPER_BOUND),
            random.randint(0, UPPER_BOUND),
            random.randint(0, UPPER_BOUND)
        )


def check_correctness():
    for sample in get_decoded_samples(100):
        pyrlp_bytes = encode(sample)
        rustyrlp_bytes = rusty_rlp.encode_fictive_type(sample)
        # FIXME: The return types aren't yet fully aligned. We can make rusty_rlp to return Python bytes
        assert bytes(rustyrlp_bytes) == pyrlp_bytes

        pyrlp_decoded = decode(pyrlp_bytes)
        rustyrlp_decoded = rusty_rlp.decode_raw(pyrlp_bytes)

        assert pyrlp_decoded == rustyrlp_decoded


def bench_pyrlp_roundtrip():
    for sample in get_decoded_samples(100):
        rlp_bytes = encode(sample)
        decoded = decode(rlp_bytes)


def bench_rustyrlp_roundtrip():
    for sample in get_decoded_samples(100):
        rlp_bytes = rusty_rlp.encode_fictive_type(sample)
        decoded = rusty_rlp.decode_raw(rlp_bytes)
