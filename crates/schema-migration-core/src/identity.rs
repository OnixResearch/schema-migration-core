#[allow(
    non_trait_imports,
    reason = "checked owned text is the bounded identifier representation; owner: schema-migration-core"
)]
use alloc::string::String;

const MAX_REPRESENTABLE_TEXT_BYTES: u32 = u32::MAX;

/// Maximum UTF-8 byte length for a checked migration identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TextLimit(u16);

impl TextLimit {
    /// Creates a non-zero text limit.
    ///
    /// # Errors
    ///
    /// Returns an error when `maximum_bytes` is zero.
    pub const fn new(maximum_bytes: u16) -> Result<Self, crate::MigrationError> {
        if maximum_bytes == 0 {
            return Err(crate::MigrationError::ZeroLimit {
                field: "text limit",
            });
        }
        Ok(Self(maximum_bytes))
    }

    /// Returns the maximum UTF-8 byte count.
    #[must_use]
    pub const fn maximum_bytes(self) -> u16 {
        self.0
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct CheckedText(String);

impl CheckedText {
    fn new(
        value: impl Into<String>,
        limit: TextLimit,
        field: &'static str,
    ) -> Result<Self, crate::MigrationError> {
        let value = value.into();
        if value.is_empty() {
            return Err(crate::MigrationError::EmptyText { field });
        }
        let actual_bytes =
            u32::try_from(value.len()).map_err(|_error| crate::MigrationError::TextTooLong {
                field,
                actual_bytes: MAX_REPRESENTABLE_TEXT_BYTES,
                maximum_bytes: limit.maximum_bytes(),
            })?;
        if actual_bytes > u32::from(limit.maximum_bytes()) {
            return Err(crate::MigrationError::TextTooLong {
                field,
                actual_bytes,
                maximum_bytes: limit.maximum_bytes(),
            });
        }
        Ok(Self(value))
    }

    const fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

macro_rules! text_id {
    ($(#[$metadata:meta])* $name:ident, $field:literal) => {
        $(#[$metadata])*
        #[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
        pub struct $name(CheckedText);

        impl $name {
            /// Creates one checked identifier.
            ///
            /// # Errors
            ///
            /// Returns an error when the text is empty or exceeds `limit`.
            pub fn new(
                value: impl Into<String>,
                limit: TextLimit,
            ) -> Result<Self, crate::MigrationError> {
                CheckedText::new(value, limit, $field).map(Self)
            }

            /// Returns the checked text.
            #[must_use]
            pub const fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }
    };
}

text_id!(/// Stable caller-owned migration entry key.
    CatalogEntryKey, "catalog entry key");
text_id!(/// Reviewed implementation cohort identity.
    ImplementationCohortId, "implementation cohort id");
text_id!(/// Migration behavior profile identity.
    MigrationProfileId, "migration profile id");
text_id!(/// Required precondition identity.
    PreconditionId, "precondition id");
text_id!(/// Caller-observed precondition fact identity.
    ObservationId, "observation id");
text_id!(/// Caller-owned identity of the source data subject.
    DataSubjectId, "data subject id");

macro_rules! digest_id {
    ($(#[$metadata:meta])* $name:ident) => {
        $(#[$metadata])*
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name([u8; 32]);

        impl $name {
            pub(crate) const fn from_bytes(bytes: [u8; 32]) -> Self {
                Self(bytes)
            }

            /// Returns the raw BLAKE3 digest bytes.
            #[must_use]
            pub const fn as_bytes(&self) -> &[u8; 32] {
                &self.0
            }

            /// Returns the canonical lowercase hexadecimal identity.
            #[must_use]
            pub fn to_hex(self) -> String {
                content_identity_core::Digest::from_bytes(self.0).to_hex()
            }
        }
    };
}

digest_id!(/// Identity of one canonical migration catalog snapshot.
    CatalogId);
digest_id!(/// Identity of one complete canonical migration entry.
    EntryId);
digest_id!(/// Identity of one admitted planning policy.
    PolicyId);
digest_id!(/// Identity of one explicit resource-limit profile.
    LimitsId);
digest_id!(/// Identity of one canonical precondition observation set.
    ObservationSetId);
digest_id!(/// Identity of one ordered selected migration path.
    PathId);
digest_id!(/// Identity of the implementation cohorts selected by one path.
    ImplementationSetId);
digest_id!(/// Complete identity of one migration plan.
    PlanId);
