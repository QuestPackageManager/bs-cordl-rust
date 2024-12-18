#[cfg(feature = "BinaryReadWriteExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct BinaryReadWriteExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BinaryReadWriteExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BinaryReadWriteExtensions => ""
    ."BinaryReadWriteExtensions"
);
#[cfg(feature = "BinaryReadWriteExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::BinaryReadWriteExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BinaryReadWriteExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::BinaryReadWriteExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BinaryReadWriteExtensions")]
impl crate::GlobalNamespace::BinaryReadWriteExtensions {}
#[cfg(feature = "BinaryReadWriteExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BinaryReadWriteExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
