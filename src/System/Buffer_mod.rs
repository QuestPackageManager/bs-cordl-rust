#[cfg(feature = "System+Buffer")]
#[repr(C)]
#[derive(Debug)]
pub struct Buffer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Buffer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Buffer => "System"."Buffer"
);
#[cfg(feature = "System+Buffer")]
impl std::ops::Deref for crate::System::Buffer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Buffer")]
impl std::ops::DerefMut for crate::System::Buffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Buffer")]
impl crate::System::Buffer {}
#[cfg(feature = "System+Buffer")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Buffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
