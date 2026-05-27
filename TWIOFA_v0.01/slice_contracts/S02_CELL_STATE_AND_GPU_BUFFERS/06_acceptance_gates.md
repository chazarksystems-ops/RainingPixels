# Acceptance Gates

The slice is accepted when:

1. All four bytes of the cell word are defined.
2. Rust and WGSL masks/shifts match.
3. Test vectors round-trip.
4. Field overflow/clamping behavior is documented.
5. V0 meanings for scalar and aux are documented.
6. Movement/race claims are not overstated.
7. A future material pass can depend on the contract without ambiguity.
