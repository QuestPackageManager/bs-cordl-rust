#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+CertificateRequest")]
#[repr(C)]
#[derive(Debug)]
pub struct CertificateRequest {
    __cordl_parent: crate::System::Object,
    pub mCertificateTypes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mSupportedSignatureAlgorithms: *mut crate::System::Collections::IList,
    pub mCertificateAuthorities: *mut crate::System::Collections::IList,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+CertificateRequest")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::CertificateRequest => "Org.BouncyCastle.Crypto.Tls"
    ."CertificateRequest"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+CertificateRequest")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::CertificateRequest {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+CertificateRequest")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::CertificateRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+CertificateRequest")]
impl crate::Org::BouncyCastle::Crypto::Tls::CertificateRequest {
    pub fn Encode(
        &mut self,
        output: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Encode", (output))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        certificateTypes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        supportedSignatureAlgorithms: *mut crate::System::Collections::IList,
        certificateAuthorities: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (certificateTypes, supportedSignatureAlgorithms, certificateAuthorities),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        certificateTypes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        supportedSignatureAlgorithms: *mut crate::System::Collections::IList,
        certificateAuthorities: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (certificateTypes, supportedSignatureAlgorithms, certificateAuthorities),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_CertificateAuthorities(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IList = __cordl_object
            .invoke("get_CertificateAuthorities", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CertificateTypes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("get_CertificateTypes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SupportedSignatureAlgorithms(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IList = __cordl_object
            .invoke("get_SupportedSignatureAlgorithms", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+CertificateRequest")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::CertificateRequest {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}