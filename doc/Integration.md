# Introduction

The integration of the application consists of several parts: the development of an application for the device and the protocol of interaction (check this repo) and the integration of the communication library, implementation of the protocol of interaction with the device in the final application.

# Ledger Application

The source code for the application is contained in this repository. The build and install instructions is [here](https://github.com/waves-enterprise/nano-rapp-west#building).

# Cryptography protocol

Waves Enterprise uses ED25519 signature with X25519 keys (Montgomery form), but Ledger (like most of integrated cryptography devices) don't support X25519 keys. But there're the libraries with conversion functions from ED25519 keys to X25519 (Curve25519) `crypto_sign_ed25519_pk_to_curve25519(curve25519_pk, ed25519_pk)` for public key and
`crypto_sign_ed25519_sk_to_curve25519(curve25519_sk, ed25519_skpk)` for private key:

* [C](https://download.libsodium.org/doc/advanced/ed25519-curve25519.html)
* [JS](https://github.com/dchest/ed2curve-js)
* [Java](https://github.com/muquit/libsodium-jna) 
* [Other languages Libsodium bindings](https://download.libsodium.org/doc/bindings_for_other_languages/)

I use the ED25519 keys and the signature inside the Ledger application, then you need to convert the keys from the device to X25519 format using that function on the client-side. Looks like the ED25519 algo from Ledger SDK already install 'sign' bit into the signature from the public key, so no additional conversation for signature are needed ([unlike the signature of libsodium](https://gist.github.com/Tolsi/d64fcb09db4ead75e5eeeab445284c93)).

# Ledger app protocol

Service statuses are used from [ledger-nanos-sdk](https://github.com/LedgerHQ/ledger-nanos-sdk/blob/master/src/io.rs#L12-L20).

## bip32 path

bip32 path bytes are the bytes of 5 int values. Waves Enterprise bip32 path prefix is [`5741564' = 0x80579bfc`](https://github.com/satoshilabs/slips/blob/master/slip-0044.md), so the bip32 path of first used address on the device is `44'/5741564'/0'/0'/1'`. In bytes this is `0x8000002c80579bfc800000008000000080000001`.

## Getting a public key

hex message bytes

`80 04 01 56 14 8000002c80579bfc800000008000000080000001`

80 [command: 1 byte: 04 hex] [ask user confirmation: 1 byte: 01 hex] [chain id: 1 byte: 00 hex] [payload size: 1 byte: 14 hex: 20 bytes] [bip32 path bytes: 20 bytes: ...]

Chain ID for mainnet is 'V'.

### Answer

If request success:

`[public key bytes: 32 bytes] [base58 address bytes: 35 bytes] [service status: 2 bytes: should be 9000 = StatusWords::Ok]`

or

`StatusWords::UserCancelled` (6e02) if the user canceled the request

# Example:

Approved request:
```
HID => 80040056148000002c80579bfc800000008000000080000001
HID <= 5bf55c73b82ebe22be80f3430667af570fae2556a6415e6b30d4065300aa947d334e676f4d374e726a4c636135437954514b665269485179347771394b6278424d51529000
publicKey (base58): 7By6kV2t2d188odEM4ExAve1UithKT6dLva4dwsDT3ak
address: 3NgoM7NrjLca5CyTQKfRiHQy4wq9KbxBMQR
```

Denied request:
```
HID => 80040056148000002c80579bfc800000008000000080000001
HID <= 6e02
User denied request on Ledger Nano S device.
```

## Sign message

A big message should be chunked by 128 bytes

hex message for the first part

`80 02 00 00 7b 8000 002c...`

`80 02 [not last command message: 1 byte: 00 hex] [unused: 1 byte: 00 hex] [payload size byte: 7b hex: 123 bytes] [bip32 path bytes: 20 bytes: ...] [amount decimals: 1 byte] [fee decimals: 1 byte] [data type: 1 byte] [data version: 1 byte] [tx chunk bytes: 98 bytes]`

hex message for next parts

`80 02 [not last command message: 1 byte: 00 hex] [unused: 1 byte: 00 hex] [payload size byte: 7b hex: 123 bytes] [tx chunk bytes: 123 bytes]`

hex message for last part

`80 02 80 00 18 3ed87...`

`80 02 [last command message: 1 byte: 80 hex] [chain id: 1 byte: 00 hex] [payload size byte: 18 hex: 24 bytes] [last 24 tx bytes]`

For signing order add byte `252` as data type (and `0` for data version), for some data - `253`, for request - `254`, for a message - `255`. For different transactions set data type and version equals to tx type and version. This byte will not be signed, it tells the device what type of message it is sent to display in the user interface. Nothing needs to be transferred for transactions, the first byte of the tx body data is the transaction type and it will be signed as expected.

#### Example:

```
HID => 800200004c04025bf55c73b82ebe22be80f3430667af570fae2556a6415e6b30d4065300aa947d00000000017410b402480000000005f5e10000000000000f4240015640b2ca70820baa3b850bf743ec6c
HID <= 9000
HID => 800280004152c79de228e3ff05fb95003502b1da5efa1ed189c4f5c21e17256e2de99186b42cb47d3f7d3cb73201586de784ebf6fa269a7f2268ccce5abf45b6040478ec1f36
HID <= fe3390be1614f0dfb3945aec6106a2832853e6168cc4fe268f79f75c4ce55be610450d4d565c332cdfa31ce66f259aff6986eb950302fe5004a494028e8138009000
```

```
HID => 800200004c04025bf55c73b82ebe22be80f3430667af570fae2556a6415e6b30d4065300aa947d00000000017410b402480000000005f5e10000000000000f4240015640b2ca70820baa3b850bf743ec6c
HID <= 9000
HID => 800280004152c79de228e3ff05fb95003502b1da5efa1ed189c4f5c21e17256e2de99186b42cb47d3f7d3cb73201586de784ebf6fa269a7f2268ccce5abf45b6040478ec1f36
HID <= 6e02
User denied signing request on Ledger Nano S device.
```

## Answer

`StatusWords::Ok` (9000) after each chunk

`[signature: 64 bytes] [service status: should be 9000 = StatusWords::Ok]` after last one

or

`StatusWords::UserCancelled` (6e02) if the user canceled the request

## App version

To get the app version you need to send `8006` to the device, then it will return 3 bytes: [major version][minor version][patch version].
