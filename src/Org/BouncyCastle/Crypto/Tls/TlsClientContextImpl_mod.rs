#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsClientContextImpl")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsClientContextImpl {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsContext,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsClientContextImpl")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::TlsClientContextImpl =>
    "Org.BouncyCastle.Crypto.Tls"."TlsClientContextImpl"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsClientContextImpl")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::TlsClientContextImpl {
    type Target = crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsContext;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsClientContextImpl")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::TlsClientContextImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsClientContextImpl")]
impl crate::Org::BouncyCastle::Crypto::Tls::TlsClientContextImpl {
    pub fn New(
        secureRandom: *mut crate::Org::BouncyCastle::Security::SecureRandom,
        securityParameters: *mut crate::Org::BouncyCastle::Crypto::Tls::SecurityParameters,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (secureRandom, securityParameters))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        secureRandom: *mut crate::Org::BouncyCastle::Security::SecureRandom,
        securityParameters: *mut crate::Org::BouncyCastle::Crypto::Tls::SecurityParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (secureRandom, securityParameters))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsServer(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsServer", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsClientContextImpl")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::TlsClientContextImpl {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
