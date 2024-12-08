#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+EncryptionAlgorithm")]
#[repr(C)]
#[derive(Debug)]
pub struct EncryptionAlgorithm {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+EncryptionAlgorithm")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::EncryptionAlgorithm =>
    "Org.BouncyCastle.Crypto.Tls"."EncryptionAlgorithm"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+EncryptionAlgorithm")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::EncryptionAlgorithm {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+EncryptionAlgorithm")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::EncryptionAlgorithm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+EncryptionAlgorithm")]
impl crate::Org::BouncyCastle::Crypto::Tls::EncryptionAlgorithm {
    pub const AES_128_CBC: i32 = 8i32;
    pub const AES_128_CCM: i32 = 15i32;
    pub const AES_128_CCM_8: i32 = 16i32;
    pub const AES_128_GCM: i32 = 10i32;
    pub const AES_128_OCB_TAGLEN96: i32 = 103i32;
    pub const AES_256_CBC: i32 = 9i32;
    pub const AES_256_CCM: i32 = 17i32;
    pub const AES_256_CCM_8: i32 = 18i32;
    pub const AES_256_GCM: i32 = 11i32;
    pub const AES_256_OCB_TAGLEN96: i32 = 104i32;
    pub const CAMELLIA_128_CBC: i32 = 12i32;
    pub const CAMELLIA_128_GCM: i32 = 19i32;
    pub const CAMELLIA_256_CBC: i32 = 13i32;
    pub const CAMELLIA_256_GCM: i32 = 20i32;
    pub const CHACHA20_POLY1305: i32 = 21i32;
    pub const DES40_CBC: i32 = 5i32;
    pub const DES_CBC: i32 = 6i32;
    pub const IDEA_CBC: i32 = 4i32;
    pub const RC2_CBC_40: i32 = 3i32;
    pub const RC4_128: i32 = 2i32;
    pub const RC4_40: i32 = 1i32;
    pub const SEED_CBC: i32 = 14i32;
    pub const _cordl_NULL: i32 = 0i32;
    pub const cls_3DES_EDE_CBC: i32 = 7i32;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
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
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+EncryptionAlgorithm")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::EncryptionAlgorithm {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}