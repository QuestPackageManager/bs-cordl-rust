#[cfg(feature = "LinkedListExtension")]
#[repr(C)]
#[derive(Debug)]
pub struct LinkedListExtension {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "LinkedListExtension")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LinkedListExtension => ""."LinkedListExtension"
);
#[cfg(feature = "LinkedListExtension")]
impl std::ops::Deref for LinkedListExtension {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LinkedListExtension")]
impl std::ops::DerefMut for LinkedListExtension {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LinkedListExtension")]
impl LinkedListExtension {}
#[cfg(feature = "LinkedListExtension")]
impl quest_hook::libil2cpp::ObjectType for LinkedListExtension {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
