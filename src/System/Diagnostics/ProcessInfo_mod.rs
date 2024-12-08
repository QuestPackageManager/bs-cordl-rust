#[cfg(feature = "System+Diagnostics+ProcessInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct ProcessInfo {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Diagnostics+ProcessInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::ProcessInfo =>
    "System.Diagnostics"."ProcessInfo"
);
#[cfg(feature = "System+Diagnostics+ProcessInfo")]
impl std::ops::Deref for crate::System::Diagnostics::ProcessInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+ProcessInfo")]
impl std::ops::DerefMut for crate::System::Diagnostics::ProcessInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+ProcessInfo")]
impl crate::System::Diagnostics::ProcessInfo {}
#[cfg(feature = "System+Diagnostics+ProcessInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Diagnostics::ProcessInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
