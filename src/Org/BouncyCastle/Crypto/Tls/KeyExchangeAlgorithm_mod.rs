#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+KeyExchangeAlgorithm")]
#[repr(C)]
#[derive(Debug)]
pub struct KeyExchangeAlgorithm {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+KeyExchangeAlgorithm")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::KeyExchangeAlgorithm =>
    "Org.BouncyCastle.Crypto.Tls"."KeyExchangeAlgorithm"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+KeyExchangeAlgorithm")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::KeyExchangeAlgorithm {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+KeyExchangeAlgorithm")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::KeyExchangeAlgorithm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+KeyExchangeAlgorithm")]
impl crate::Org::BouncyCastle::Crypto::Tls::KeyExchangeAlgorithm {
    pub const DHE_DSS: i32 = 3i32;
    pub const DHE_DSS_EXPORT: i32 = 4i32;
    pub const DHE_PSK: i32 = 14i32;
    pub const DHE_RSA: i32 = 5i32;
    pub const DHE_RSA_EXPORT: i32 = 6i32;
    pub const DH_DSS: i32 = 7i32;
    pub const DH_DSS_EXPORT: i32 = 8i32;
    pub const DH_RSA: i32 = 9i32;
    pub const DH_RSA_EXPORT: i32 = 10i32;
    pub const DH_anon: i32 = 11i32;
    pub const DH_anon_EXPORT: i32 = 12i32;
    pub const ECDHE_ECDSA: i32 = 17i32;
    pub const ECDHE_PSK: i32 = 24i32;
    pub const ECDHE_RSA: i32 = 19i32;
    pub const ECDH_ECDSA: i32 = 16i32;
    pub const ECDH_RSA: i32 = 18i32;
    pub const ECDH_anon: i32 = 20i32;
    pub const PSK: i32 = 13i32;
    pub const RSA: i32 = 1i32;
    pub const RSA_EXPORT: i32 = 2i32;
    pub const RSA_PSK: i32 = 15i32;
    pub const SRP: i32 = 21i32;
    pub const SRP_DSS: i32 = 22i32;
    pub const SRP_RSA: i32 = 23i32;
    pub const _cordl_NULL: i32 = 0i32;
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+KeyExchangeAlgorithm")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::KeyExchangeAlgorithm {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
