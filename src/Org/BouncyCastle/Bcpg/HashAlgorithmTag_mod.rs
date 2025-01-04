#[cfg(feature = "Org+BouncyCastle+Bcpg+HashAlgorithmTag")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HashAlgorithmTag {
    #[default]
    DoubleSha = 4i32,
    Haval5pass160 = 7i32,
    MD2 = 5i32,
    MD5 = 1i32,
    RipeMD160 = 3i32,
    Sha1 = 2i32,
    Sha224 = 11i32,
    Sha256 = 8i32,
    Sha384 = 9i32,
    Sha512 = 10i32,
    Tiger192 = 6i32,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+HashAlgorithmTag")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Bcpg::HashAlgorithmTag =>
    "Org.BouncyCastle.Bcpg"."HashAlgorithmTag"
);
