#[cfg(feature = "Unity+Properties+CreatePropertyAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct CreatePropertyAttribute {
    __cordl_parent: crate::System::Attribute,
}
#[cfg(feature = "Unity+Properties+CreatePropertyAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Properties::CreatePropertyAttribute =>
    "Unity.Properties"."CreatePropertyAttribute"
);
#[cfg(feature = "Unity+Properties+CreatePropertyAttribute")]
impl std::ops::Deref for crate::Unity::Properties::CreatePropertyAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+CreatePropertyAttribute")]
impl std::ops::DerefMut for crate::Unity::Properties::CreatePropertyAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+CreatePropertyAttribute")]
impl crate::Unity::Properties::CreatePropertyAttribute {}
#[cfg(feature = "Unity+Properties+CreatePropertyAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::CreatePropertyAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
