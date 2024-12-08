#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiHeader")]
#[repr(C)]
#[derive(Debug)]
pub struct PkiHeader {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiHeader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Cmp::PkiHeader =>
    "Org.BouncyCastle.Asn1.Cmp"."PkiHeader"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiHeader")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiHeader")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiHeader")]
impl crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader {
    pub fn get_SenderKID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString = __cordl_object
            .invoke("get_SenderKID", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetGeneralInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::Cmp::InfoTypeAndValue,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::Cmp::InfoTypeAndValue,
        > = __cordl_object.invoke("GetGeneralInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn ToAsn1Object(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::Asn1Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Object = __cordl_object
            .invoke("ToAsn1Object", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Recipient(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName = __cordl_object
            .invoke("get_Recipient", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TransactionID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString = __cordl_object
            .invoke("get_TransactionID", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Sender(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName = __cordl_object
            .invoke("get_Sender", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MessageTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime = __cordl_object
            .invoke("get_MessageTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Pvno(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::DerInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerInteger = __cordl_object
            .invoke("get_Pvno", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SenderNonce(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString = __cordl_object
            .invoke("get_SenderNonce", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ProtectionAlg(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier = __cordl_object
            .invoke("get_ProtectionAlg", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_RecipKID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString = __cordl_object
            .invoke("get_RecipKID", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Asn1Sequence0(
        &mut self,
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (seq))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_GeneralName_GeneralName1(
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
    pub fn _ctor_DerInteger_GeneralName_GeneralName2(
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
    pub fn get_RecipNonce(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString = __cordl_object
            .invoke("get_RecipNonce", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_FreeText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiFreeText,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiFreeText = __cordl_object
            .invoke("get_FreeText", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_Asn1Sequence0(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_GeneralName_GeneralName1(
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
    pub fn New_DerInteger_GeneralName_GeneralName2(
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiHeader")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
