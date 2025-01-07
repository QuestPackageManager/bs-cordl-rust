#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+EnhancedTouchSupport")]
#[repr(C)]
#[derive(Debug)]
pub struct EnhancedTouchSupport {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+EnhancedTouchSupport")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::EnhancedTouch::EnhancedTouchSupport {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.EnhancedTouch";
    const CLASS_NAME: &'static str = "EnhancedTouchSupport";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+EnhancedTouchSupport")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::EnhancedTouch::EnhancedTouchSupport {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+EnhancedTouchSupport")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::EnhancedTouch::EnhancedTouchSupport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+EnhancedTouchSupport")]
impl crate::UnityEngine::InputSystem::EnhancedTouch::EnhancedTouchSupport {
    pub fn CheckEnabled() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Disable() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Disable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Enable() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Enable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnDeviceChange(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        change: crate::UnityEngine::InputSystem::InputDeviceChange,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnDeviceChange", (device, change))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnSettingsChange() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnSettingsChange", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Reset() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Reset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUpState() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetUpState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TearDownState() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TearDownState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enabled() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_enabled", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+EnhancedTouchSupport")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::EnhancedTouch::EnhancedTouchSupport {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
