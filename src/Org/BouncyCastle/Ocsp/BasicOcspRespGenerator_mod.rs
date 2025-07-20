#[cfg(feature = "Org+BouncyCastle+Ocsp+BasicOcspRespGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct BasicOcspRespGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub list: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    pub responseExtensions: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    >,
    pub responderID: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Ocsp::RespID>,
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+BasicOcspRespGenerator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Ocsp";
    const CLASS_NAME: &'static str = "BasicOcspRespGenerator";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+BasicOcspRespGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        certID: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Ocsp::CertificateID>,
        certStatus: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Ocsp::CertificateStatus,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Ocsp::CertificateID,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Ocsp::CertificateStatus,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("AddResponse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "AddResponse", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (certID, certStatus))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddResponse_DateTime_DateTime_X509Extensions3(
        &mut self,
        certID: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Ocsp::CertificateID>,
        certStatus: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Ocsp::CertificateStatus,
        >,
        thisUpdate: crate::System::DateTime,
        nextUpdate: crate::System::DateTime,
        singleExtensions: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Ocsp::CertificateID,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Ocsp::CertificateStatus,
                    >,
                    crate::System::DateTime,
                    crate::System::DateTime,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("AddResponse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "AddResponse", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (certID, certStatus, thisUpdate, nextUpdate, singleExtensions),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddResponse_DateTime_X509Extensions2(
        &mut self,
        certID: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Ocsp::CertificateID>,
        certStatus: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Ocsp::CertificateStatus,
        >,
        nextUpdate: crate::System::DateTime,
        singleExtensions: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Ocsp::CertificateID,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Ocsp::CertificateStatus,
                    >,
                    crate::System::DateTime,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("AddResponse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "AddResponse", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (certID, certStatus, nextUpdate, singleExtensions),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddResponse_X509Extensions1(
        &mut self,
        certID: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Ocsp::CertificateID>,
        certStatus: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Ocsp::CertificateStatus,
        >,
        singleExtensions: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Ocsp::CertificateID,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Ocsp::CertificateStatus,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("AddResponse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "AddResponse", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (certID, certStatus, singleExtensions))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateResponse(
        &mut self,
        signatureCalculator: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ISignatureFactory,
        >,
        chain: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::X509::X509Certificate,
                >,
            >,
        >,
        producedAt: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Ocsp::BasicOcspResp>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::ISignatureFactory,
                    >,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::X509::X509Certificate,
                            >,
                        >,
                    >,
                    crate::System::DateTime,
                ),
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Ocsp::BasicOcspResp>,
                3usize,
            >("GenerateResponse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "GenerateResponse", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Ocsp::BasicOcspResp,
        > = unsafe {
            method.invoke_unchecked(self, (signatureCalculator, chain, producedAt))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Generate_ISignatureFactory_Il2CppArray_DateTime2(
        &mut self,
        signatureCalculatorFactory: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ISignatureFactory,
        >,
        chain: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::X509::X509Certificate,
                >,
            >,
        >,
        producedAt: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Ocsp::BasicOcspResp>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::ISignatureFactory,
                    >,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::X509::X509Certificate,
                            >,
                        >,
                    >,
                    crate::System::DateTime,
                ),
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Ocsp::BasicOcspResp>,
                3usize,
            >("Generate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "Generate", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Ocsp::BasicOcspResp,
        > = unsafe {
            method
                .invoke_unchecked(self, (signatureCalculatorFactory, chain, producedAt))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Generate_Il2CppString_AsymmetricKeyParameter_Il2CppArray_DateTime0(
        &mut self,
        signingAlgorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        privateKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        chain: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::X509::X509Certificate,
                >,
            >,
        >,
        thisUpdate: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Ocsp::BasicOcspResp>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                    >,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::X509::X509Certificate,
                            >,
                        >,
                    >,
                    crate::System::DateTime,
                ),
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Ocsp::BasicOcspResp>,
                4usize,
            >("Generate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "Generate", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Ocsp::BasicOcspResp,
        > = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (signingAlgorithm, privateKey, chain, thisUpdate),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Generate_Il2CppString_AsymmetricKeyParameter_Il2CppArray_DateTime_SecureRandom1(
        &mut self,
        signingAlgorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        privateKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        chain: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::X509::X509Certificate,
                >,
            >,
        >,
        producedAt: crate::System::DateTime,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Ocsp::BasicOcspResp>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                    >,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::X509::X509Certificate,
                            >,
                        >,
                    >,
                    crate::System::DateTime,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Security::SecureRandom,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Ocsp::BasicOcspResp>,
                5usize,
            >("Generate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "Generate", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Ocsp::BasicOcspResp,
        > = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (signingAlgorithm, privateKey, chain, producedAt, random),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_AsymmetricKeyParameter1(
        publicKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (publicKey))?;
        Ok(__cordl_object.into())
    }
    pub fn New_RespID0(
        responderID: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Ocsp::RespID>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (responderID))?;
        Ok(__cordl_object.into())
    }
    pub fn SetResponseExtensions(
        &mut self,
        responseExtensions: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetResponseExtensions")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "SetResponseExtensions",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (responseExtensions))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_AsymmetricKeyParameter1(
        &mut self,
        publicKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (publicKey))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_RespID0(
        &mut self,
        responderID: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Ocsp::RespID>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Ocsp::RespID>),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (responderID))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_SignatureAlgNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
                0usize,
            >("get_SignatureAlgNames")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "get_SignatureAlgNames",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerable,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub certId: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Ocsp::CertificateID>,
    pub certStatus: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Ocsp::CertStatus,
    >,
    pub thisUpdate: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
    >,
    pub nextUpdate: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
    >,
    pub extensions: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+BasicOcspRespGenerator+ResponseObject")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator_ResponseObject {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Ocsp";
    const CLASS_NAME: &'static str = "BasicOcspRespGenerator/ResponseObject";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+BasicOcspRespGenerator+ResponseObject")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator_ResponseObject {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        certId: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Ocsp::CertificateID>,
        certStatus: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Ocsp::CertificateStatus,
        >,
        thisUpdate: crate::System::DateTime,
        nextUpdate: crate::System::DateTime,
        extensions: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (certId, certStatus, thisUpdate, nextUpdate, extensions),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_DateTime_X509Extensions0(
        certId: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Ocsp::CertificateID>,
        certStatus: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Ocsp::CertificateStatus,
        >,
        thisUpdate: crate::System::DateTime,
        extensions: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (certId, certStatus, thisUpdate, extensions))?;
        Ok(__cordl_object.into())
    }
    pub fn New_DerGeneralizedTime_DerGeneralizedTime_X509Extensions2(
        certId: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Ocsp::CertificateID>,
        certStatus: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Ocsp::CertificateStatus,
        >,
        thisUpdate: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
        >,
        nextUpdate: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
        >,
        extensions: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (certId, certStatus, thisUpdate, nextUpdate, extensions),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn ToResponse(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Ocsp::SingleResponse>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator_ResponseObject as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::Ocsp::SingleResponse,
                >,
                0usize,
            >("ToResponse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator_ResponseObject as
                    quest_hook::libil2cpp::Type > ::class(), "ToResponse", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Ocsp::SingleResponse,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DateTime_DateTime_X509Extensions1(
        &mut self,
        certId: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Ocsp::CertificateID>,
        certStatus: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Ocsp::CertificateStatus,
        >,
        thisUpdate: crate::System::DateTime,
        nextUpdate: crate::System::DateTime,
        extensions: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator_ResponseObject as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Ocsp::CertificateID,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Ocsp::CertificateStatus,
                    >,
                    crate::System::DateTime,
                    crate::System::DateTime,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator_ResponseObject as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (certId, certStatus, thisUpdate, nextUpdate, extensions),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DateTime_X509Extensions0(
        &mut self,
        certId: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Ocsp::CertificateID>,
        certStatus: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Ocsp::CertificateStatus,
        >,
        thisUpdate: crate::System::DateTime,
        extensions: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator_ResponseObject as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Ocsp::CertificateID,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Ocsp::CertificateStatus,
                    >,
                    crate::System::DateTime,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator_ResponseObject as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (certId, certStatus, thisUpdate, extensions))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DerGeneralizedTime_DerGeneralizedTime_X509Extensions2(
        &mut self,
        certId: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Ocsp::CertificateID>,
        certStatus: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Ocsp::CertificateStatus,
        >,
        thisUpdate: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
        >,
        nextUpdate: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
        >,
        extensions: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator_ResponseObject as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Ocsp::CertificateID,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Ocsp::CertificateStatus,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::Org::BouncyCastle::Ocsp::BasicOcspRespGenerator_ResponseObject as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (certId, certStatus, thisUpdate, nextUpdate, extensions),
                )?
        };
        Ok(__cordl_ret.into())
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
