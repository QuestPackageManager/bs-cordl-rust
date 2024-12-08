#[cfg(feature = "Org+BouncyCastle+Ocsp+BasicOcspRespGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct BasicOcspRespGenerator {
    __cordl_parent: crate::System::Object,
    pub list: *mut crate::System::Collections::IList,
    pub responseExtensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    pub responderID: *mut crate::Org::BouncyCastle::Ocsp::RespID,
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+BasicOcspRespGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator
    => "Org.BouncyCastle.Ocsp"."BasicOcspRespGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Ocsp+BasicOcspRespGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+BasicOcspRespGenerator")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+BasicOcspRespGenerator")]
impl crate::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator {
    #[cfg(feature = "Org+BouncyCastle+Ocsp+BasicOcspRespGenerator+ResponseObject")]
    pub type ResponseObject = crate::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator_ResponseObject;
    pub fn AddResponse_CertificateID_CertificateStatus0(
        &mut self,
        certID: *mut crate::Org::BouncyCastle::Ocsp::CertificateID,
        certStatus: *mut crate::Org::BouncyCastle::Ocsp::CertificateStatus,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddResponse", (certID, certStatus))?;
        Ok(__cordl_ret)
    }
    pub fn AddResponse_DateTime_DateTime_X509Extensions3(
        &mut self,
        certID: *mut crate::Org::BouncyCastle::Ocsp::CertificateID,
        certStatus: *mut crate::Org::BouncyCastle::Ocsp::CertificateStatus,
        thisUpdate: crate::System::DateTime,
        nextUpdate: crate::System::DateTime,
        singleExtensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddResponse",
                (certID, certStatus, thisUpdate, nextUpdate, singleExtensions),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddResponse_DateTime_X509Extensions2(
        &mut self,
        certID: *mut crate::Org::BouncyCastle::Ocsp::CertificateID,
        certStatus: *mut crate::Org::BouncyCastle::Ocsp::CertificateStatus,
        nextUpdate: crate::System::DateTime,
        singleExtensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddResponse", (certID, certStatus, nextUpdate, singleExtensions))?;
        Ok(__cordl_ret)
    }
    pub fn AddResponse_X509Extensions1(
        &mut self,
        certID: *mut crate::Org::BouncyCastle::Ocsp::CertificateID,
        certStatus: *mut crate::Org::BouncyCastle::Ocsp::CertificateStatus,
        singleExtensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddResponse", (certID, certStatus, singleExtensions))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateResponse(
        &mut self,
        signatureCalculator: *mut crate::Org::BouncyCastle::Crypto::ISignatureFactory,
        chain: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::X509::X509Certificate,
        >,
        producedAt: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Ocsp::BasicOcspResp,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Ocsp::BasicOcspResp = __cordl_object
            .invoke("GenerateResponse", (signatureCalculator, chain, producedAt))?;
        Ok(__cordl_ret)
    }
    pub fn Generate_ISignatureFactory_Il2CppArray_DateTime2(
        &mut self,
        signatureCalculatorFactory: *mut crate::Org::BouncyCastle::Crypto::ISignatureFactory,
        chain: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::X509::X509Certificate,
        >,
        producedAt: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Ocsp::BasicOcspResp,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Ocsp::BasicOcspResp = __cordl_object
            .invoke("Generate", (signatureCalculatorFactory, chain, producedAt))?;
        Ok(__cordl_ret)
    }
    pub fn Generate_String_AsymmetricKeyParameter_Il2CppArray_DateTime0(
        &mut self,
        signingAlgorithm: *mut crate::System::String,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        chain: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::X509::X509Certificate,
        >,
        thisUpdate: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Ocsp::BasicOcspResp,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Ocsp::BasicOcspResp = __cordl_object
            .invoke("Generate", (signingAlgorithm, privateKey, chain, thisUpdate))?;
        Ok(__cordl_ret)
    }
    pub fn Generate_String_AsymmetricKeyParameter_Il2CppArray_DateTime_SecureRandom1(
        &mut self,
        signingAlgorithm: *mut crate::System::String,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        chain: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::X509::X509Certificate,
        >,
        producedAt: crate::System::DateTime,
        random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Ocsp::BasicOcspResp,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Ocsp::BasicOcspResp = __cordl_object
            .invoke(
                "Generate",
                (signingAlgorithm, privateKey, chain, producedAt, random),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New_AsymmetricKeyParameter1(
        publicKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (publicKey))?;
        Ok(__cordl_object)
    }
    pub fn New_RespID0(
        responderID: *mut crate::Org::BouncyCastle::Ocsp::RespID,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (responderID))?;
        Ok(__cordl_object)
    }
    pub fn SetResponseExtensions(
        &mut self,
        responseExtensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetResponseExtensions", (responseExtensions))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_AsymmetricKeyParameter1(
        &mut self,
        publicKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (publicKey))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_RespID0(
        &mut self,
        responderID: *mut crate::Org::BouncyCastle::Ocsp::RespID,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (responderID))?;
        Ok(__cordl_ret)
    }
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
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+BasicOcspRespGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+BasicOcspRespGenerator+ResponseObject")]
#[repr(C)]
#[derive(Debug)]
pub struct BasicOcspRespGenerator_ResponseObject {
    __cordl_parent: crate::System::Object,
    pub certId: *mut crate::Org::BouncyCastle::Ocsp::CertificateID,
    pub certStatus: *mut crate::Org::BouncyCastle::Asn1::Ocsp::CertStatus,
    pub thisUpdate: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
    pub nextUpdate: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
    pub extensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+BasicOcspRespGenerator+ResponseObject")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator_ResponseObject =>
    "Org.BouncyCastle.Ocsp"."BasicOcspRespGenerator/ResponseObject"
);
#[cfg(feature = "Org+BouncyCastle+Ocsp+BasicOcspRespGenerator+ResponseObject")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator_ResponseObject {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+BasicOcspRespGenerator+ResponseObject")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator_ResponseObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+BasicOcspRespGenerator+ResponseObject")]
impl crate::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator_ResponseObject {
    pub fn New_DateTime_DateTime_X509Extensions1(
        certId: *mut crate::Org::BouncyCastle::Ocsp::CertificateID,
        certStatus: *mut crate::Org::BouncyCastle::Ocsp::CertificateStatus,
        thisUpdate: crate::System::DateTime,
        nextUpdate: crate::System::DateTime,
        extensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (certId, certStatus, thisUpdate, nextUpdate, extensions),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_DateTime_X509Extensions0(
        certId: *mut crate::Org::BouncyCastle::Ocsp::CertificateID,
        certStatus: *mut crate::Org::BouncyCastle::Ocsp::CertificateStatus,
        thisUpdate: crate::System::DateTime,
        extensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (certId, certStatus, thisUpdate, extensions))?;
        Ok(__cordl_object)
    }
    pub fn New_DerGeneralizedTime_DerGeneralizedTime_X509Extensions2(
        certId: *mut crate::Org::BouncyCastle::Ocsp::CertificateID,
        certStatus: *mut crate::Org::BouncyCastle::Ocsp::CertificateStatus,
        thisUpdate: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
        nextUpdate: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
        extensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (certId, certStatus, thisUpdate, nextUpdate, extensions),
            )?;
        Ok(__cordl_object)
    }
    pub fn ToResponse(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Ocsp::SingleResponse,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Ocsp::SingleResponse = __cordl_object
            .invoke("ToResponse", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DateTime_DateTime_X509Extensions1(
        &mut self,
        certId: *mut crate::Org::BouncyCastle::Ocsp::CertificateID,
        certStatus: *mut crate::Org::BouncyCastle::Ocsp::CertificateStatus,
        thisUpdate: crate::System::DateTime,
        nextUpdate: crate::System::DateTime,
        extensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (certId, certStatus, thisUpdate, nextUpdate, extensions))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DateTime_X509Extensions0(
        &mut self,
        certId: *mut crate::Org::BouncyCastle::Ocsp::CertificateID,
        certStatus: *mut crate::Org::BouncyCastle::Ocsp::CertificateStatus,
        thisUpdate: crate::System::DateTime,
        extensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (certId, certStatus, thisUpdate, extensions))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DerGeneralizedTime_DerGeneralizedTime_X509Extensions2(
        &mut self,
        certId: *mut crate::Org::BouncyCastle::Ocsp::CertificateID,
        certStatus: *mut crate::Org::BouncyCastle::Ocsp::CertificateStatus,
        thisUpdate: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
        nextUpdate: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
        extensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (certId, certStatus, thisUpdate, nextUpdate, extensions))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+BasicOcspRespGenerator+ResponseObject")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator_ResponseObject {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
