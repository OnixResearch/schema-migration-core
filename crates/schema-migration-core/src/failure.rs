#[allow(
    non_trait_imports,
    reason = "owned diagnostic text is required by checked identifier failures; owner: schema-migration-core"
)]
use alloc::string::String;

/// Construction and admission failures from the pure migration core.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MigrationError {
    /// A selected resource limit was zero.
    ZeroLimit {
        /// Limit field.
        field: &'static str,
    },
    /// A checked text value was empty.
    EmptyText {
        /// Text field.
        field: &'static str,
    },
    /// A checked text value exceeded its selected byte limit.
    TextTooLong {
        /// Text field.
        field: &'static str,
        /// Observed UTF-8 byte count.
        actual_bytes: u32,
        /// Selected UTF-8 byte limit.
        maximum_bytes: u16,
    },
    /// A catalog exceeded the selected entry limit.
    TooManyEntries,
    /// One schema had too many outgoing migration entries.
    TooManyOutgoingEdges,
    /// One migration entry had too many preconditions.
    TooManyPreconditions,
    /// The catalog reused one stable entry key.
    DuplicateEntryKey {
        /// Conflicting stable key.
        key: String,
    },
    /// One entry repeated a precondition identity and kind.
    DuplicatePrecondition {
        /// Repeated precondition identity.
        precondition: String,
    },
    /// A migration entry connected one schema to itself.
    SelfEdge,
    /// The policy allowed no migration direction.
    NoAllowedDirection,
    /// The policy admitted no implementation cohort.
    NoImplementationCohort,
    /// The policy repeated an implementation cohort.
    DuplicateCohort {
        /// Repeated cohort.
        cohort: String,
    },
    /// The observation set repeated one precondition observation.
    DuplicateObservation {
        /// Repeated precondition identity.
        precondition: String,
    },
    /// Checked cost arithmetic overflowed.
    CostOverflow,
    /// A content identity could not be derived.
    IdentityFailure,
}

impl core::fmt::Display for MigrationError {
    #[allow(
        missing_const_fn,
        mutating_input_in_pure,
        reason = "the core formatting trait requires a mutable formatter and cannot be const; owner: schema-migration-core"
    )]
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ZeroLimit { field } => write!(formatter, "{field} must be non-zero"),
            Self::EmptyText { field } => write!(formatter, "{field} must not be empty"),
            Self::TextTooLong {
                field,
                actual_bytes,
                maximum_bytes,
            } => write!(
                formatter,
                "{field} has {actual_bytes} bytes; maximum is {maximum_bytes}"
            ),
            Self::TooManyEntries => formatter.write_str("catalog entry limit exceeded"),
            Self::TooManyOutgoingEdges => {
                formatter.write_str("catalog outgoing-edge limit exceeded")
            }
            Self::TooManyPreconditions => {
                formatter.write_str("migration precondition limit exceeded")
            }
            Self::DuplicateEntryKey { key } => {
                write!(formatter, "duplicate migration entry key: {key}")
            }
            Self::DuplicatePrecondition { precondition } => {
                write!(
                    formatter,
                    "duplicate migration precondition: {precondition}"
                )
            }
            Self::SelfEdge => formatter.write_str("migration entry source equals target"),
            Self::NoAllowedDirection => formatter.write_str("policy allows no migration direction"),
            Self::NoImplementationCohort => {
                formatter.write_str("policy admits no implementation cohort")
            }
            Self::DuplicateCohort { cohort } => {
                write!(formatter, "duplicate implementation cohort: {cohort}")
            }
            Self::DuplicateObservation { precondition } => {
                write!(formatter, "duplicate observation: {precondition}")
            }
            Self::CostOverflow => formatter.write_str("migration cost arithmetic overflowed"),
            Self::IdentityFailure => formatter.write_str("migration identity framing failed"),
        }
    }
}

impl core::error::Error for MigrationError {}

impl From<content_identity_core::IdentityError> for MigrationError {
    fn from(_error: content_identity_core::IdentityError) -> Self {
        Self::IdentityFailure
    }
}
