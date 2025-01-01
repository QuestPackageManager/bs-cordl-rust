#[cfg(feature = "RandomExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct RandomExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "RandomExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::RandomExtensions => ""
    ."RandomExtensions"
);
#[cfg(feature = "RandomExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::RandomExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RandomExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::RandomExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RandomExtensions")]
impl crate::GlobalNamespace::RandomExtensions {}
#[cfg(feature = "RandomExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::RandomExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}