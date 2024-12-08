#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SignatureAlgorithm")]
#[repr(C)]
#[derive(Debug)]
pub struct SignatureAlgorithm {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SignatureAlgorithm")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::SignatureAlgorithm => "Org.BouncyCastle.Crypto.Tls"
    ."SignatureAlgorithm"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SignatureAlgorithm")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::SignatureAlgorithm {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SignatureAlgorithm")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::SignatureAlgorithm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SignatureAlgorithm")]
impl crate::Org::BouncyCastle::Crypto::Tls::SignatureAlgorithm {
    pub const anonymous: u8 = 0u8;
    pub const dsa: u8 = 2u8;
    pub const ecdsa: u8 = 3u8;
    pub const rsa: u8 = 1u8;
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SignatureAlgorithm")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::SignatureAlgorithm {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}