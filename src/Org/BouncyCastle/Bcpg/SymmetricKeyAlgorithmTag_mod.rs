#[cfg(feature = "Org+BouncyCastle+Bcpg+SymmetricKeyAlgorithmTag")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SymmetricKeyAlgorithmTag {
    #[default]
    Aes128 = 7i32,
    Aes192 = 8i32,
    Aes256 = 9i32,
    Blowfish = 4i32,
    Camellia128 = 11i32,
    Camellia192 = 12i32,
    Camellia256 = 13i32,
    Cast5 = 3i32,
    Des = 6i32,
    Idea = 1i32,
    Null = 0i32,
    Safer = 5i32,
    TripleDes = 2i32,
    Twofish = 10i32,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+SymmetricKeyAlgorithmTag")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag => "Org.BouncyCastle.Bcpg"
    ."SymmetricKeyAlgorithmTag"
);
