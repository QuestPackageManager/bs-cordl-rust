#[cfg(feature = "Org+BouncyCastle+Asn1+X509+UserNotice")]
#[repr(C)]
#[derive(Debug)]
pub struct UserNotice {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub noticeRef: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::NoticeReference,
    >,
    pub explicitText: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::DisplayText,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+UserNotice")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::X509::UserNotice =>
    "Org.BouncyCastle.Asn1.X509"."UserNotice"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+UserNotice")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::X509::UserNotice {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+UserNotice")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::X509::UserNotice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+UserNotice")]
impl crate::Org::BouncyCastle::Asn1::X509::UserNotice {
    pub fn GetInstance(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::UserNotice>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::UserNotice,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetInstance", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Asn1Sequence2(
        seq: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Sequence>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object.into())
    }
    pub fn New_NoticeReference_DisplayText0(
        noticeRef: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::NoticeReference,
        >,
        explicitText: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::DisplayText,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (noticeRef, explicitText))?;
        Ok(__cordl_object.into())
    }
    pub fn New_NoticeReference_Il2CppString1(
        noticeRef: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::NoticeReference,
        >,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (noticeRef, str))?;
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
    pub fn _ctor_NoticeReference_DisplayText0(
        &mut self,
        noticeRef: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::NoticeReference,
        >,
        explicitText: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::DisplayText,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (noticeRef, explicitText))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_NoticeReference_Il2CppString1(
        &mut self,
        noticeRef: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::NoticeReference,
        >,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (noticeRef, str))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ExplicitText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::DisplayText>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::DisplayText,
        > = __cordl_object.invoke("get_ExplicitText", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NoticeRef(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::NoticeReference>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::NoticeReference,
        > = __cordl_object.invoke("get_NoticeRef", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+UserNotice")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::X509::UserNotice {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
