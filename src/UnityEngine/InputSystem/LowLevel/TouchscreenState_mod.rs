#[cfg(feature = "UnityEngine+InputSystem+LowLevel+TouchscreenState")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TouchscreenState {
    padding: [u8; 616usize],
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+TouchscreenState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::TouchscreenState =>
    "UnityEngine.InputSystem.LowLevel"."TouchscreenState"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+TouchscreenState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::TouchscreenState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+TouchscreenState")]
impl crate::UnityEngine::InputSystem::LowLevel::TouchscreenState {
    pub const MaxTouches: i32 = 10i32;
    pub const kTouchDataOffset: i32 = 56i32;
    #[cfg(
        feature = "UnityEngine+InputSystem+LowLevel+TouchscreenState+_primaryTouchData_e__FixedBuffer"
    )]
    pub type _primaryTouchData_e__FixedBuffer = crate::UnityEngine::InputSystem::LowLevel::TouchscreenState__primaryTouchData_e__FixedBuffer;
    #[cfg(
        feature = "UnityEngine+InputSystem+LowLevel+TouchscreenState+_touchData_e__FixedBuffer"
    )]
    pub type _touchData_e__FixedBuffer = crate::UnityEngine::InputSystem::LowLevel::TouchscreenState__touchData_e__FixedBuffer;
    pub fn get_Format() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::FourCC,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Format", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_primaryTouch(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_primaryTouch", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_touches(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_touches", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+TouchscreenState")]
impl AsRef<crate::UnityEngine::InputSystem::LowLevel::IInputStateTypeInfo>
for crate::UnityEngine::InputSystem::LowLevel::TouchscreenState {
    fn as_ref(&self) -> &crate::UnityEngine::InputSystem::LowLevel::IInputStateTypeInfo {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+TouchscreenState")]
impl AsMut<crate::UnityEngine::InputSystem::LowLevel::IInputStateTypeInfo>
for crate::UnityEngine::InputSystem::LowLevel::TouchscreenState {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::InputSystem::LowLevel::IInputStateTypeInfo {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+TouchscreenState+_primaryTouchData_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TouchscreenState__primaryTouchData_e__FixedBuffer {
    pub FixedElementField: u8,
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+TouchscreenState+_primaryTouchData_e__FixedBuffer"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::TouchscreenState__primaryTouchData_e__FixedBuffer
    => "UnityEngine.InputSystem.LowLevel"
    ."TouchscreenState/<primaryTouchData>e__FixedBuffer"
);
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+TouchscreenState+_primaryTouchData_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::TouchscreenState__primaryTouchData_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+TouchscreenState+_primaryTouchData_e__FixedBuffer"
)]
impl crate::UnityEngine::InputSystem::LowLevel::TouchscreenState__primaryTouchData_e__FixedBuffer {}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+TouchscreenState+_touchData_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TouchscreenState__touchData_e__FixedBuffer {
    pub FixedElementField: u8,
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+TouchscreenState+_touchData_e__FixedBuffer"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::TouchscreenState__touchData_e__FixedBuffer =>
    "UnityEngine.InputSystem.LowLevel"."TouchscreenState/<touchData>e__FixedBuffer"
);
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+TouchscreenState+_touchData_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::TouchscreenState__touchData_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+TouchscreenState+_touchData_e__FixedBuffer"
)]
impl crate::UnityEngine::InputSystem::LowLevel::TouchscreenState__touchData_e__FixedBuffer {}
