#[cfg(feature = "Org+BouncyCastle+Asn1+X509+UserNotice")]
#[repr(C)]
#[derive(Debug)]
pub struct UserNotice {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub noticeRef: *mut crate::Org::BouncyCastle::Asn1::X509::NoticeReference,
    pub explicitText: *mut crate::Org::BouncyCastle::Asn1::X509::DisplayText,
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
    pub fn New_Asn1Sequence2(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
    pub fn New_NoticeReference_DisplayText0(
        noticeRef: *mut crate::Org::BouncyCastle::Asn1::X509::NoticeReference,
        explicitText: *mut crate::Org::BouncyCastle::Asn1::X509::DisplayText,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (noticeRef, explicitText))?;
        Ok(__cordl_object)
    }
    pub fn New_NoticeReference_String1(
        noticeRef: *mut crate::Org::BouncyCastle::Asn1::X509::NoticeReference,
        str: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (noticeRef, str))?;
        Ok(__cordl_object)
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
    pub fn _ctor_Asn1Sequence2(
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
    pub fn _ctor_NoticeReference_DisplayText0(
        &mut self,
        noticeRef: *mut crate::Org::BouncyCastle::Asn1::X509::NoticeReference,
        explicitText: *mut crate::Org::BouncyCastle::Asn1::X509::DisplayText,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (noticeRef, explicitText))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_NoticeReference_String1(
        &mut self,
        noticeRef: *mut crate::Org::BouncyCastle::Asn1::X509::NoticeReference,
        str: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (noticeRef, str))?;
        Ok(__cordl_ret)
    }
    pub fn get_ExplicitText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::DisplayText,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::DisplayText = __cordl_object
            .invoke("get_ExplicitText", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_NoticeRef(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::NoticeReference,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::NoticeReference = __cordl_object
            .invoke("get_NoticeRef", ())?;
        Ok(__cordl_ret)
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
