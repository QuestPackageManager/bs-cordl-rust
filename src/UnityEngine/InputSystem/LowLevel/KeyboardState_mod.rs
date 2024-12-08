#[cfg(feature = "UnityEngine+InputSystem+LowLevel+KeyboardState+_keys_e__FixedBuffer")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct KeyboardState__keys_e__FixedBuffer {
    pub FixedElementField: u8,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+KeyboardState+_keys_e__FixedBuffer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::KeyboardState__keys_e__FixedBuffer =>
    "UnityEngine.InputSystem.LowLevel"."KeyboardState/<keys>e__FixedBuffer"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+KeyboardState+_keys_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::KeyboardState__keys_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+KeyboardState+_keys_e__FixedBuffer")]
impl crate::UnityEngine::InputSystem::LowLevel::KeyboardState__keys_e__FixedBuffer {}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+KeyboardState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct KeyboardState {
    pub keys: crate::UnityEngine::InputSystem::LowLevel::KeyboardState__keys_e__FixedBuffer,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+KeyboardState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::KeyboardState =>
    "UnityEngine.InputSystem.LowLevel"."KeyboardState"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+KeyboardState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::KeyboardState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+KeyboardState")]
impl crate::UnityEngine::InputSystem::LowLevel::KeyboardState {
    pub const kSizeInBits: i32 = 110i32;
    pub const kSizeInBytes: i32 = 14i32;
    #[cfg(
        feature = "UnityEngine+InputSystem+LowLevel+KeyboardState+_keys_e__FixedBuffer"
    )]
    pub type _keys_e__FixedBuffer = crate::UnityEngine::InputSystem::LowLevel::KeyboardState__keys_e__FixedBuffer;
    pub fn Press(
        &mut self,
        key: crate::UnityEngine::InputSystem::Key,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Press",
            (key),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Release(
        &mut self,
        key: crate::UnityEngine::InputSystem::Key,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Release",
            (key),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Set(
        &mut self,
        key: crate::UnityEngine::InputSystem::Key,
        state: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Set",
            (key, state),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        pressedKeys: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::InputSystem::Key,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (pressedKeys),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_format(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::FourCC,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_format",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
