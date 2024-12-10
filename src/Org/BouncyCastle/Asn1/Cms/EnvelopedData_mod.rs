#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+EnvelopedData")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvelopedData {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub version: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    pub originatorInfo: *mut crate::Org::BouncyCastle::Asn1::Cms::OriginatorInfo,
    pub recipientInfos: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    pub encryptedContentInfo: *mut crate::Org::BouncyCastle::Asn1::Cms::EncryptedContentInfo,
    pub unprotectedAttrs: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+EnvelopedData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Cms::EnvelopedData =>
    "Org.BouncyCastle.Asn1.Cms"."EnvelopedData"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+EnvelopedData")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Cms::EnvelopedData {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+EnvelopedData")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Cms::EnvelopedData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+EnvelopedData")]
impl crate::Org::BouncyCastle::Asn1::Cms::EnvelopedData {
    pub fn New_Asn1Sequence2(
        seq: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Sequence>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object.into())
    }
    pub fn New_OriginatorInfo_Asn1Set_EncryptedContentInfo_Asn1Set0(
        originatorInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::OriginatorInfo,
        >,
        recipientInfos: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Set,
        >,
        encryptedContentInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::EncryptedContentInfo,
        >,
        unprotectedAttrs: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Set,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (originatorInfo, recipientInfos, encryptedContentInfo, unprotectedAttrs),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_OriginatorInfo_Asn1Set_EncryptedContentInfo_Attributes1(
        originatorInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::OriginatorInfo,
        >,
        recipientInfos: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Set,
        >,
        encryptedContentInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::EncryptedContentInfo,
        >,
        unprotectedAttrs: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::Attributes,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (originatorInfo, recipientInfos, encryptedContentInfo, unprotectedAttrs),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn ToAsn1Object(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Object,
        > = __cordl_object.invoke("ToAsn1Object", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Asn1Sequence2(
        &mut self,
        seq: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Sequence>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (seq))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_OriginatorInfo_Asn1Set_EncryptedContentInfo_Asn1Set0(
        &mut self,
        originatorInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::OriginatorInfo,
        >,
        recipientInfos: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Set,
        >,
        encryptedContentInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::EncryptedContentInfo,
        >,
        unprotectedAttrs: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Set,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (originatorInfo, recipientInfos, encryptedContentInfo, unprotectedAttrs),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_OriginatorInfo_Asn1Set_EncryptedContentInfo_Attributes1(
        &mut self,
        originatorInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::OriginatorInfo,
        >,
        recipientInfos: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Set,
        >,
        encryptedContentInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::EncryptedContentInfo,
        >,
        unprotectedAttrs: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::Attributes,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (originatorInfo, recipientInfos, encryptedContentInfo, unprotectedAttrs),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_EncryptedContentInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::EncryptedContentInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::EncryptedContentInfo,
        > = __cordl_object.invoke("get_EncryptedContentInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OriginatorInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cms::OriginatorInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::OriginatorInfo,
        > = __cordl_object.invoke("get_OriginatorInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RecipientInfos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Set>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Set,
        > = __cordl_object.invoke("get_RecipientInfos", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UnprotectedAttrs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Set>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Set,
        > = __cordl_object.invoke("get_UnprotectedAttrs", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Version(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerInteger,
        > = __cordl_object.invoke("get_Version", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+EnvelopedData")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Cms::EnvelopedData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
