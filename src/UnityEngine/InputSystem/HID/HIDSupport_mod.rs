#[cfg(feature = "UnityEngine+InputSystem+HID+HIDSupport")]
#[repr(C)]
#[derive(Debug)]
pub struct HIDSupport {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDSupport")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::HID::HIDSupport =>
    "UnityEngine.InputSystem.HID"."HIDSupport"
);
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDSupport")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::HID::HIDSupport {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDSupport")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::HID::HIDSupport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDSupport")]
impl crate::UnityEngine::InputSystem::HID::HIDSupport {
    #[cfg(feature = "UnityEngine+InputSystem+HID+HIDSupport+HIDPageUsage")]
    pub type HIDPageUsage = crate::UnityEngine::InputSystem::HID::HIDSupport_HIDPageUsage;
    pub fn Initialize() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Initialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_supportedHIDUsages() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::HID::HIDSupport_HIDPageUsage,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::HID::HIDSupport_HIDPageUsage,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_supportedHIDUsages", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_supportedHIDUsages(
        value: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::HID::HIDSupport_HIDPageUsage,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_supportedHIDUsages", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDSupport")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::HID::HIDSupport {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDSupport+HIDPageUsage")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct HIDSupport_HIDPageUsage {
    pub page: crate::UnityEngine::InputSystem::HID::HID_UsagePage,
    pub usage: i32,
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDSupport+HIDPageUsage")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::HID::HIDSupport_HIDPageUsage =>
    "UnityEngine.InputSystem.HID"."HIDSupport/HIDPageUsage"
);
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDSupport+HIDPageUsage")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::HID::HIDSupport_HIDPageUsage {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDSupport+HIDPageUsage")]
impl crate::UnityEngine::InputSystem::HID::HIDSupport_HIDPageUsage {
    pub fn _ctor_HID_GenericDesktop1(
        &mut self,
        usage: crate::UnityEngine::InputSystem::HID::HID_GenericDesktop,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (usage),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_HID_UsagePage_i32_0(
        &mut self,
        page: crate::UnityEngine::InputSystem::HID::HID_UsagePage,
        usage: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (page, usage),
        )?;
        Ok(__cordl_ret.into())
    }
}
