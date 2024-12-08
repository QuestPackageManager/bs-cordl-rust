#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+RecipientInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct RecipientInfo {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub info: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+RecipientInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Cms::RecipientInfo =>
    "Org.BouncyCastle.Asn1.Cms"."RecipientInfo"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+RecipientInfo")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Cms::RecipientInfo {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+RecipientInfo")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Cms::RecipientInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+RecipientInfo")]
impl crate::Org::BouncyCastle::Asn1::Cms::RecipientInfo {
    pub fn get_IsTagged(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsTagged", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetKekInfo(
        &mut self,
        o: *mut crate::Org::BouncyCastle::Asn1::Asn1TaggedObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cms::KekRecipientInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cms::KekRecipientInfo = __cordl_object
            .invoke("GetKekInfo", (o))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_KeyTransRecipientInfo0(
        &mut self,
        info: *mut crate::Org::BouncyCastle::Asn1::Cms::KeyTransRecipientInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_KeyAgreeRecipientInfo1(
        &mut self,
        info: *mut crate::Org::BouncyCastle::Asn1::Cms::KeyAgreeRecipientInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_KekRecipientInfo2(
        &mut self,
        info: *mut crate::Org::BouncyCastle::Asn1::Cms::KekRecipientInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_PasswordRecipientInfo3(
        &mut self,
        info: *mut crate::Org::BouncyCastle::Asn1::Cms::PasswordRecipientInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_OtherRecipientInfo4(
        &mut self,
        info: *mut crate::Org::BouncyCastle::Asn1::Cms::OtherRecipientInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Asn1Object5(
        &mut self,
        info: *mut crate::Org::BouncyCastle::Asn1::Asn1Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info))?;
        Ok(__cordl_ret)
    }
    pub fn get_Version(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::DerInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerInteger = __cordl_object
            .invoke("get_Version", ())?;
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
    pub fn get_Info(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable = __cordl_object
            .invoke("get_Info", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_KeyTransRecipientInfo0(
        info: *mut crate::Org::BouncyCastle::Asn1::Cms::KeyTransRecipientInfo,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info))?;
        Ok(__cordl_object)
    }
    pub fn New_KeyAgreeRecipientInfo1(
        info: *mut crate::Org::BouncyCastle::Asn1::Cms::KeyAgreeRecipientInfo,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info))?;
        Ok(__cordl_object)
    }
    pub fn New_KekRecipientInfo2(
        info: *mut crate::Org::BouncyCastle::Asn1::Cms::KekRecipientInfo,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info))?;
        Ok(__cordl_object)
    }
    pub fn New_PasswordRecipientInfo3(
        info: *mut crate::Org::BouncyCastle::Asn1::Cms::PasswordRecipientInfo,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info))?;
        Ok(__cordl_object)
    }
    pub fn New_OtherRecipientInfo4(
        info: *mut crate::Org::BouncyCastle::Asn1::Cms::OtherRecipientInfo,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info))?;
        Ok(__cordl_object)
    }
    pub fn New_Asn1Object5(
        info: *mut crate::Org::BouncyCastle::Asn1::Asn1Object,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+RecipientInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Cms::RecipientInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
