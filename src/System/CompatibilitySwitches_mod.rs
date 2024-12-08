#[cfg(feature = "System+CompatibilitySwitches")]
#[repr(C)]
#[derive(Debug)]
pub struct CompatibilitySwitches {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+CompatibilitySwitches")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::CompatibilitySwitches => "System"
    ."CompatibilitySwitches"
);
#[cfg(feature = "System+CompatibilitySwitches")]
impl std::ops::Deref for crate::System::CompatibilitySwitches {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+CompatibilitySwitches")]
impl std::ops::DerefMut for crate::System::CompatibilitySwitches {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+CompatibilitySwitches")]
impl crate::System::CompatibilitySwitches {}
#[cfg(feature = "System+CompatibilitySwitches")]
impl quest_hook::libil2cpp::ObjectType for crate::System::CompatibilitySwitches {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
