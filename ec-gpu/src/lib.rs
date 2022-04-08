use ff::Field;

/// Describes how to generate the elliptic curve operations for
/// - `Scalar`
/// - `Fp`
/// - `Fp2`
/// - `G1`
/// - `G2`
pub trait GpuEngine {
    type Scalar: GpuField;
    type Fp: GpuField;
}

// TODO vmx 2022-04-06: Check if "least signigicant limb first is actually correct".

/// A prime field that returns the values in a representation that is suited for the use on a GPU.
pub trait GpuField: Field {
    // TODO vmx 2022-04-06: rename to `r` so that it is in montgomery form.
    /// Returns `1` as a vector of 32-bit limbs in little-endian non-Montgomery form (least significant limb first).
    fn one() -> Vec<u32>;

    /// Returns `R ^ 2 mod P` as a vector of 32-bit limbs in little-endian non-Montgomery form (least significant limb first).
    fn r2() -> Vec<u32>;

    /// Returns the field modulus as vector of 32-bit limbs in little-endian non-Montgomery form (least significant limb first).
    fn modulus() -> Vec<u32>;

    ///// Returns `0` as a vector of 32-bit limbs in TODO vmx 2022-04-06: do what the other forms also do
    //fn zero() -> Vec<u32>;
}
