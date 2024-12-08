#[cfg(feature = "Org+BouncyCastle+Crmf+CertificateRequestMessage")]
#[repr(C)]
#[derive(Debug)]
pub struct CertificateRequestMessage {
    __cordl_parent: crate::System::Object,
    pub certReqMsg: *mut crate::Org::BouncyCastle::Asn1::Crmf::CertReqMsg,
    pub controls: *mut crate::Org::BouncyCastle::Asn1::Crmf::Controls,
}
#[cfg(feature = "Org+BouncyCastle+Crmf+CertificateRequestMessage")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crmf::CertificateRequestMessage => "Org.BouncyCastle.Crmf"
    ."CertificateRequestMessage"
);
#[cfg(feature = "Org+BouncyCastle+Crmf+CertificateRequestMessage")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crmf::CertificateRequestMessage {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+CertificateRequestMessage")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crmf::CertificateRequestMessage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+CertificateRequestMessage")]
impl crate::Org::BouncyCastle::Crmf::CertificateRequestMessage {
    pub fn FindControl(
        &mut self,
        _cordl_type: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Crmf::AttributeTypeAndValue,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Crmf::AttributeTypeAndValue = __cordl_object
            .invoke("FindControl", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn GetCertTemplate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplate = __cordl_object
            .invoke("GetCertTemplate", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetControl(
        &mut self,
        _cordl_type: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Crmf::IControl> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crmf::IControl = __cordl_object
            .invoke("GetControl", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn GetEncoded(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetEncoded", ())?;
        Ok(__cordl_ret)
    }
    pub fn HasControl(
        &mut self,
        objectIdentifier: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasControl", (objectIdentifier))?;
        Ok(__cordl_ret)
    }
    pub fn IsValidSigningKeyPop(
        &mut self,
        verifierProvider: *mut crate::Org::BouncyCastle::Crypto::IVerifierFactoryProvider,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsValidSigningKeyPop", (verifierProvider))?;
        Ok(__cordl_ret)
    }
    pub fn New_CertReqMsg1(
        certReqMsg: *mut crate::Org::BouncyCastle::Asn1::Crmf::CertReqMsg,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (certReqMsg))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray0(
        encoded: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encoded))?;
        Ok(__cordl_object)
    }
    pub fn ToAsn1Structure(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Crmf::CertReqMsg,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Crmf::CertReqMsg = __cordl_object
            .invoke("ToAsn1Structure", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_CertReqMsg1(
        &mut self,
        certReqMsg: *mut crate::Org::BouncyCastle::Asn1::Crmf::CertReqMsg,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (certReqMsg))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray0(
        &mut self,
        encoded: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (encoded))?;
        Ok(__cordl_ret)
    }
    pub fn get_HasControls(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasControls", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasProofOfPossession(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasProofOfPossession", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasSigningKeyProofOfPossessionWithPkMac(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_HasSigningKeyProofOfPossessionWithPkMac", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ProofOfPossession(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ProofOfPossession", ())?;
        Ok(__cordl_ret)
    }
    pub fn verifySignature(
        &mut self,
        verifierFactoryProvider: *mut crate::Org::BouncyCastle::Crypto::IVerifierFactoryProvider,
        signKey: *mut crate::Org::BouncyCastle::Asn1::Crmf::PopoSigningKey,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("verifySignature", (verifierFactoryProvider, signKey))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+CertificateRequestMessage")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crmf::CertificateRequestMessage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}