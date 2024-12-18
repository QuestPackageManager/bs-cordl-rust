#[cfg(feature = "ConstructorStringExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct ConstructorStringExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ConstructorStringExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ConstructorStringExtensions =>
    ""."ConstructorStringExtensions"
);
#[cfg(feature = "ConstructorStringExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::ConstructorStringExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ConstructorStringExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::ConstructorStringExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ConstructorStringExtensions")]
impl crate::GlobalNamespace::ConstructorStringExtensions {}
#[cfg(feature = "ConstructorStringExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ConstructorStringExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
