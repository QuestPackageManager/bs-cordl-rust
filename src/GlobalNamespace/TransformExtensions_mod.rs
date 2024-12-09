#[cfg(feature = "TransformExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct TransformExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "TransformExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::TransformExtensions => ""
    ."TransformExtensions"
);
#[cfg(feature = "TransformExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::TransformExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TransformExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::TransformExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TransformExtensions")]
impl crate::GlobalNamespace::TransformExtensions {}
#[cfg(feature = "TransformExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::TransformExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
