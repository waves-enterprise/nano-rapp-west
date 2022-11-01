from ledgerblue.commTCP import getDongle as getDongleTCP
from ledgerblue.comm import getDongle

from random import getrandbits as rnd
from binascii import hexlify, unhexlify

rand_msg = hexlify(rnd(256).to_bytes(32, 'big')).decode()

CMDS = [
    "8002",
    "8003000020" + "00112233445566778899aabbccddeeff0123456789abcdeffedcba9876543210",
    "800300008d" + "04021eb35f3d4b526bb39d9ad5a081cfcd4b992535806cf4918886912b112e41c80801010000017410b402480000000005f5e10000000000000f4240015640b2ca70820baa3b850bf743ec6c52c79de228e3ff05fb95003502b1da5efa1ed189c4f5c21e17256e2de99186b42cb47d3f7d3cb73201586de784ebf6fa269a7f2268ccce5abf45b6040478ec1f36",
    "80032000" + rand_msg,
    "8004",
    "80050008",
    "80FE",
    "80FF",
]

d = getDongleTCP(port=9999)     # Speculos
# d = getDongle()               # Nano

from time import sleep
for cmd in map(unhexlify,CMDS):
    r = None 
    try:
        r = d.exchange(cmd, 20)
        sleep(1)
    except Exception as e:
        print(e)
    if r is not None: 
        print("Response : ", hexlify(r))
