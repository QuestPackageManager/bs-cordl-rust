#[cfg(feature = "ArrayExtension")]
#[repr(C)]
#[derive(Debug)]
pub struct ArrayExtension {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ArrayExtension")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ArrayExtension => ""
    ."ArrayExtension"
);
#[cfg(feature = "ArrayExtension")]
impl std::ops::Deref for crate::GlobalNamespace::ArrayExtension {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ArrayExtension")]
impl std::ops::DerefMut for crate::GlobalNamespace::ArrayExtension {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ArrayExtension")]
impl crate::GlobalNamespace::ArrayExtension {}
#[cfg(feature = "ArrayExtension")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ArrayExtension {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
