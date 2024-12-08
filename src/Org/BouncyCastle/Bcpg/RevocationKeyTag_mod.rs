#[cfg(feature = "Org+BouncyCastle+Bcpg+RevocationKeyTag")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RevocationKeyTag {
    ClassDefault = 128u8,
    ClassSensitive = 64u8,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+RevocationKeyTag")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Bcpg::RevocationKeyTag =>
    "Org.BouncyCastle.Bcpg"."RevocationKeyTag"
);
