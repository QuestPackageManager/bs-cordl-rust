#[cfg(feature = "Org+BouncyCastle+Cmp+ProtectedPkiMessageBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct ProtectedPkiMessageBuilder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub hdrBuilBuilder: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
    >,
    pub body: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiBody>,
    pub generalInfos: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    pub extraCerts: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
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
        certificate: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Certificate,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder,
        > = __cordl_object.invoke("AddCmpCertificate", (certificate))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddGeneralInfo(
        &mut self,
        genInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::InfoTypeAndValue,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder,
        > = __cordl_object.invoke("AddGeneralInfo", (genInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn Build_IMacFactory1(
        &mut self,
        factory: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IMacFactory>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cmp::ProtectedPkiMessage>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::ProtectedPkiMessage,
        > = __cordl_object.invoke("Build", (factory))?;
        Ok(__cordl_ret.into())
    }
    pub fn Build_ISignatureFactory0(
        &mut self,
        signatureFactory: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ISignatureFactory,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cmp::ProtectedPkiMessage>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::ProtectedPkiMessage,
        > = __cordl_object.invoke("Build", (signatureFactory))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateSignature(
        &mut self,
        signer: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IStreamCalculator,
        >,
        header: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader,
        >,
        body: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiBody>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("CalculateSignature", (signer, header, body))?;
        Ok(__cordl_ret.into())
    }
    pub fn FinalizeHeader(
        &mut self,
        algorithmIdentifier: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinalizeHeader", (algorithmIdentifier))?;
        Ok(__cordl_ret.into())
    }
    pub fn FinalizeMessage(
        &mut self,
        header: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader,
        >,
        protection: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerBitString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cmp::ProtectedPkiMessage>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::ProtectedPkiMessage,
        > = __cordl_object.invoke("FinalizeMessage", (header, protection))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_GeneralName0(
        sender: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::GeneralName,
        >,
        recipient: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::GeneralName,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sender, recipient))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_GeneralName1(
        pvno: i32,
        sender: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::GeneralName,
        >,
        recipient: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::GeneralName,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pvno, sender, recipient))?;
        Ok(__cordl_object.into())
    }
    pub fn SetBody(
        &mut self,
        body: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiBody>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder,
        > = __cordl_object.invoke("SetBody", (body))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetFreeText(
        &mut self,
        freeText: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiFreeText,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder,
        > = __cordl_object.invoke("SetFreeText", (freeText))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMessageTime(
        &mut self,
        generalizedTime: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder,
        > = __cordl_object.invoke("SetMessageTime", (generalizedTime))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetRecipKID(
        &mut self,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder,
        > = __cordl_object.invoke("SetRecipKID", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetRecipNonce(
        &mut self,
        nonce: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder,
        > = __cordl_object.invoke("SetRecipNonce", (nonce))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSenderKID(
        &mut self,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder,
        > = __cordl_object.invoke("SetSenderKID", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSenderNonce(
        &mut self,
        nonce: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder,
        > = __cordl_object.invoke("SetSenderNonce", (nonce))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTransactionId(
        &mut self,
        tid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::ProtectedPkiMessageBuilder,
        > = __cordl_object.invoke("SetTransactionId", (tid))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_GeneralName0(
        &mut self,
        sender: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::GeneralName,
        >,
        recipient: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::GeneralName,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sender, recipient))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_GeneralName1(
        &mut self,
        pvno: i32,
        sender: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::GeneralName,
        >,
        recipient: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::GeneralName,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pvno, sender, recipient))?;
        Ok(__cordl_ret.into())
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
