use crate::Error;

/// A trait collecting information about a given `hash-to-curve`
/// suite.
///
/// NOTE: At the moment, the following restrictions apply:
///
/// * Curve must be over a prime order field.
/// * Suite must specify uniform output encoding.
///
pub trait HashToCurve {
    /// The SuiteID.
    const ID: &'static str;

    /// The target security level of the suite in bits.
    const K: usize;

    /// The length parameter for [hash_to_field](HashToCurveSuite::hash_to_field).
    const L: usize;

    /// A point type for an elliptic curve over the [base field](HashToCurveSuite::BaseField).
    type OutputCurve;

    /// A field of prime characteristic p ≠ 2.
    type BaseField;

    /// `hash_to_curve` is a uniform encoding from byte strings to points in
    /// G.  That is, the distribution of its output is statistically close
    /// to uniform in G.
    ///
    /// This function is suitable for most applications requiring a random
    /// oracle returning points in G, when instantiated with any of the
    /// map_to_curve functions described in Section 6.  See Section 10.1
    /// for further discussion.
    ///
    /// ``` text
    ///       hash_to_curve(msg)
    ///
    ///       Input: msg, an arbitrary-length byte string.
    ///       Output: P, a point in G.
    /// ```
    fn hash_to_curve(msg: &[u8], dst: &[u8]) -> Result<Self::OutputCurve, Error>;

    // TODO: What about `encode_to_curve`?
}
