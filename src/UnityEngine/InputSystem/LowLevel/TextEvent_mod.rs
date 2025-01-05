#[cfg(feature = "UnityEngine+InputSystem+LowLevel+TextEvent")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TextEvent {
    padding: [u8; 24usize],
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+TextEvent")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::LowLevel::TextEvent =>
    "UnityEngine.InputSystem.LowLevel"."TextEvent"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+TextEvent")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::TextEvent {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+TextEvent")]
impl crate::UnityEngine::InputSystem::LowLevel::TextEvent {
    pub const Type: i32 = 1413830740i32;
    pub fn Create__cordl_char0(
        deviceId: i32,
        character: char,
        _cordl_time: f64,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::TextEvent,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::TextEvent = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (deviceId, character, _cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn Create_i32_1(
        deviceId: i32,
        character: i32,
        _cordl_time: f64,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::TextEvent,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::TextEvent = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (deviceId, character, _cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn From(
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("From", (eventPtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_typeStatic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::FourCC,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_typeStatic",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+TextEvent")]
impl AsRef<crate::UnityEngine::InputSystem::LowLevel::IInputEventTypeInfo>
for crate::UnityEngine::InputSystem::LowLevel::TextEvent {
    fn as_ref(&self) -> &crate::UnityEngine::InputSystem::LowLevel::IInputEventTypeInfo {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+TextEvent")]
impl AsMut<crate::UnityEngine::InputSystem::LowLevel::IInputEventTypeInfo>
for crate::UnityEngine::InputSystem::LowLevel::TextEvent {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::InputSystem::LowLevel::IInputEventTypeInfo {
        todo!()
    }
}
