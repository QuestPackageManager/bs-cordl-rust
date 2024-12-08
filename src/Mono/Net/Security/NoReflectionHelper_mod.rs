#[cfg(feature = "Mono+Net+Security+NoReflectionHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct NoReflectionHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Mono+Net+Security+NoReflectionHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Net::Security::NoReflectionHelper =>
    "Mono.Net.Security"."NoReflectionHelper"
);
#[cfg(feature = "Mono+Net+Security+NoReflectionHelper")]
impl std::ops::Deref for crate::Mono::Net::Security::NoReflectionHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+NoReflectionHelper")]
impl std::ops::DerefMut for crate::Mono::Net::Security::NoReflectionHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+NoReflectionHelper")]
impl crate::Mono::Net::Security::NoReflectionHelper {}
#[cfg(feature = "Mono+Net+Security+NoReflectionHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Net::Security::NoReflectionHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
