#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsAuthentication")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsAuthentication {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsAuthentication")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::TlsAuthentication => "Org.BouncyCastle.Crypto.Tls"
    ."TlsAuthentication"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsAuthentication")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::TlsAuthentication {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsAuthentication")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::TlsAuthentication {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsAuthentication")]
impl crate::Org::BouncyCastle::Crypto::Tls::TlsAuthentication {
    pub fn GetClientCredentials(
        &mut self,
        certificateRequest: *mut crate::Org::BouncyCastle::Crypto::Tls::CertificateRequest,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCredentials,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCredentials = __cordl_object
            .invoke("GetClientCredentials", (certificateRequest))?;
        Ok(__cordl_ret)
    }
    pub fn NotifyServerCertificate(
        &mut self,
        serverCertificate: *mut crate::Org::BouncyCastle::Crypto::Tls::Certificate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyServerCertificate", (serverCertificate))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsAuthentication")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::TlsAuthentication {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
