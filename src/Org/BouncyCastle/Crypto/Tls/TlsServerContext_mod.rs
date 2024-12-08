#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsServerContext")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsServerContext {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsServerContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::TlsServerContext
    => "Org.BouncyCastle.Crypto.Tls"."TlsServerContext"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsServerContext")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::TlsServerContext {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsServerContext")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::TlsServerContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsServerContext")]
impl crate::Org::BouncyCastle::Crypto::Tls::TlsServerContext {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsServerContext")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::TlsServerContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
