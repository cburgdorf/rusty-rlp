import random
from typing import Sequence, Tuple

from rlp import decode
from rlp.codec import encode_raw

from target.release import rusty_rlp


ENCODED_SAMPLES = [
    b'\xf8=\x88l\x07\xea\xd9N\xca\x84V\xa0\x1b/\x08\xf3l\xea\xe1/\x00`\xecs}\xf0\x19\xafgP\xec\x92\xbb\x9c\xa9\xa0l\x1f$\xca\xcc\x13\xeb\x81\x81\xcf\x90\xc5\x132\xd7\x8d\x8e\xf3J\xa3\x7fSUqc\xf7\x1c',
    b'\xf8<\x88\xe6\x89\\\xf6\x17bQ\x95\xa0\x97A\x1a\xb30\x9b\x9b#\xea\xb9 \xe1B\xae\xdb\xd82\xe6\xf3\x1b\x06\x8e\x94F\x85\x92%\x83`\xfdq\x1e&\x90l\x8eC\xb2\xf6&\xee\x1ca\xe1H\xfb\x8c\xdf%Q',
    b'\xf8<\x88P\x18MWL\x8fU\x0c\xa0\x80f\xd3\xf9%\xd1\xae.K\xabI\x94\xcfe\xfdV\x06\xed\xc7[\x13\x7fS<\xe1~\n\xaf\xfb\xa2\xe3\x9ej\x90"\x9eUr\'&\xd1\xe4\xd2\x18\xff\xe3:\xaa B',
    b'\xf8=\x88\xde\x9e\x98\xc1M\xbf\x91\xe8\xa0\xb2G\x8d\\P\x0b\x81\x10\xfe\xdb\xba\xdaX~\x05o\xf9\xc4\xec\xc7\xd6f\x81Lm\x0b\x94\xc91mM.\x81\xf0\x90\x10pq\xf6br\x03\xb2`\xd0\x8bZ\xa1\x16\rC',
]


DECODED_SAMPLES = [
    [b'l\x07\xea\xd9N\xca\x84V', b'\x1b/\x08\xf3l\xea\xe1/\x00`\xecs}\xf0\x19\xafgP\xec\x92\xbb\x9c\xa9\xa0l\x1f$\xca\xcc\x13\xeb\x81', b'\xcf', b'\xc5\x132\xd7\x8d\x8e\xf3J\xa3\x7fSUqc\xf7\x1c'],
    [b'\xe6\x89\\\xf6\x17bQ\x95', b'\x97A\x1a\xb30\x9b\x9b#\xea\xb9 \xe1B\xae\xdb\xd82\xe6\xf3\x1b\x06\x8e\x94F\x85\x92%\x83`\xfdq\x1e', b'&', b'l\x8eC\xb2\xf6&\xee\x1ca\xe1H\xfb\x8c\xdf%Q'],
    [b'P\x18MWL\x8fU\x0c', b'\x80f\xd3\xf9%\xd1\xae.K\xabI\x94\xcfe\xfdV\x06\xed\xc7[\x13\x7fS<\xe1~\n\xaf\xfb\xa2\xe3\x9e', b'j', b'"\x9eUr\'&\xd1\xe4\xd2\x18\xff\xe3:\xaa B'],
    [b'\xde\x9e\x98\xc1M\xbf\x91\xe8', b'\xb2G\x8d\\P\x0b\x81\x10\xfe\xdb\xba\xdaX~\x05o\xf9\xc4\xec\xc7\xd6f\x81Lm\x0b\x94\xc91mM.', b'\xf0', b'\x10pq\xf6br\x03\xb2`\xd0\x8bZ\xa1\x16\rC'],
]

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
        rustyrlp_decoded = rusty_rlp.decode_raw(pyrlp_bytes, True)

        assert pyrlp_decoded == rustyrlp_decoded


def bench_pyrlp_roundtrip():
    for sample in get_decoded_samples(100):
        rlp_bytes = encode_raw(sample)
        decoded = decode(rlp_bytes)
        assert decoded == sample


def bench_rustyrlp_roundtrip():
    for sample in get_decoded_samples(100):
        rlp_bytes = rusty_rlp.encode_raw(sample)
        decoded, _ = rusty_rlp.decode_raw(rlp_bytes, True, False)
        assert decoded == sample


def bench_pyrlp_decoding():
    for index, sample in enumerate(ENCODED_SAMPLES):
        decoded = decode(sample)
        assert decoded == DECODED_SAMPLES[index]

def bench_rustyrlp_decoding():
    for index, sample in enumerate(ENCODED_SAMPLES):
        decoded, _ = rusty_rlp.decode_raw(sample, True, False)
        assert decoded == DECODED_SAMPLES[index]


def bench_pyrlp_encoding():
    for index, sample in enumerate(DECODED_SAMPLES):
        encoded = encode_raw(sample)
        assert encoded == ENCODED_SAMPLES[index]

def bench_rustyrlp_encoding():
    for index, sample in enumerate(DECODED_SAMPLES):
        encoded = rusty_rlp.encode_raw(sample)
        assert encoded == ENCODED_SAMPLES[index]