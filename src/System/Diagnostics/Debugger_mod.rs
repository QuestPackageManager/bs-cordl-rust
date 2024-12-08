#[cfg(feature = "System+Diagnostics+Debugger")]
#[repr(C)]
#[derive(Debug)]
pub struct Debugger {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Diagnostics+Debugger")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::Debugger =>
    "System.Diagnostics"."Debugger"
);
#[cfg(feature = "System+Diagnostics+Debugger")]
impl std::ops::Deref for crate::System::Diagnostics::Debugger {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+Debugger")]
impl std::ops::DerefMut for crate::System::Diagnostics::Debugger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+Debugger")]
impl crate::System::Diagnostics::Debugger {}
#[cfg(feature = "System+Diagnostics+Debugger")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Diagnostics::Debugger {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
