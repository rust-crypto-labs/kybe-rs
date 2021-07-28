`kybe-rs` is a Pure Rust implementation of the CRYSTALS-KYBER lattice-based key encapsulation suite (KYBER [1]), a post-quantum candidate submitted to the NIST standardization process [2].

### Why `kybe-rs`?

The CRYSTALS-KYBER submission already comes with reference implementations, including optimised versions and as made through the 3rd round of NIST standardization process.

`kybe-rs` is concerned with providing high *correctness* guarantees: adherence to the CRYSTALS-KYBER specification, memory and type safety, and reproducibility across platforms. Extensive testing and documentation is desired. Performance matters but is a longer-term concern.

### Status

#### Supported features and algorithms

* Key encapsulation mechanism (`KEM`)
* Public-key encryption (`PKE`)
* All the parameters described in the NIST submission: `kyber-512` and `kyber-768`.

The 2nd round updated specification (30 march 2019) is used as a basis for implementation.

#### Unsupported features and caveats

* The implementation is not guaranteed to be constant time
* The implementation is not `no_std` compatible (for non-essential reasons)

### References and documentation

* https://pq-crystals.org/kyber/
* https://csrc.nist.gov/Projects/Post-Quantum-Cryptography

[1]: https://pq-crystals.org/kyber/
[2]: https://csrc.nist.gov/Projects/Post-Quantum-Cryptography