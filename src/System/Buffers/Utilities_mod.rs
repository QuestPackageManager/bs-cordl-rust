#[cfg(feature = "System+Buffers+Utilities")]
#[repr(C)]
#[derive(Debug)]
pub struct Utilities {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Buffers+Utilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Buffers::Utilities => "System.Buffers"
    ."Utilities"
);
#[cfg(feature = "System+Buffers+Utilities")]
impl std::ops::Deref for crate::System::Buffers::Utilities {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Buffers+Utilities")]
impl std::ops::DerefMut for crate::System::Buffers::Utilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Buffers+Utilities")]
impl crate::System::Buffers::Utilities {}
#[cfg(feature = "System+Buffers+Utilities")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Buffers::Utilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
