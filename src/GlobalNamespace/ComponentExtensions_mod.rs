#[cfg(feature = "ComponentExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct ComponentExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "ComponentExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ComponentExtensions => ""."ComponentExtensions"
);
#[cfg(feature = "ComponentExtensions")]
impl std::ops::Deref for ComponentExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ComponentExtensions")]
impl std::ops::DerefMut for ComponentExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ComponentExtensions")]
impl ComponentExtensions {}
#[cfg(feature = "ComponentExtensions")]
impl quest_hook::libil2cpp::ObjectType for ComponentExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
