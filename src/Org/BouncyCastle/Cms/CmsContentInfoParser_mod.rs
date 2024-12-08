#[cfg(feature = "Org+BouncyCastle+Cms+CmsContentInfoParser")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsContentInfoParser {
    __cordl_parent: crate::System::Object,
    pub contentInfo: *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfoParser,
    pub data: *mut crate::System::IO::Stream,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsContentInfoParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Cms::CmsContentInfoParser =>
    "Org.BouncyCastle.Cms"."CmsContentInfoParser"
);
#[cfg(feature = "Org+BouncyCastle+Cms+CmsContentInfoParser")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::CmsContentInfoParser {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsContentInfoParser")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::CmsContentInfoParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsContentInfoParser")]
impl crate::Org::BouncyCastle::Cms::CmsContentInfoParser {
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        data: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (data))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        data: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (data))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsContentInfoParser")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsContentInfoParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
