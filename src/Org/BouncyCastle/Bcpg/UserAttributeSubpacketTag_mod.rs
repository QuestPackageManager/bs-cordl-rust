#[cfg(feature = "Org+BouncyCastle+Bcpg+UserAttributeSubpacketTag")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UserAttributeSubpacketTag {
    #[default]
    ImageAttribute = 1i32,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+UserAttributeSubpacketTag")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Bcpg::UserAttributeSubpacketTag => "Org.BouncyCastle.Bcpg"
    ."UserAttributeSubpacketTag"
);
