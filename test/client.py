import struct

from ledgerblue.commTCP import getDongle as getDongleTCP
from ledgerblue.comm import getDongle

from binascii import hexlify, unhexlify

from time import sleep

d = getDongleTCP(port=9999)     # Speculos
# d = getDongle()               # Nano

def exchange_raw(ins):
    response = None
    cmd = unhexlify(ins)
    try:
        response = d.exchange(cmd, 20)
        sleep(1)
    except Exception as e:
        print(e)
    if response is not None:
        return response

def sign(raw_tx):
    """Sends APDU Sign instructions

    Returns
    -------
    bytes
        Returns signature bytes
    """
    length = format(int(len(raw_tx) / 2), "x")
    signature = exchange_raw("80020000" + length + raw_tx)

    return signature

def get_pubkey(chain_id):
    """Sends APDU GetPubkey instructions

    Returns
    -------
    bytes, bytes
        Returns pub_key and address bytes
    """
    chain_id_hex = hexlify(chain_id.encode("ascii"))

    response = exchange_raw("800400" + str(chain_id_hex)[2:-1])

    pub_key: bytes = response[0:32]
    address: bytes = response[32:67]

    return pub_key, address

def get_app_version():
    """Sends APDU GetVersion instructions

    Returns
    -------
    int, int, int
        Returns major, minor, patch version of the application
    """
    response = exchange_raw("8006")

    # response = MAJOR (1) || MINOR (1) || PATCH (1)
    assert len(response) == 3

    major, minor, patch = struct.unpack(
        "BBB",
        response
    )  # type: int, int, int

    return major, minor, patch

def get_app_name():
    """Sends APDU GetName instructions

    Returns
    -------
    string
        Returns the name of the application
    """
    response = exchange_raw("8008")
    return response.decode("ascii")
