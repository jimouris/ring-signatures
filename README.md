<p align="center">
  <img src="images/logo.png" alt="Ring Signatures Logo" width="150"/>
</p>

# Ring Signatures <a href="https://github.com/jimouris/ring-signatures/blob/main/LICENSE" target="_blank"><img src="https://img.shields.io/badge/License-GPLv3-blue.svg?logo=gnu" alt="GPLv3 License"/></a>

A ring signature allows a member of a group to sign a message on behalf of the group, without revealing who the actual signer is.
It is used in privacy-focused systems like [Monero](https://www.getmonero.org/) to hide the sender among a group of decoys ([Monero (RingCT)](https://www.getmonero.org/resources/moneropedia/ringCT.html)).


> [!NOTE]
> Implementation of [ring signatures](https://en.wikipedia.org/wiki/Ring_signature) with the [curve25519-dalek](https://docs.rs/curve25519-dalek/latest/curve25519_dalek/index.html) crate using the Ristretto group ([Why Ristretto?](https://ristretto.group/why_ristretto.html)).

# Usage

```shell
Options:
  -n, --n <N>              Ring size [default: 5]
  -s, --signer <SIGNER>    Index of signer [default: 2]
  -m, --message <MESSAGE>  Message to sign [default: "Ring signatures are cool."]
```

For example,
```shell
cargo run -r -- -n 10 -s 2 -m "Ring signatures are cool."
```
outputs:
```shell
Number of Signers : 10
Signer Index      : 2
Message           : Ring signatures are cool.

Signature:
  c0 = 99b46fb00ae67a7e12b9cc5097c22db6bb855aa23d0182742fd90a84538afc09
  s[0] = 15d5ba651ff5757af80ce986c6041ac31366c841127bc508fc8a78438756f700
  s[1] = d198821b087dff7d9099395cdd531a65b3862ec62161418fbc4965ca9ab3740b
  s[2] = c6de14551480fef1c91330beffce60e8456035ac6a6fa5b7c61993daa2e8990b
  s[3] = e8ef01811d9237f7bbcae0bc2c1270b8b916a48fd07d9dc152438f68df934b01
  s[4] = c60387b3b840631460a56dc9a2bf4ae93c70f9c439033b5e7567b08231cd7b02
  s[5] = 294ed77e893b2146aa6d6e28c2d6ed9a4451647045982404ef16a9c20e9c910f
  s[6] = 65763d4500b4d6a83f3d3e978b58e2139078f5a182dba48ca36c05c04caf8f0b
  s[7] = e0572ac14dce8a50c1a2fa33f48bc9bbe1a0e0be99bcab4e5ca6925d3a973705
  s[8] = 559603b71a6036a49e25f0a00323d92ce8c55cde176295a6bb947e141575320c
  s[9] = 948038559c6d4ceebff4e15e1cba5f94a879fcfe8fbdc6f2a4dce410f5414f0c

Valid signature: true
```
