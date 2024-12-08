#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsDHVerifier")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsDHVerifier {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsDHVerifier")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::TlsDHVerifier =>
    "Org.BouncyCastle.Crypto.Tls"."TlsDHVerifier"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsDHVerifier")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::TlsDHVerifier {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsDHVerifier")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::TlsDHVerifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsDHVerifier")]
impl crate::Org::BouncyCastle::Crypto::Tls::TlsDHVerifier {
    pub fn Accept(
        &mut self,
        dhParameters: *mut crate::Org::BouncyCastle::Crypto::Parameters::DHParameters,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Accept", (dhParameters))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsDHVerifier")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::TlsDHVerifier {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
