import struct
import threading

from ledgerblue.commTCP import getDongle as getDongleTCP
from ledgerblue.comm import getDongle

from binascii import hexlify, unhexlify

from time import sleep

from button import *
from utils import expand_path, path_to_bytes

d = getDongleTCP(port=9999)     # Speculos
# d = getDongle()               # Nano

# button_client = ButtonTCP(server="127.0.0.1", port=9999)
button_client = ButtonAPI(server="127.0.0.1", port=5000)

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

def sign_first_chunk(raw_tx, bip32_path):
    """Sends APDU Sign instructions
    """

    bip32_bytes = str(hexlify(path_to_bytes(expand_path(bip32_path))))[2:-1]

    raw_tx = bip32_bytes + "0808" + raw_tx

    length = get_data_length(raw_tx)

    exchange_raw("80020000" + length + raw_tx)

def sign_next_chunk(raw_tx):
    """Sends APDU Sign instructions
    """
    length = get_data_length(raw_tx)

    exchange_raw("80020000" + length + raw_tx)

def sign_last_chunk(raw_tx):
    """Sends APDU Sign instructions

    Returns
    -------
    bytes
        Returns signature bytes
    """
    length = get_data_length(raw_tx)

    result = []

    def run():
      result.append(exchange_raw("80028000" + length + raw_tx))

    th = threading.Thread(target=run, args=())
    th.start()

    # Review
    button_client.right_click()
    sleep(1)
    # Amount
    button_client.right_click()
    sleep(1)
    # Asset
    button_client.right_click()
    sleep(1)
    # Fee
    button_client.right_click()
    sleep(1)
    # Fee Asset
    button_client.right_click()
    sleep(1)
    # Accept
    button_client.both_click()
    sleep(1)

    th.join()

    signature = result[0]

    return signature

def get_pubkey(chain_id, bip32_path):
    """Sends APDU GetPubkey instructions

    Returns
    -------
    bytes, bytes
        Returns pub_key and address bytes
    """
    chain_id_hex = hexlify(chain_id.encode("ascii"))

    bip32_bytes = str(hexlify(path_to_bytes(expand_path(bip32_path))))[2:-1]

    length = get_data_length(bip32_bytes)

    response = exchange_raw("800400" + str(chain_id_hex)[2:-1] + length + bip32_bytes)

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

def get_data_length(data):
    return format(int(len(data) / 2), "x")
