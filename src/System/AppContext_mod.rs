#[cfg(feature = "System+AppContext")]
#[repr(C)]
#[derive(Debug)]
pub struct AppContext {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+AppContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::AppContext => "System"."AppContext"
);
#[cfg(feature = "System+AppContext")]
impl std::ops::Deref for crate::System::AppContext {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn InitializeDefaultSwitchValues() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitializeDefaultSwitchValues", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetSwitch(
        switchName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isEnabled: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetSwitch", (switchName, isEnabled))?;
        Ok(__cordl_ret.into())
    }
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AppContext_SwitchValueState {
    #[default]
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
