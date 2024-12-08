#[cfg(feature = "Mono+Security+Authenticode+AuthenticodeDeformatter")]
#[repr(C)]
#[derive(Debug)]
pub struct AuthenticodeDeformatter {
    __cordl_parent: crate::Mono::Security::Authenticode::AuthenticodeBase,
    pub filename: *mut crate::System::String,
    pub rawdata: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub hash: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub coll: *mut crate::Mono::Security::X509::X509CertificateCollection,
    pub signedHash: *mut crate::Mono::Security::ASN1,
    pub timestamp: crate::System::DateTime,
    pub signingCertificate: *mut crate::Mono::Security::X509::X509Certificate,
    pub reason: i32,
    pub trustedRoot: bool,
    pub trustedTimestampRoot: bool,
    pub entry: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub signerChain: *mut crate::Mono::Security::X509::X509Chain,
    pub timestampChain: *mut crate::Mono::Security::X509::X509Chain,
}
#[cfg(feature = "Mono+Security+Authenticode+AuthenticodeDeformatter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Security::Authenticode::AuthenticodeDeformatter =>
    "Mono.Security.Authenticode"."AuthenticodeDeformatter"
);
#[cfg(feature = "Mono+Security+Authenticode+AuthenticodeDeformatter")]
impl std::ops::Deref for crate::Mono::Security::Authenticode::AuthenticodeDeformatter {
    type Target = crate::Mono::Security::Authenticode::AuthenticodeBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Authenticode+AuthenticodeDeformatter")]
impl std::ops::DerefMut
for crate::Mono::Security::Authenticode::AuthenticodeDeformatter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Authenticode+AuthenticodeDeformatter")]
impl crate::Mono::Security::Authenticode::AuthenticodeDeformatter {
    pub fn set_RawData(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_RawData", (value))?;
        Ok(__cordl_ret)
    }
    pub fn CompareIssuerSerial(
        &mut self,
        issuer: *mut crate::System::String,
        serial: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        x509: *mut crate::Mono::Security::X509::X509Certificate,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CompareIssuerSerial", (issuer, serial, x509))?;
        Ok(__cordl_ret)
    }
    pub fn CheckSignature(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CheckSignature", ())?;
        Ok(__cordl_ret)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn VerifyCounterSignature(
        &mut self,
        cs: *mut crate::Mono::Security::PKCS7_SignerInfo,
        signature: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("VerifyCounterSignature", (cs, signature))?;
        Ok(__cordl_ret)
    }
    pub fn get_SigningCertificate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Mono::Security::X509::X509Certificate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Security::X509::X509Certificate = __cordl_object
            .invoke("get_SigningCertificate", ())?;
        Ok(__cordl_ret)
    }
    pub fn VerifySignature(
        &mut self,
        sd: *mut crate::Mono::Security::PKCS7_SignedData,
        calculatedMessageDigest: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        ha: *mut crate::System::Security::Cryptography::HashAlgorithm,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("VerifySignature", (sd, calculatedMessageDigest, ha))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray1(
        &mut self,
        rawData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (rawData))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray1(
        rawData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (rawData))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Mono+Security+Authenticode+AuthenticodeDeformatter")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Security::Authenticode::AuthenticodeDeformatter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
