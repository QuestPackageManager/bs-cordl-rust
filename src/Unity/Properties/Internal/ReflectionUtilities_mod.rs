#[cfg(feature = "Unity+Properties+Internal+ReflectionUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionUtilities {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Unity+Properties+Internal+ReflectionUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Properties::Internal::ReflectionUtilities
    => "Unity.Properties.Internal"."ReflectionUtilities"
);
#[cfg(feature = "Unity+Properties+Internal+ReflectionUtilities")]
impl std::ops::Deref for crate::Unity::Properties::Internal::ReflectionUtilities {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+ReflectionUtilities")]
impl std::ops::DerefMut for crate::Unity::Properties::Internal::ReflectionUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+ReflectionUtilities")]
impl crate::Unity::Properties::Internal::ReflectionUtilities {}
#[cfg(feature = "Unity+Properties+Internal+ReflectionUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::ReflectionUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
