#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+MetaData")]
#[repr(C)]
#[derive(Debug)]
pub struct MetaData {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub hashProtected: *mut crate::Org::BouncyCastle::Asn1::DerBoolean,
    pub fileName: *mut crate::Org::BouncyCastle::Asn1::DerUtf8String,
    pub mediaType: *mut crate::Org::BouncyCastle::Asn1::DerIA5String,
    pub otherMetaData: *mut crate::Org::BouncyCastle::Asn1::Cms::Attributes,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+MetaData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Cms::MetaData =>
    "Org.BouncyCastle.Asn1.Cms"."MetaData"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+MetaData")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Cms::MetaData {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+MetaData")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Cms::MetaData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+MetaData")]
impl crate::Org::BouncyCastle::Asn1::Cms::MetaData {
    pub fn get_MediaType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::DerIA5String,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerIA5String = __cordl_object
            .invoke("get_MediaType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_FileName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::DerUtf8String,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerUtf8String = __cordl_object
            .invoke("get_FileName", ())?;
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
    pub fn get_OtherMetaData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cms::Attributes,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cms::Attributes = __cordl_object
            .invoke("get_OtherMetaData", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DerBoolean_DerUtf8String_DerIA5String_Attributes0(
        &mut self,
        hashProtected: *mut crate::Org::BouncyCastle::Asn1::DerBoolean,
        fileName: *mut crate::Org::BouncyCastle::Asn1::DerUtf8String,
        mediaType: *mut crate::Org::BouncyCastle::Asn1::DerIA5String,
        otherMetaData: *mut crate::Org::BouncyCastle::Asn1::Cms::Attributes,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (hashProtected, fileName, mediaType, otherMetaData))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Asn1Sequence1(
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
    pub fn get_IsHashProtected(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsHashProtected", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_DerBoolean_DerUtf8String_DerIA5String_Attributes0(
        hashProtected: *mut crate::Org::BouncyCastle::Asn1::DerBoolean,
        fileName: *mut crate::Org::BouncyCastle::Asn1::DerUtf8String,
        mediaType: *mut crate::Org::BouncyCastle::Asn1::DerIA5String,
        otherMetaData: *mut crate::Org::BouncyCastle::Asn1::Cms::Attributes,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (hashProtected, fileName, mediaType, otherMetaData))?;
        Ok(__cordl_object)
    }
    pub fn New_Asn1Sequence1(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+MetaData")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Cms::MetaData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
