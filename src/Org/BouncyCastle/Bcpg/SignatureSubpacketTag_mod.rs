#[cfg(feature = "Org+BouncyCastle+Bcpg+SignatureSubpacketTag")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SignatureSubpacketTag {
    #[default]
    CreationTime = 2i32,
    EmbeddedSignature = 32i32,
    ExpireTime = 3i32,
    Exportable = 4i32,
    Features = 30i32,
    IssuerKeyId = 16i32,
    KeyExpireTime = 9i32,
    KeyFlags = 27i32,
    KeyServerPreferences = 23i32,
    NotationData = 20i32,
    Placeholder = 10i32,
    PolicyUrl = 26i32,
    PreferredCompressionAlgorithms = 22i32,
    PreferredHashAlgorithms = 21i32,
    PreferredKeyServer = 24i32,
    PreferredSymmetricAlgorithms = 11i32,
    PrimaryUserId = 25i32,
    RegExp = 6i32,
    Revocable = 7i32,
    RevocationKey = 12i32,
    RevocationReason = 29i32,
    SignatureTarget = 31i32,
    SignerUserId = 28i32,
    TrustSig = 5i32,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+SignatureSubpacketTag")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Bcpg::SignatureSubpacketTag
    => "Org.BouncyCastle.Bcpg"."SignatureSubpacketTag"
);
