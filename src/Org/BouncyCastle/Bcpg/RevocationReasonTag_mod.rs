#[cfg(feature = "Org+BouncyCastle+Bcpg+RevocationReasonTag")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RevocationReasonTag {
    #[default]
    KeyCompromised = 2u8,
    KeyRetired = 3u8,
    KeySuperseded = 1u8,
    NoReason = 0u8,
    UserNoLongerValid = 32u8,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+RevocationReasonTag")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Bcpg::RevocationReasonTag =>
    "Org.BouncyCastle.Bcpg"."RevocationReasonTag"
);
