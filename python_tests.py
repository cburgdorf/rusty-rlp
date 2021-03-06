import pytest
from rlp import decode
from rlp.codec import encode_raw
import rusty_rlp

from eth_utils import decode_hex


@pytest.mark.parametrize(
    'input',
    (
        b'',
        b'asdf',
        b'fds89032#$@%',
        b'dfsa',
        [b'dfsa', b''],
        [],
        [b'fdsa', [b'dfs', [b'jfdkl']]],
        # https://etherscan.io/block/400000
        [b'\x1ew\xd8\xf1&sH\xb5\x16\xeb\xc4\xf4\xda\x1e*\xa5\x9f\x85\xf0\xcb\xd8S\x94\x95\x00\xff\xac\x8b\xfc8\xba\x14', b'\x1d\xccM\xe8\xde\xc7]z\xab\x85\xb5g\xb6\xcc\xd4\x1a\xd3\x12E\x1b\x94\x8at\x13\xf0\xa1B\xfd@\xd4\x93G', b'*e\xac\xa4\xd5\xfc[\\\x85\x90\x90\xa6\xc3M\x16A59\x82&', b'\x0b^C\x86h\x0fC\xc2$\xc5\xc07\xef\xc0\xb6E\xc8\xe1\xc3\xf6\xb3\r\xa0\xee\xc0rr\xb4\xe6\xf8\xcd\x89', b'V\xe8\x1f\x17\x1b\xccU\xa6\xff\x83E\xe6\x92\xc0\xf8n[H\xe0\x1b\x99l\xad\xc0\x01b/\xb5\xe3c\xb4!', b'V\xe8\x1f\x17\x1b\xccU\xa6\xff\x83E\xe6\x92\xc0\xf8n[H\xe0\x1b\x99l\xad\xc0\x01b/\xb5\xe3c\xb4!', b'\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00', b'\x05zA\x8a|>', b'\x06\x1a\x80', b'/\xef\xd8', b'', b'V"\xef\xdc', b'\xd5\x83\x01\x02\x02\x84Geth\x85go1.5\x85linux', b'?\xbe\xa7\xafd*N \xcd\x93\xa9E\xa1\xf5\xe2;\xd7/\xc5&\x11S\xe0\x91\x02\xcfq\x89\x80\xae\xff8', b'j\xf2<\xaa\xe9V\x92\xef'],
    )
)
def test_decode_raw(input):
    pyrlp_encoded = encode_raw(input)
    rustyrlp_encoded = rusty_rlp.encode_raw(input)

    assert pyrlp_encoded == rustyrlp_encoded

    pyrlp_decoded = decode(pyrlp_encoded)
    rustyrlp_decoded, _ = rusty_rlp.decode_raw(rustyrlp_encoded, True, False)

    assert pyrlp_decoded == rustyrlp_decoded == input

@pytest.mark.parametrize(
    'input, expected',
    (
        ((b'fdsa', (b'dfs', [b'jfdkl']),), [b'fdsa', [b'dfs', [b'jfdkl']]],),
    )
)
def test_decode_tuple_as_list(input, expected):
    pyrlp_encoded = encode_raw(input)
    rustyrlp_encoded = rusty_rlp.encode_raw(input)

    assert pyrlp_encoded == rustyrlp_encoded

    pyrlp_decoded = decode(pyrlp_encoded)
    rustyrlp_decoded, _ = rusty_rlp.decode_raw(rustyrlp_encoded, True, False)

    assert pyrlp_decoded == rustyrlp_decoded == expected


@pytest.mark.parametrize(
    'rlp_data',
    (
        0,
        32,
        ['asdf', ['fdsa', [5]]],
        str
    ),
)
def test_invalid_serializations(rlp_data):
    with pytest.raises(rusty_rlp.EncodingError, match='Can not encode value'):
        rusty_rlp.encode_raw(rlp_data)


@pytest.mark.parametrize(
    'rlp_data, expected_error',
    (
        (None, TypeError),
        ('asdf', TypeError),
        # Empty list with trailing bytes
        (decode_hex('0xc000'), rusty_rlp.DecodingError),
        # https://github.com/ethereum/pyrlp/blob/37396698aeb949932e70a53fa10f3046b7915bf3/tests/test_codec.py#L47-L50
        (decode_hex('b8056d6f6f7365'), rusty_rlp.DecodingError),
        # trailing bytes to https://github.com/ethereum/pyrlp/blob/37396698aeb949932e70a53fa10f3046b7915bf3/tests/rlptest.json#L68
        (decode_hex('0xcc83646f6783676f648363617400'), rusty_rlp.DecodingError),
        # trailing bytes to short string
        (decode_hex('0x83646f6700'), rusty_rlp.DecodingError),
        (b'', rusty_rlp.DecodingError),
        (b'\x83do', rusty_rlp.DecodingError),
        (b'\xb8\x00', rusty_rlp.DecodingError),
        (b'\xb9\x00\x00', rusty_rlp.DecodingError),
        (b'\xba\x00\x02\xff\xff', rusty_rlp.DecodingError),
        (b'\x81\x54', rusty_rlp.DecodingError),
    ),
)
def test_invalid_deserializations(rlp_data, expected_error):
    with pytest.raises(expected_error):
        rusty_rlp.decode_raw(rlp_data, True, False)


@pytest.mark.parametrize(
    'rlp_data, expected',
    (
        # The following case was taken from Py-EVM. Since it wasn't caught by the Py-RLP test suite, it
        # is valuable to have here.
        (decode_hex('0xf8518080808080a08591cad10d1692b94ac37d41f0834d4e350350926babfca8793c885bc846aa478080808080808080a0ed3e6bc5f6af82aec3a3d9ba1f06af4854631201347e2f6f2a5da66c7117355d8080'), [b'', b'', b'', b'', b'', b'\x85\x91\xca\xd1\r\x16\x92\xb9J\xc3}A\xf0\x83MN5\x03P\x92k\xab\xfc\xa8y<\x88[\xc8F\xaaG', b'', b'', b'', b'', b'', b'', b'', b'', b'\xed>k\xc5\xf6\xaf\x82\xae\xc3\xa3\xd9\xba\x1f\x06\xafHTc\x12\x014~/o*]\xa6lq\x175]', b'', b'']),
        (decode_hex('0xc0'), []),
        (decode_hex('0xcc83646f6783676f6483636174'), [ b"dog", b"god", b"cat" ]),
        (decode_hex('0xc6827a77c10401'), [b'zw', [b'\x04'], b'\x01']),
    ),
)
def test_decode_special_cases(rlp_data, expected):
    decoded, _ = rusty_rlp.decode_raw(rlp_data, True, False)
    assert decoded == expected


@pytest.mark.parametrize(
    'rlp_data, expected',
    (
        # Taken from a failing Trinity test: https://app.circleci.com/pipelines/github/ethereum/trinity/7246/workflows/73f3758f-6edf-4c1d-a48a-8e192ac9d0e0/jobs/276808
        (
            b'\xf8\xa7\xb8A)\x9c\xa6\xac\xfd5\xe3\xd7-\x8b\xa3\xd1\xe2\xb6\x0bUa\xd5\xafR\x18\xeb[\xc1\x82\x04Wi\xebB&\x91\n0\x1a\xca\xe3\xb3i\xff\xfcJH\x99\xd6\xb0%1\xe8\x9f\xd4\xfe6\xa2\xcf\r\x93`{\xa4p\xb5\x0fx\x00\xb8@\xfd\xa1\xcf\xf6t\xc9\x0c\x9a\x19u9\xfe=\xfbS\x08j\xced\xf8>\xd7\xc6\xea\xbe\xc7A\xf7\xf3\x81\xcc\x80>R\xab,\xd5]Ui\xbc\xe44q\x07\xa3\x10\xdf\xd5\xf8\x8a\x01\x0c\xd2\xff\xd1\x00\\\xa4\x06\xf1\x84(w\xa0~\x96\x8b\xba\x13\xb6\xc5\x0e,L\xd7\xf2A\xcc\rd\xd1\xac%\xc7\xf5\x95-\xf21\xacj+\xda\x8e\xe5\xd6\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00',
            [
                b')\x9c\xa6\xac\xfd5\xe3\xd7-\x8b\xa3\xd1\xe2\xb6\x0bUa\xd5\xafR\x18\xeb[\xc1'
                b'\x82\x04Wi\xebB&\x91\n0\x1a\xca\xe3\xb3i\xff\xfcJH\x99\xd6\xb0%1'
                b'\xe8\x9f\xd4\xfe6\xa2\xcf\r\x93`{\xa4p\xb5\x0fx\x00',
                b'\xfd\xa1\xcf\xf6t\xc9\x0c\x9a\x19u9\xfe=\xfbS\x08j\xced\xf8>\xd7\xc6\xea'
                b'\xbe\xc7A\xf7\xf3\x81\xcc\x80>R\xab,\xd5]Ui\xbc\xe44q\x07\xa3\x10\xdf'
                b'\xd5\xf8\x8a\x01\x0c\xd2\xff\xd1\x00\\\xa4\x06\xf1\x84(w',
                b'~\x96\x8b\xba\x13\xb6\xc5\x0e,L\xd7\xf2A\xcc\rd\xd1\xac%\xc7\xf5\x95-\xf2'
                b'1\xacj+\xda\x8e\xe5\xd6',
                b'\x04',
            ]
        ),
        # Trailing bytes to empty list
        (decode_hex('0xc000'), []),
        #trailing bytes to https://github.com/ethereum/pyrlp/blob/37396698aeb949932e70a53fa10f3046b7915bf3/tests/rlptest.json#L68
        (decode_hex('0xcc83646f6783676f648363617400'), [b'dog', b'god', b'cat']),
        # trailing bytes to short string
        (decode_hex('0x83646f6700'), b'dog'),
    ),
)
def test_nonstrict_deserializations(rlp_data, expected):
    decoded, _ = rusty_rlp.decode_raw(rlp_data, False, False)
    assert decoded == expected


@pytest.mark.parametrize(
    'rlp_data, expected, expected_per_item_rlp',
    (
        (
            b'\xcc\xc5\x05a\xc2\x80\x80\xc5\x05a\xc2\x80\x80',
            [[b'\x05', b'a', [b'', b'']], [b'\x05', b'a', [b'', b'']]],
            [b'\xcc\xc5\x05a\xc2\x80\x80\xc5\x05a\xc2\x80\x80', [b'\xc5\x05a\xc2\x80\x80', [b'\x05'], [b'a'], [b'\xc2\x80\x80', [b'\x80'], [b'\x80']]], [b'\xc5\x05a\xc2\x80\x80', [b'\x05'], [b'a'], [b'\xc2\x80\x80', [b'\x80'], [b'\x80']]]]
        ),
    ),
)
def test_preserving_api(rlp_data, expected, expected_per_item_rlp):
    decoded, per_item_rlp = rusty_rlp.decode_raw(rlp_data, strict=True, preserve_cache_info=True)
    assert decoded == expected
    assert per_item_rlp == expected_per_item_rlp

    decoded, per_item_rlp = rusty_rlp.decode_raw(rlp_data, strict=True, preserve_cache_info=False)
    assert decoded == expected
    assert per_item_rlp == []
