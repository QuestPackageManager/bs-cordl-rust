#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiHeaderBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct PkiHeaderBuilder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub pvno: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerInteger>,
    pub sender: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    >,
    pub recipient: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    >,
    pub messageTime: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
    >,
    pub protectionAlg: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    >,
    pub senderKID: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    >,
    pub recipKID: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    >,
    pub transactionID: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    >,
    pub senderNonce: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    >,
    pub recipNonce: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    >,
    pub freeText: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Cmp::PkiFreeText,
    >,
    pub generalInfo: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiHeaderBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder
    => "Org.BouncyCastle.Asn1.Cmp"."PkiHeaderBuilder"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiHeaderBuilder")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiHeaderBuilder")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiHeaderBuilder")]
impl crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder {
    pub fn AddOptional(
        &mut self,
        v: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1EncodableVector,
        >,
        tagNo: i32,
        obj: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Encodable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddOptional", (v, tagNo, obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn Build(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader,
        > = __cordl_object.invoke("Build", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeGeneralInfoSeq_Il2CppArray1(
        generalInfos: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Asn1::Cmp::InfoTypeAndValue,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Sequence>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Sequence,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeGeneralInfoSeq", (generalInfos))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeGeneralInfoSeq_InfoTypeAndValue0(
        generalInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::InfoTypeAndValue,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Sequence>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Sequence,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeGeneralInfoSeq", (generalInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_DerInteger1(
        pvno: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerInteger>,
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
    pub fn New_i32_0(
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
    pub fn SetFreeText(
        &mut self,
        text: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiFreeText>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
        > = __cordl_object.invoke("SetFreeText", (text))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGeneralInfo_Asn1Sequence2(
        &mut self,
        seqOfInfoTypeAndValue: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Sequence,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
        > = __cordl_object.invoke("SetGeneralInfo", (seqOfInfoTypeAndValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGeneralInfo_Il2CppArray1(
        &mut self,
        genInfos: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Asn1::Cmp::InfoTypeAndValue,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
        > = __cordl_object.invoke("SetGeneralInfo", (genInfos))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGeneralInfo_InfoTypeAndValue0(
        &mut self,
        genInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::InfoTypeAndValue,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
        > = __cordl_object.invoke("SetGeneralInfo", (genInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMessageTime(
        &mut self,
        _cordl_time: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
        > = __cordl_object.invoke("SetMessageTime", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetProtectionAlg(
        &mut self,
        aid: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
        > = __cordl_object.invoke("SetProtectionAlg", (aid))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetRecipKID_Asn1OctetString1(
        &mut self,
        kid: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1OctetString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
        > = __cordl_object.invoke("SetRecipKID", (kid))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetRecipKID_Il2CppArray0(
        &mut self,
        kid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
        > = __cordl_object.invoke("SetRecipKID", (kid))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetRecipNonce_Asn1OctetString1(
        &mut self,
        nonce: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1OctetString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
        > = __cordl_object.invoke("SetRecipNonce", (nonce))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetRecipNonce_Il2CppArray0(
        &mut self,
        nonce: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
        > = __cordl_object.invoke("SetRecipNonce", (nonce))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSenderKID_Asn1OctetString1(
        &mut self,
        kid: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1OctetString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
        > = __cordl_object.invoke("SetSenderKID", (kid))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSenderKID_Il2CppArray0(
        &mut self,
        kid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
        > = __cordl_object.invoke("SetSenderKID", (kid))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSenderNonce_Asn1OctetString1(
        &mut self,
        nonce: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1OctetString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
        > = __cordl_object.invoke("SetSenderNonce", (nonce))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSenderNonce_Il2CppArray0(
        &mut self,
        nonce: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
        > = __cordl_object.invoke("SetSenderNonce", (nonce))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTransactionID_Asn1OctetString1(
        &mut self,
        tid: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1OctetString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
        > = __cordl_object.invoke("SetTransactionID", (tid))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTransactionID_Il2CppArray0(
        &mut self,
        tid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
        > = __cordl_object.invoke("SetTransactionID", (tid))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DerInteger1(
        &mut self,
        pvno: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerInteger>,
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
    pub fn _ctor_i32_0(
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiHeaderBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
