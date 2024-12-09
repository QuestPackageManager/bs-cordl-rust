#[cfg(feature = "ListExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct ListExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ListExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ListExtensions => ""
    ."ListExtensions"
);
#[cfg(feature = "ListExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::ListExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ListExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::ListExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ListExtensions")]
impl crate::GlobalNamespace::ListExtensions {}
#[cfg(feature = "ListExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ListExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
