#[cfg(feature = "System+Diagnostics+ProcessModuleCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct ProcessModuleCollection {
    __cordl_parent: crate::System::Collections::ReadOnlyCollectionBase,
}
#[cfg(feature = "System+Diagnostics+ProcessModuleCollection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::ProcessModuleCollection =>
    "System.Diagnostics"."ProcessModuleCollection"
);
#[cfg(feature = "System+Diagnostics+ProcessModuleCollection")]
impl std::ops::Deref for crate::System::Diagnostics::ProcessModuleCollection {
    type Target = crate::System::Collections::ReadOnlyCollectionBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+ProcessModuleCollection")]
impl std::ops::DerefMut for crate::System::Diagnostics::ProcessModuleCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+ProcessModuleCollection")]
impl crate::System::Diagnostics::ProcessModuleCollection {}
#[cfg(feature = "System+Diagnostics+ProcessModuleCollection")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Diagnostics::ProcessModuleCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
