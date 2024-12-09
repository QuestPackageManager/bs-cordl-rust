#[cfg(feature = "Org+BouncyCastle+Crmf+CertificateRequestMessageBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct CertificateRequestMessageBuilder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _certReqId: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub _extGenerator: *mut crate::Org::BouncyCastle::Asn1::X509::X509ExtensionsGenerator,
    pub _templateBuilder: *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplateBuilder,
    pub _controls: *mut crate::System::Collections::IList,
    pub _popSigner: *mut crate::Org::BouncyCastle::Crypto::ISignatureFactory,
    pub _pkMacBuilder: *mut crate::Org::BouncyCastle::Crmf::PKMacBuilder,
    pub _password: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    pub _sender: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    pub _popoType: i32,
    pub _popoPrivKey: *mut crate::Org::BouncyCastle::Asn1::Crmf::PopoPrivKey,
    pub _popRaVerified: *mut crate::Org::BouncyCastle::Asn1::Asn1Null,
    pub _agreeMac: *mut crate::Org::BouncyCastle::Asn1::Crmf::PKMacValue,
}
#[cfg(feature = "Org+BouncyCastle+Crmf+CertificateRequestMessageBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder =>
    "Org.BouncyCastle.Crmf"."CertificateRequestMessageBuilder"
);
#[cfg(feature = "Org+BouncyCastle+Crmf+CertificateRequestMessageBuilder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+CertificateRequestMessageBuilder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+CertificateRequestMessageBuilder")]
impl crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder {
    pub fn AddControl(
        &mut self,
        control: *mut crate::Org::BouncyCastle::Crmf::IControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder = __cordl_object
            .invoke("AddControl", (control))?;
        Ok(__cordl_ret)
    }
    pub fn AddExtension_Asn1Encodable0(
        &mut self,
        oid: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        critical: bool,
        value: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder = __cordl_object
            .invoke("AddExtension", (oid, critical, value))?;
        Ok(__cordl_ret)
    }
    pub fn AddExtension_Il2CppArray1(
        &mut self,
        oid: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        critical: bool,
        value: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder = __cordl_object
            .invoke("AddExtension", (oid, critical, value))?;
        Ok(__cordl_ret)
    }
    pub fn Build(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessage,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessage = __cordl_object
            .invoke("Build", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        certReqId: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (certReqId))?;
        Ok(__cordl_object)
    }
    pub fn SetAuthInfoPKMAC(
        &mut self,
        pkmacFactory: *mut crate::Org::BouncyCastle::Crmf::PKMacBuilder,
        password: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder = __cordl_object
            .invoke("SetAuthInfoPKMAC", (pkmacFactory, password))?;
        Ok(__cordl_ret)
    }
    pub fn SetAuthInfoSender_GeneralName1(
        &mut self,
        sender: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder = __cordl_object
            .invoke("SetAuthInfoSender", (sender))?;
        Ok(__cordl_ret)
    }
    pub fn SetAuthInfoSender_X509Name0(
        &mut self,
        sender: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder = __cordl_object
            .invoke("SetAuthInfoSender", (sender))?;
        Ok(__cordl_ret)
    }
    pub fn SetIssuer(
        &mut self,
        issuer: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder = __cordl_object
            .invoke("SetIssuer", (issuer))?;
        Ok(__cordl_ret)
    }
    pub fn SetProofOfPossessionAgreeMac(
        &mut self,
        macValue: *mut crate::Org::BouncyCastle::Asn1::Crmf::PKMacValue,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder = __cordl_object
            .invoke("SetProofOfPossessionAgreeMac", (macValue))?;
        Ok(__cordl_ret)
    }
    pub fn SetProofOfPossessionRaVerified(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder = __cordl_object
            .invoke("SetProofOfPossessionRaVerified", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetProofOfPossessionSignKeySigner(
        &mut self,
        popoSignatureFactory: *mut crate::Org::BouncyCastle::Crypto::ISignatureFactory,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder = __cordl_object
            .invoke("SetProofOfPossessionSignKeySigner", (popoSignatureFactory))?;
        Ok(__cordl_ret)
    }
    pub fn SetProofOfPossessionSubsequentMessage_SubsequentMessage0(
        &mut self,
        msg: *mut crate::Org::BouncyCastle::Asn1::Crmf::SubsequentMessage,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder = __cordl_object
            .invoke("SetProofOfPossessionSubsequentMessage", (msg))?;
        Ok(__cordl_ret)
    }
    pub fn SetProofOfPossessionSubsequentMessage_i32_SubsequentMessage1(
        &mut self,
        _cordl_type: i32,
        msg: *mut crate::Org::BouncyCastle::Asn1::Crmf::SubsequentMessage,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder = __cordl_object
            .invoke("SetProofOfPossessionSubsequentMessage", (_cordl_type, msg))?;
        Ok(__cordl_ret)
    }
    pub fn SetPublicKey(
        &mut self,
        publicKeyInfo: *mut crate::Org::BouncyCastle::Asn1::X509::SubjectPublicKeyInfo,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder = __cordl_object
            .invoke("SetPublicKey", (publicKeyInfo))?;
        Ok(__cordl_ret)
    }
    pub fn SetSerialNumber(
        &mut self,
        serialNumber: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder = __cordl_object
            .invoke("SetSerialNumber", (serialNumber))?;
        Ok(__cordl_ret)
    }
    pub fn SetSubject(
        &mut self,
        subject: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder = __cordl_object
            .invoke("SetSubject", (subject))?;
        Ok(__cordl_ret)
    }
    pub fn SetValidity(
        &mut self,
        notBefore: *mut crate::Org::BouncyCastle::Asn1::X509::Time,
        notAfter: *mut crate::Org::BouncyCastle::Asn1::X509::Time,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder = __cordl_object
            .invoke("SetValidity", (notBefore, notAfter))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        certReqId: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (certReqId))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+CertificateRequestMessageBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crmf::CertificateRequestMessageBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
