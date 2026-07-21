//! The `Choice` trait that generated choice-argument types implement.

/// A choice exercisable on a contract of template `T`.
///
/// Generated code implements this on each choice's argument type, linking it to
/// the template it belongs to, the value the choice returns, and its on-ledger
/// name and consuming flag.
pub trait Choice<T> {
    /// The type the choice returns.
    type Return;
    /// The choice name as it appears on the ledger.
    const NAME: &'static str;
    /// Whether exercising the choice archives the contract.
    const CONSUMING: bool;
}
