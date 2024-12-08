#[cfg(feature = "Org+BouncyCastle+Bcpg+PacketTag")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketTag {
    CompressedData = 8i32,
    Experimental1 = 60i32,
    Experimental2 = 61i32,
    Experimental3 = 62i32,
    Experimental4 = 63i32,
    LiteralData = 11i32,
    Marker = 10i32,
    ModificationDetectionCode = 19i32,
    OnePassSignature = 4i32,
    PublicKey = 6i32,
    PublicKeyEncryptedSession = 1i32,
    PublicSubkey = 14i32,
    Reserved = 0i32,
    SecretKey = 5i32,
    SecretSubkey = 7i32,
    Signature = 2i32,
    SymmetricEncryptedIntegrityProtected = 18i32,
    SymmetricKeyEncrypted = 9i32,
    SymmetricKeyEncryptedSessionKey = 3i32,
    Trust = 12i32,
    UserAttribute = 17i32,
    UserId = 13i32,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+PacketTag")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Bcpg::PacketTag =>
    "Org.BouncyCastle.Bcpg"."PacketTag"
);
