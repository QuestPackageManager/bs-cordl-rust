#[cfg(feature = "BinaryReadWriteExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct BinaryReadWriteExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BinaryReadWriteExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BinaryReadWriteExtensions => ""
    ."BinaryReadWriteExtensions"
);
#[cfg(feature = "BinaryReadWriteExtensions")]
impl std::ops::Deref for BinaryReadWriteExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BinaryReadWriteExtensions")]
impl std::ops::DerefMut for BinaryReadWriteExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BinaryReadWriteExtensions")]
impl BinaryReadWriteExtensions {}
#[cfg(feature = "BinaryReadWriteExtensions")]
impl quest_hook::libil2cpp::ObjectType for BinaryReadWriteExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
