# Altcha Deobfuscation

Update 12/11/2025: The vendor retroactively attempted to dispute the validity of this vulnerability, see discussions [here](https://github.com/github/advisory-database/pull/6536), my response is in short:

- The correct time to debate about the validity and scope is during the coordinated responsible disclosure process using the posted timeline and contact venue, not after the fact and attempt to muddy the waters by bypassing collaborative channels. I published my vendor notification emails [here](./emails), provided plenty of opportunity for the vendor to dispute constructively or request an extension to the CNA-LR disclosure process and received _no reply_ as of 12/11/2025.
- This is a vulnerability because there is **no proof of work**. It is apparent from the referenced web material and the referenced dispute basis that this computational effort is the primary if not sole way of achieving any practical deterrent to abuse. This nonlinear and nontrivial computational effort to decode the obfuscated data is a fundamental correctness requirement of this protection mechanism, and this PoC demonstrates the computational effort is fixed and trivial (approx. 2 AES operations) regardless of any cost factors.
- The vendor did disclose "this is not cryptographic security", but I believe any reasonable person reading the text would believe the feature SHOULD provide at least some level of additional protection that is more than trivial encoding only methods like base64, and I believe this additional protection is proven to be invalid by this issue.

---

A cryptanalytic break for deobfuscating text obfuscated by Altcha's ["Proof-of-Work" obfuscation scheme](https://altcha.org/docs/v2/obfuscation/#how-it-works) present in 0.8.0 and later versions.

Altcha documentation describes the design of the obfuscation scheme as follows:

> Similar to the proof-of-work mechanism used for ALTCHA challenges, obfuscation employs a proof-of-work (PoW) approach based on symmetric AES encryption.
>
> The obfuscated data provided to the widget is encrypted using an encryption key and an initialization vector based on a random number. Just like the challenge PoW, _the client must iterate over a range of numbers_ to find a matching initialization vector.
>
> The encryption key is _shared with the client_ and defaults to an empty string if not specified.
>
> [Data Obfuscation - Altcha.org](https://altcha.org/docs/v2/obfuscation/)

## Details

The scheme has a crypanalytic total break that allows an untrusted party with learn the secret information (nonce $\mathtt{IV}$ and plaintext $\mathtt{P}$) in constant time due to misuse of symmetric encryption and placement of secret information in a non-confidential position with respect to the mode of operation.

Encryption use cases with an unshared key are not affected.

The core cryptographic routine is generic and has independent educational and reuse value thus was implemented and explained in a separate [gcm-nonceless crate](https://github.com/eternal-flame-AD/gcm-nonceless).

## Demo

A copy of the official obfuscation script is available for reference [here](./obfuscate.ts) (note this script is licensed under MIT to original author, see file header).

A piece of ciphertext taken straight from https://altcha.org/contact/#reporting-security-issues:

```bash
target/release/altcha-deobfs --data HD9IT+QrWtjss/0IgpfKbifkkNsSTFxS6PLv0vTjYfcIrcP1l+TfDpNZ
Nonce: a30f00000000000000000000 (number: 4003)
mailto:security@altcha.org
```

Prove it really is constant time, no searching or precomputation required:

```bash
 time target/release/altcha-deobfs --key SodiumChloride --data \
    $(NUMBER=10000000000 KEY=SodiumChloride ./obfuscate.ts \
    "Meet me at mile marker 663.")
Nonce: 00e40b540200000000000000 (number: 10000000000)
Meet me at mile marker 663.
real    0m0.007s
user    0m0.000s
sys     0m0.008s
```

## Vendor Notification

This was identified to be a vulnerability or other security issue that requires embargo. As such, a vendor notification was sent on two occasions under the [vendor's provided timeline](https://github.com/altcha-org/altcha/blob/ac1e6192ecc7a77115be213834d518f6b025613f/SECURITY.md):

- 11/11/2025 for first contact
- 11/14/2025 for second contact with additional information

As of today, 11/22/2025, this is considered dismissed due to no response, out of scope, dated and published as is.

Catalog numbers: CVE-2025-65849 [GHSA-mpmc-qchh-r9q8](https://github.com/advisories/ghsa-mpmc-qchh-r9q8)

## References

- [CWE-327: Use of a Broken or Risky Cryptographic Algorithm](https://cwe.mitre.org/data/definitions/327.html)
- [The Galois/Counter Mode of Operation (GCM)](https://csrc.nist.rip/groups/ST/toolkit/BCM/documents/proposedmodes/gcm/gcm-spec.pdf)
