#[cfg(feature = "Org+BouncyCastle+Bcpg+PublicKeyAlgorithmTag")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PublicKeyAlgorithmTag {
    #[default]
    DiffieHellman = 21i32,
    Dsa = 17i32,
    EC = 18i32,
    ECDsa = 19i32,
    EdDsa = 22i32,
    ElGamalEncrypt = 16i32,
    ElGamalGeneral = 20i32,
    Experimental_1 = 100i32,
    Experimental_10 = 109i32,
    Experimental_11 = 110i32,
    Experimental_2 = 101i32,
    Experimental_3 = 102i32,
    Experimental_4 = 103i32,
    Experimental_5 = 104i32,
    Experimental_6 = 105i32,
    Experimental_7 = 106i32,
    Experimental_8 = 107i32,
    Experimental_9 = 108i32,
    RsaEncrypt = 2i32,
    RsaGeneral = 1i32,
    RsaSign = 3i32,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+PublicKeyAlgorithmTag")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag
    => "Org.BouncyCastle.Bcpg"."PublicKeyAlgorithmTag"
);
