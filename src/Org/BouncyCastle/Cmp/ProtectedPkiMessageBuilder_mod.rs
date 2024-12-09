#[cfg(feature = "Org+BouncyCastle+Cmp+ProtectedPkiMessageBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct ProtectedPkiMessageBuilder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub hdrBuilBuilder: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
    pub body: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiBody,
    pub generalInfos: *mut crate::System::Collections::IList,
    pub extraCerts: *mut crate::System::Collections::IList,
}
#[cfg(feature = "Org+BouncyCastle+Cmp+ProtectedPkiMessageBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder => "Org.BouncyCastle.Cmp"
    ."ProtectedPkiMessageBuilder"
);
#[cfg(feature = "Org+BouncyCastle+Cmp+ProtectedPkiMessageBuilder")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cmp+ProtectedPkiMessageBuilder")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cmp+ProtectedPkiMessageBuilder")]
impl crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder {
    pub fn AddCmpCertificate(
        &mut self,
        certificate: *mut crate::Org::BouncyCastle::X509::X509Certificate,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder = __cordl_object
            .invoke("AddCmpCertificate", (certificate))?;
        Ok(__cordl_ret)
    }
    pub fn AddGeneralInfo(
        &mut self,
        genInfo: *mut crate::Org::BouncyCastle::Asn1::Cmp::InfoTypeAndValue,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder = __cordl_object
            .invoke("AddGeneralInfo", (genInfo))?;
        Ok(__cordl_ret)
    }
    pub fn Build_IMacFactory1(
        &mut self,
        factory: *mut crate::Org::BouncyCastle::Crypto::IMacFactory,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cmp::ProtectedPkiMessage,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cmp::ProtectedPkiMessage = __cordl_object
            .invoke("Build", (factory))?;
        Ok(__cordl_ret)
    }
    pub fn Build_ISignatureFactory0(
        &mut self,
        signatureFactory: *mut crate::Org::BouncyCastle::Crypto::ISignatureFactory,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cmp::ProtectedPkiMessage,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cmp::ProtectedPkiMessage = __cordl_object
            .invoke("Build", (signatureFactory))?;
        Ok(__cordl_ret)
    }
    pub fn CalculateSignature(
        &mut self,
        signer: *mut crate::Org::BouncyCastle::Crypto::IStreamCalculator,
        header: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader,
        body: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiBody,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("CalculateSignature", (signer, header, body))?;
        Ok(__cordl_ret)
    }
    pub fn FinalizeHeader(
        &mut self,
        algorithmIdentifier: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinalizeHeader", (algorithmIdentifier))?;
        Ok(__cordl_ret)
    }
    pub fn FinalizeMessage(
        &mut self,
        header: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader,
        protection: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cmp::ProtectedPkiMessage,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cmp::ProtectedPkiMessage = __cordl_object
            .invoke("FinalizeMessage", (header, protection))?;
        Ok(__cordl_ret)
    }
    pub fn New_GeneralName0(
        sender: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
        recipient: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sender, recipient))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_GeneralName1(
        pvno: i32,
        sender: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
        recipient: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pvno, sender, recipient))?;
        Ok(__cordl_object)
    }
    pub fn SetBody(
        &mut self,
        body: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiBody,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder = __cordl_object
            .invoke("SetBody", (body))?;
        Ok(__cordl_ret)
    }
    pub fn SetFreeText(
        &mut self,
        freeText: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiFreeText,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder = __cordl_object
            .invoke("SetFreeText", (freeText))?;
        Ok(__cordl_ret)
    }
    pub fn SetMessageTime(
        &mut self,
        generalizedTime: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder = __cordl_object
            .invoke("SetMessageTime", (generalizedTime))?;
        Ok(__cordl_ret)
    }
    pub fn SetRecipKID(
        &mut self,
        id: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder = __cordl_object
            .invoke("SetRecipKID", (id))?;
        Ok(__cordl_ret)
    }
    pub fn SetRecipNonce(
        &mut self,
        nonce: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder = __cordl_object
            .invoke("SetRecipNonce", (nonce))?;
        Ok(__cordl_ret)
    }
    pub fn SetSenderKID(
        &mut self,
        id: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder = __cordl_object
            .invoke("SetSenderKID", (id))?;
        Ok(__cordl_ret)
    }
    pub fn SetSenderNonce(
        &mut self,
        nonce: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder = __cordl_object
            .invoke("SetSenderNonce", (nonce))?;
        Ok(__cordl_ret)
    }
    pub fn SetTransactionId(
        &mut self,
        tid: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder = __cordl_object
            .invoke("SetTransactionId", (tid))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_GeneralName0(
        &mut self,
        sender: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
        recipient: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sender, recipient))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_GeneralName1(
        &mut self,
        pvno: i32,
        sender: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
        recipient: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pvno, sender, recipient))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cmp+ProtectedPkiMessageBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
