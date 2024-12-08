#[cfg(feature = "Org+BouncyCastle+Security+DigestUtilities+DigestAlgorithm")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DigestUtilities_DigestAlgorithm {
    BLAKE2B_160 = 0i32,
    BLAKE2B_256 = 1i32,
    BLAKE2B_384 = 2i32,
    BLAKE2B_512 = 3i32,
    BLAKE2S_128 = 4i32,
    BLAKE2S_160 = 5i32,
    BLAKE2S_224 = 6i32,
    BLAKE2S_256 = 7i32,
    DSTU7564_256 = 8i32,
    DSTU7564_384 = 9i32,
    DSTU7564_512 = 10i32,
    GOST3411 = 11i32,
    GOST3411_2012_256 = 12i32,
    GOST3411_2012_512 = 13i32,
    KECCAK_224 = 14i32,
    KECCAK_256 = 15i32,
    KECCAK_288 = 16i32,
    KECCAK_384 = 17i32,
    KECCAK_512 = 18i32,
    MD2 = 19i32,
    MD4 = 20i32,
    MD5 = 21i32,
    NONE = 22i32,
    RIPEMD128 = 23i32,
    RIPEMD160 = 24i32,
    RIPEMD256 = 25i32,
    RIPEMD320 = 26i32,
    SHA3_224 = 34i32,
    SHA3_256 = 35i32,
    SHA3_384 = 36i32,
    SHA3_512 = 37i32,
    SHAKE128 = 38i32,
    SHAKE256 = 39i32,
    SHA_1 = 27i32,
    SHA_224 = 28i32,
    SHA_256 = 29i32,
    SHA_384 = 30i32,
    SHA_512 = 31i32,
    SHA_512_224 = 32i32,
    SHA_512_256 = 33i32,
    SM3 = 40i32,
    TIGER = 41i32,
    WHIRLPOOL = 42i32,
}
#[cfg(feature = "Org+BouncyCastle+Security+DigestUtilities+DigestAlgorithm")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Security::DigestUtilities_DigestAlgorithm =>
    "Org.BouncyCastle.Security"."DigestUtilities/DigestAlgorithm"
);
#[cfg(feature = "Org+BouncyCastle+Security+DigestUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct DigestUtilities {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Org+BouncyCastle+Security+DigestUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Security::DigestUtilities =>
    "Org.BouncyCastle.Security"."DigestUtilities"
);
#[cfg(feature = "Org+BouncyCastle+Security+DigestUtilities")]
impl std::ops::Deref for crate::Org::BouncyCastle::Security::DigestUtilities {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+DigestUtilities")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Security::DigestUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+DigestUtilities")]
impl crate::Org::BouncyCastle::Security::DigestUtilities {
    #[cfg(feature = "Org+BouncyCastle+Security+DigestUtilities+DigestAlgorithm")]
    pub type DigestAlgorithm = crate::Org::BouncyCastle::Security::DigestUtilities_DigestAlgorithm;
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+DigestUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Security::DigestUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
