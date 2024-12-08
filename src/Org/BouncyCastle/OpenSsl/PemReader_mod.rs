#[cfg(feature = "Org+BouncyCastle+OpenSsl+PemReader")]
#[repr(C)]
#[derive(Debug)]
pub struct PemReader {
    __cordl_parent: crate::Org::BouncyCastle::Utilities::IO::Pem::PemReader,
    pub pFinder: *mut crate::Org::BouncyCastle::OpenSsl::IPasswordFinder,
}
#[cfg(feature = "Org+BouncyCastle+OpenSsl+PemReader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::OpenSsl::PemReader =>
    "Org.BouncyCastle.OpenSsl"."PemReader"
);
#[cfg(feature = "Org+BouncyCastle+OpenSsl+PemReader")]
impl std::ops::Deref for crate::Org::BouncyCastle::OpenSsl::PemReader {
    type Target = crate::Org::BouncyCastle::Utilities::IO::Pem::PemReader;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+OpenSsl+PemReader")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::OpenSsl::PemReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+OpenSsl+PemReader")]
impl crate::Org::BouncyCastle::OpenSsl::PemReader {
    pub fn New_IPasswordFinder1(
        reader: *mut crate::System::IO::TextReader,
        pFinder: *mut crate::Org::BouncyCastle::OpenSsl::IPasswordFinder,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reader, pFinder))?;
        Ok(__cordl_object)
    }
    pub fn New_TextReader0(
        reader: *mut crate::System::IO::TextReader,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reader))?;
        Ok(__cordl_object)
    }
    pub fn ReadAttributeCertificate(
        &mut self,
        pemObject: *mut crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::X509::IX509AttributeCertificate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::X509::IX509AttributeCertificate = __cordl_object
            .invoke("ReadAttributeCertificate", (pemObject))?;
        Ok(__cordl_ret)
    }
    pub fn ReadCertificate(
        &mut self,
        pemObject: *mut crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::X509::X509Certificate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::X509::X509Certificate = __cordl_object
            .invoke("ReadCertificate", (pemObject))?;
        Ok(__cordl_ret)
    }
    pub fn ReadCertificateRequest(
        &mut self,
        pemObject: *mut crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Pkcs::Pkcs10CertificationRequest,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Pkcs::Pkcs10CertificationRequest = __cordl_object
            .invoke("ReadCertificateRequest", (pemObject))?;
        Ok(__cordl_ret)
    }
    pub fn ReadCrl(
        &mut self,
        pemObject: *mut crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::X509::X509Crl> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::X509::X509Crl = __cordl_object
            .invoke("ReadCrl", (pemObject))?;
        Ok(__cordl_ret)
    }
    pub fn ReadObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ReadObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadPkcs7(
        &mut self,
        pemObject: *mut crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfo = __cordl_object
            .invoke("ReadPkcs7", (pemObject))?;
        Ok(__cordl_ret)
    }
    pub fn ReadPrivateKey(
        &mut self,
        pemObject: *mut crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ReadPrivateKey", (pemObject))?;
        Ok(__cordl_ret)
    }
    pub fn ReadPublicKey(
        &mut self,
        pemObject: *mut crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter = __cordl_object
            .invoke("ReadPublicKey", (pemObject))?;
        Ok(__cordl_ret)
    }
    pub fn ReadRsaPublicKey(
        &mut self,
        pemObject: *mut crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter = __cordl_object
            .invoke("ReadRsaPublicKey", (pemObject))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IPasswordFinder1(
        &mut self,
        reader: *mut crate::System::IO::TextReader,
        pFinder: *mut crate::Org::BouncyCastle::OpenSsl::IPasswordFinder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (reader, pFinder))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_TextReader0(
        &mut self,
        reader: *mut crate::System::IO::TextReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (reader))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+OpenSsl+PemReader")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::OpenSsl::PemReader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
