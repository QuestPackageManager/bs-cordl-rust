#[cfg(feature = "System+Xml+Linq+BaseUriAnnotation")]
#[repr(C)]
#[derive(Debug)]
pub struct BaseUriAnnotation {
    __cordl_parent: crate::System::Object,
    pub baseUri: *mut crate::System::String,
}
#[cfg(feature = "System+Xml+Linq+BaseUriAnnotation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Linq::BaseUriAnnotation =>
    "System.Xml.Linq"."BaseUriAnnotation"
);
#[cfg(feature = "System+Xml+Linq+BaseUriAnnotation")]
impl std::ops::Deref for crate::System::Xml::Linq::BaseUriAnnotation {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Linq+BaseUriAnnotation")]
impl std::ops::DerefMut for crate::System::Xml::Linq::BaseUriAnnotation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Linq+BaseUriAnnotation")]
impl crate::System::Xml::Linq::BaseUriAnnotation {
    pub fn New(
        baseUri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (baseUri))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        baseUri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (baseUri))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Linq+BaseUriAnnotation")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Linq::BaseUriAnnotation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
