#[cfg(feature = "System+DateTimeFormat")]
#[repr(C)]
#[derive(Debug)]
pub struct DateTimeFormat {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+DateTimeFormat")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::DateTimeFormat => "System"
    ."DateTimeFormat"
);
#[cfg(feature = "System+DateTimeFormat")]
impl std::ops::Deref for crate::System::DateTimeFormat {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+DateTimeFormat")]
impl std::ops::DerefMut for crate::System::DateTimeFormat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+DateTimeFormat")]
impl crate::System::DateTimeFormat {}
#[cfg(feature = "System+DateTimeFormat")]
impl quest_hook::libil2cpp::ObjectType for crate::System::DateTimeFormat {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
