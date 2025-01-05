#[cfg(feature = "Org+BouncyCastle+Asn1+X509+NoticeReference")]
#[repr(C)]
#[derive(Debug)]
pub struct NoticeReference {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub organization: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::DisplayText,
    >,
    pub noticeNumbers: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+NoticeReference")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::X509::NoticeReference
    => "Org.BouncyCastle.Asn1.X509"."NoticeReference"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+NoticeReference")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::X509::NoticeReference {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+NoticeReference")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::X509::NoticeReference {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+NoticeReference")]
impl crate::Org::BouncyCastle::Asn1::X509::NoticeReference {
    pub fn ConvertVector(
        numbers: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1EncodableVector>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1EncodableVector,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertVector", (numbers))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInstance(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::NoticeReference>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::NoticeReference,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetInstance", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNoticeNumbers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Asn1::DerInteger,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Asn1::DerInteger,
            >,
        > = __cordl_object.invoke("GetNoticeNumbers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Asn1Sequence3(
        seq: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Sequence>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object.into())
    }
    pub fn New_DisplayText_Asn1EncodableVector2(
        organization: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::DisplayText,
        >,
        noticeNumbers: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1EncodableVector,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (organization, noticeNumbers))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_Asn1EncodableVector1(
        organization: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        noticeNumbers: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1EncodableVector,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (organization, noticeNumbers))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_IList0(
        organization: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        numbers: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (organization, numbers))?;
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
    pub fn _ctor_Asn1Sequence3(
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
    pub fn _ctor_DisplayText_Asn1EncodableVector2(
        &mut self,
        organization: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::DisplayText,
        >,
        noticeNumbers: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1EncodableVector,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (organization, noticeNumbers))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_Asn1EncodableVector1(
        &mut self,
        organization: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        noticeNumbers: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1EncodableVector,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (organization, noticeNumbers))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_IList0(
        &mut self,
        organization: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        numbers: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (organization, numbers))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Organization(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::DisplayText>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::DisplayText,
        > = __cordl_object.invoke("get_Organization", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+NoticeReference")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::X509::NoticeReference {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
