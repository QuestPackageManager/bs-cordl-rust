#[cfg(feature = "System+MemoryExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct MemoryExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+MemoryExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::MemoryExtensions => "System"
    ."MemoryExtensions"
);
#[cfg(feature = "System+MemoryExtensions")]
impl std::ops::Deref for crate::System::MemoryExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+MemoryExtensions")]
impl std::ops::DerefMut for crate::System::MemoryExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+MemoryExtensions")]
impl crate::System::MemoryExtensions {}
#[cfg(feature = "System+MemoryExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::System::MemoryExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
