#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiHeaderBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct PkiHeaderBuilder {
    __cordl_parent: crate::System::Object,
    pub pvno: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    pub sender: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    pub recipient: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    pub messageTime: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
    pub protectionAlg: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub senderKID: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    pub recipKID: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    pub transactionID: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    pub senderNonce: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    pub recipNonce: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    pub freeText: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiFreeText,
    pub generalInfo: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiHeaderBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder
    => "Org.BouncyCastle.Asn1.Cmp"."PkiHeaderBuilder"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiHeaderBuilder")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder {
    type Target = crate::System::Object;
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
        v: *mut crate::Org::BouncyCastle::Asn1::Asn1EncodableVector,
        tagNo: i32,
        obj: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddOptional", (v, tagNo, obj))?;
        Ok(__cordl_ret)
    }
    pub fn SetFreeText(
        &mut self,
        text: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiFreeText,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder = __cordl_object
            .invoke("SetFreeText", (text))?;
        Ok(__cordl_ret)
    }
    pub fn SetProtectionAlg(
        &mut self,
        aid: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder = __cordl_object
            .invoke("SetProtectionAlg", (aid))?;
        Ok(__cordl_ret)
    }
    pub fn SetTransactionID_Il2CppArray0(
        &mut self,
        tid: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder = __cordl_object
            .invoke("SetTransactionID", (tid))?;
        Ok(__cordl_ret)
    }
    pub fn SetTransactionID_Asn1OctetString1(
        &mut self,
        tid: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder = __cordl_object
            .invoke("SetTransactionID", (tid))?;
        Ok(__cordl_ret)
    }
    pub fn SetSenderNonce_Il2CppArray0(
        &mut self,
        nonce: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder = __cordl_object
            .invoke("SetSenderNonce", (nonce))?;
        Ok(__cordl_ret)
    }
    pub fn SetSenderNonce_Asn1OctetString1(
        &mut self,
        nonce: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder = __cordl_object
            .invoke("SetSenderNonce", (nonce))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_0(
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
    pub fn _ctor_DerInteger1(
        &mut self,
        pvno: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
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
    pub fn SetGeneralInfo_InfoTypeAndValue0(
        &mut self,
        genInfo: *mut crate::Org::BouncyCastle::Asn1::Cmp::InfoTypeAndValue,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder = __cordl_object
            .invoke("SetGeneralInfo", (genInfo))?;
        Ok(__cordl_ret)
    }
    pub fn SetGeneralInfo_Il2CppArray1(
        &mut self,
        genInfos: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::Cmp::InfoTypeAndValue,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder = __cordl_object
            .invoke("SetGeneralInfo", (genInfos))?;
        Ok(__cordl_ret)
    }
    pub fn SetGeneralInfo_Asn1Sequence2(
        &mut self,
        seqOfInfoTypeAndValue: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder = __cordl_object
            .invoke("SetGeneralInfo", (seqOfInfoTypeAndValue))?;
        Ok(__cordl_ret)
    }
    pub fn SetSenderKID_Il2CppArray0(
        &mut self,
        kid: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder = __cordl_object
            .invoke("SetSenderKID", (kid))?;
        Ok(__cordl_ret)
    }
    pub fn SetSenderKID_Asn1OctetString1(
        &mut self,
        kid: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder = __cordl_object
            .invoke("SetSenderKID", (kid))?;
        Ok(__cordl_ret)
    }
    pub fn SetMessageTime(
        &mut self,
        _cordl_time: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder = __cordl_object
            .invoke("SetMessageTime", (_cordl_time))?;
        Ok(__cordl_ret)
    }
    pub fn SetRecipKID_Il2CppArray0(
        &mut self,
        kid: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder = __cordl_object
            .invoke("SetRecipKID", (kid))?;
        Ok(__cordl_ret)
    }
    pub fn SetRecipKID_Asn1OctetString1(
        &mut self,
        kid: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder = __cordl_object
            .invoke("SetRecipKID", (kid))?;
        Ok(__cordl_ret)
    }
    pub fn SetRecipNonce_Il2CppArray0(
        &mut self,
        nonce: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder = __cordl_object
            .invoke("SetRecipNonce", (nonce))?;
        Ok(__cordl_ret)
    }
    pub fn SetRecipNonce_Asn1OctetString1(
        &mut self,
        nonce: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeaderBuilder = __cordl_object
            .invoke("SetRecipNonce", (nonce))?;
        Ok(__cordl_ret)
    }
    pub fn Build(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader = __cordl_object
            .invoke("Build", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_i32_0(
        pvno: i32,
        sender: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
        recipient: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pvno, sender, recipient))?;
        Ok(__cordl_object)
    }
    pub fn New_DerInteger1(
        pvno: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
        sender: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
        recipient: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pvno, sender, recipient))?;
        Ok(__cordl_object)
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
