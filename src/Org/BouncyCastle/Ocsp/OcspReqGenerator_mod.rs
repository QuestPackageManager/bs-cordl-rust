#[cfg(feature = "Org+BouncyCastle+Ocsp+OcspReqGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct OcspReqGenerator {
    __cordl_parent: crate::System::Object,
    pub list: *mut crate::System::Collections::IList,
    pub requestorName: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    pub requestExtensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+OcspReqGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Ocsp::OcspReqGenerator =>
    "Org.BouncyCastle.Ocsp"."OcspReqGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Ocsp+OcspReqGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Ocsp::OcspReqGenerator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+OcspReqGenerator")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Ocsp::OcspReqGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+OcspReqGenerator")]
impl crate::Org::BouncyCastle::Ocsp::OcspReqGenerator {
    #[cfg(feature = "Org+BouncyCastle+Ocsp+OcspReqGenerator+RequestObject")]
    pub type RequestObject = crate::Org::BouncyCastle::Ocsp::OcspReqGenerator_RequestObject;
    pub fn get_SignatureAlgNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerable = __cordl_object
            .invoke("get_SignatureAlgNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetRequestExtensions(
        &mut self,
        requestExtensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetRequestExtensions", (requestExtensions))?;
        Ok(__cordl_ret)
    }
    pub fn SetRequestorName_X509Name0(
        &mut self,
        requestorName: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetRequestorName", (requestorName))?;
        Ok(__cordl_ret)
    }
    pub fn SetRequestorName_GeneralName1(
        &mut self,
        requestorName: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetRequestorName", (requestorName))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateRequest(
        &mut self,
        signingAlgorithm: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        chain: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::X509::X509Certificate,
        >,
        random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Ocsp::OcspReq> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Ocsp::OcspReq = __cordl_object
            .invoke("GenerateRequest", (signingAlgorithm, privateKey, chain, random))?;
        Ok(__cordl_ret)
    }
    pub fn Generate_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Ocsp::OcspReq> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Ocsp::OcspReq = __cordl_object
            .invoke("Generate", ())?;
        Ok(__cordl_ret)
    }
    pub fn Generate_String_AsymmetricKeyParameter_Il2CppArray1(
        &mut self,
        signingAlgorithm: *mut crate::System::String,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        chain: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::X509::X509Certificate,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Ocsp::OcspReq> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Ocsp::OcspReq = __cordl_object
            .invoke("Generate", (signingAlgorithm, privateKey, chain))?;
        Ok(__cordl_ret)
    }
    pub fn Generate_String_AsymmetricKeyParameter_Il2CppArray_SecureRandom2(
        &mut self,
        signingAlgorithm: *mut crate::System::String,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        chain: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::X509::X509Certificate,
        >,
        random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Ocsp::OcspReq> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Ocsp::OcspReq = __cordl_object
            .invoke("Generate", (signingAlgorithm, privateKey, chain, random))?;
        Ok(__cordl_ret)
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
    pub fn AddRequest_CertificateID0(
        &mut self,
        certId: *mut crate::Org::BouncyCastle::Ocsp::CertificateID,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddRequest", (certId))?;
        Ok(__cordl_ret)
    }
    pub fn AddRequest_X509Extensions1(
        &mut self,
        certId: *mut crate::Org::BouncyCastle::Ocsp::CertificateID,
        singleRequestExtensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddRequest", (certId, singleRequestExtensions))?;
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
#[cfg(feature = "Org+BouncyCastle+Ocsp+OcspReqGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Ocsp::OcspReqGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+OcspReqGenerator+RequestObject")]
#[repr(C)]
#[derive(Debug)]
pub struct OcspReqGenerator_RequestObject {
    __cordl_parent: crate::System::Object,
    pub certId: *mut crate::Org::BouncyCastle::Ocsp::CertificateID,
    pub extensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+OcspReqGenerator+RequestObject")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Ocsp::OcspReqGenerator_RequestObject => "Org.BouncyCastle.Ocsp"
    ."OcspReqGenerator/RequestObject"
);
#[cfg(feature = "Org+BouncyCastle+Ocsp+OcspReqGenerator+RequestObject")]
impl std::ops::Deref for crate::Org::BouncyCastle::Ocsp::OcspReqGenerator_RequestObject {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+OcspReqGenerator+RequestObject")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Ocsp::OcspReqGenerator_RequestObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+OcspReqGenerator+RequestObject")]
impl crate::Org::BouncyCastle::Ocsp::OcspReqGenerator_RequestObject {
    pub fn _ctor(
        &mut self,
        certId: *mut crate::Org::BouncyCastle::Ocsp::CertificateID,
        extensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (certId, extensions))?;
        Ok(__cordl_ret)
    }
    pub fn ToRequest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Ocsp::Request,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Ocsp::Request = __cordl_object
            .invoke("ToRequest", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        certId: *mut crate::Org::BouncyCastle::Ocsp::CertificateID,
        extensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (certId, extensions))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+OcspReqGenerator+RequestObject")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Ocsp::OcspReqGenerator_RequestObject {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
