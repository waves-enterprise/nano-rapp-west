import axolotl_curve25519 as curve
import base58
import pywaves as pw

from binascii import hexlify, unhexlify

from client import *

def test_sign():
    pub_key, address = get_pubkey("V")

    p = str(hexlify(pub_key))[2:-1]

    raw_tx = "0402" + p + "00000000017410b402480000000005f5e10000000000000f4240015640b2ca70820baa3b850bf743ec6c52c79de228e3ff05fb95003502b1da5efa1ed189c4f5c21e17256e2de99186b42cb47d3f7d3cb73201586de784ebf6fa269a7f2268ccce5abf45b6040478ec1f36"

    signature = sign(raw_tx)

    assert len(pub_key) == 32
    assert len(signature) == 64

    result = curve.verifySignature(bytes(pub_key), unhexlify(raw_tx), bytes(signature))

    assert result == 0

def test_pubkey():
    pub_key, address = get_pubkey("V")

    assert len(pub_key) == 32
    assert len(address) == 35

    pw.setChain("WEST", "V")
    p = pw.Address(publicKey = base58.b58encode(bytes(pub_key)))

    assert p.address == bytes(address).decode()

def test_app_version():
    major, minor, patch = get_app_version()
    assert major == 0 and minor == 2 and patch == 0

def test_app_name():
    app_name = get_app_name()
    assert app_name == "Waves Enterprise"

# RUN
try:
    test_sign()

    test_pubkey()

    test_app_version()

    test_app_name()
except AssertionError:
    print("The tests ended with an error!")
finally:
    # Exit app
    exchange_raw("80FF")
