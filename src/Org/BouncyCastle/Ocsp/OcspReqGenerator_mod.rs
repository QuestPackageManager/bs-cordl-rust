#[cfg(feature = "Org+BouncyCastle+Ocsp+OcspReqGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct OcspReqGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub list: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    pub requestorName: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    >,
    pub requestExtensions: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+OcspReqGenerator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Ocsp::OcspReqGenerator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Ocsp";
    const CLASS_NAME: &'static str = "OcspReqGenerator";
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
#[cfg(feature = "Org+BouncyCastle+Ocsp+OcspReqGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Ocsp::OcspReqGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn AddRequest_CertificateID0(
        &mut self,
        certId: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Ocsp::CertificateID>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddRequest", (certId))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddRequest_X509Extensions1(
        &mut self,
        certId: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Ocsp::CertificateID>,
        singleRequestExtensions: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddRequest", (certId, singleRequestExtensions))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateRequest(
        &mut self,
        signingAlgorithm: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
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
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Ocsp::OcspReq>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Ocsp::OcspReq,
        > = __cordl_object
            .invoke("GenerateRequest", (signingAlgorithm, privateKey, chain, random))?;
        Ok(__cordl_ret.into())
    }
    pub fn Generate_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Ocsp::OcspReq>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Ocsp::OcspReq,
        > = __cordl_object.invoke("Generate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Generate_Il2CppString_AsymmetricKeyParameter_Il2CppArray1(
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
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Ocsp::OcspReq>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Ocsp::OcspReq,
        > = __cordl_object.invoke("Generate", (signingAlgorithm, privateKey, chain))?;
        Ok(__cordl_ret.into())
    }
    pub fn Generate_Il2CppString_AsymmetricKeyParameter_Il2CppArray_SecureRandom2(
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
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Ocsp::OcspReq>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Ocsp::OcspReq,
        > = __cordl_object
            .invoke("Generate", (signingAlgorithm, privateKey, chain, random))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetRequestExtensions(
        &mut self,
        requestExtensions: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetRequestExtensions", (requestExtensions))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetRequestorName_GeneralName1(
        &mut self,
        requestorName: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::GeneralName,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetRequestorName", (requestorName))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetRequestorName_X509Name0(
        &mut self,
        requestorName: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509Name,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetRequestorName", (requestorName))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SignatureAlgNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerable,
        > = __cordl_object.invoke("get_SignatureAlgNames", ())?;
        Ok(__cordl_ret.into())
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub certId: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Ocsp::CertificateID>,
    pub extensions: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+OcspReqGenerator+RequestObject")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Ocsp::OcspReqGenerator_RequestObject {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Ocsp";
    const CLASS_NAME: &'static str = "RequestObject";
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
#[cfg(feature = "Org+BouncyCastle+Ocsp+OcspReqGenerator+RequestObject")]
impl std::ops::Deref for crate::Org::BouncyCastle::Ocsp::OcspReqGenerator_RequestObject {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn New(
        certId: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Ocsp::CertificateID>,
        extensions: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (certId, extensions))?;
        Ok(__cordl_object.into())
    }
    pub fn ToRequest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Ocsp::Request>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Ocsp::Request,
        > = __cordl_object.invoke("ToRequest", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        certId: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Ocsp::CertificateID>,
        extensions: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (certId, extensions))?;
        Ok(__cordl_ret.into())
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
