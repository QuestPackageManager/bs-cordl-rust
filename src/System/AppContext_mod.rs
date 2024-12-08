#[cfg(feature = "System+AppContext")]
#[repr(C)]
#[derive(Debug)]
pub struct AppContext {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+AppContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::AppContext => "System"."AppContext"
);
#[cfg(feature = "System+AppContext")]
impl std::ops::Deref for crate::System::AppContext {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+AppContext")]
impl std::ops::DerefMut for crate::System::AppContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+AppContext")]
impl crate::System::AppContext {
    #[cfg(feature = "System+AppContext+SwitchValueState")]
    pub type SwitchValueState = crate::System::AppContext_SwitchValueState;
}
#[cfg(feature = "System+AppContext")]
impl quest_hook::libil2cpp::ObjectType for crate::System::AppContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+AppContext+SwitchValueState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppContext_SwitchValueState {
    HasFalseValue = 1i32,
    HasLookedForOverride = 4i32,
    HasTrueValue = 2i32,
    UnknownValue = 8i32,
}
#[cfg(feature = "System+AppContext+SwitchValueState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::AppContext_SwitchValueState => "System"
    ."AppContext/SwitchValueState"
);
