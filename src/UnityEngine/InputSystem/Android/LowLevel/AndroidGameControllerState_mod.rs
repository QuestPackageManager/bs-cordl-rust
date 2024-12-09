#[cfg(feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidGameControllerState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AndroidGameControllerState {
    pub buttons: crate::UnityEngine::InputSystem::Android::LowLevel::AndroidGameControllerState__buttons_e__FixedBuffer,
    pub axis: crate::UnityEngine::InputSystem::Android::LowLevel::AndroidGameControllerState__axis_e__FixedBuffer,
}
#[cfg(feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidGameControllerState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Android::LowLevel::AndroidGameControllerState =>
    "UnityEngine.InputSystem.Android.LowLevel"."AndroidGameControllerState"
);
#[cfg(feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidGameControllerState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Android::LowLevel::AndroidGameControllerState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidGameControllerState")]
impl crate::UnityEngine::InputSystem::Android::LowLevel::AndroidGameControllerState {
    pub const MaxAxes: i32 = 48i32;
    pub const MaxButtons: i32 = 220i32;
    pub const kAxisOffset: u32 = 1632046620u32;
    #[cfg(
        feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidGameControllerState+Variants"
    )]
    pub type Variants = crate::UnityEngine::InputSystem::Android::LowLevel::AndroidGameControllerState_Variants;
    #[cfg(
        feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidGameControllerState+_axis_e__FixedBuffer"
    )]
    pub type _axis_e__FixedBuffer = crate::UnityEngine::InputSystem::Android::LowLevel::AndroidGameControllerState__axis_e__FixedBuffer;
    #[cfg(
        feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidGameControllerState+_buttons_e__FixedBuffer"
    )]
    pub type _buttons_e__FixedBuffer = crate::UnityEngine::InputSystem::Android::LowLevel::AndroidGameControllerState__buttons_e__FixedBuffer;
    pub fn WithAxis(
        &mut self,
        axis: crate::UnityEngine::InputSystem::Android::LowLevel::AndroidAxis,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Android::LowLevel::AndroidGameControllerState,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Android::LowLevel::AndroidGameControllerState = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithAxis",
            (axis, value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn WithButton(
        &mut self,
        code: crate::UnityEngine::InputSystem::Android::LowLevel::AndroidKeyCode,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Android::LowLevel::AndroidGameControllerState,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Android::LowLevel::AndroidGameControllerState = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithButton",
            (code, value),
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
#[cfg(
    feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidGameControllerState+Variants"
)]
#[repr(C)]
#[derive(Debug)]
pub struct AndroidGameControllerState_Variants {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidGameControllerState+Variants"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Android::LowLevel::AndroidGameControllerState_Variants =>
    "UnityEngine.InputSystem.Android.LowLevel"."AndroidGameControllerState/Variants"
);
#[cfg(
    feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidGameControllerState+Variants"
)]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::Android::LowLevel::AndroidGameControllerState_Variants {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidGameControllerState+Variants"
)]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::Android::LowLevel::AndroidGameControllerState_Variants {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidGameControllerState+Variants"
)]
impl crate::UnityEngine::InputSystem::Android::LowLevel::AndroidGameControllerState_Variants {
    pub const DPadAxes: &'static str = "DpadAxes";
    pub const DPadButtons: &'static str = "DpadButtons";
    pub const Gamepad: &'static str = "Gamepad";
    pub const Joystick: &'static str = "Joystick";
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidGameControllerState+Variants"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Android::LowLevel::AndroidGameControllerState_Variants {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidGameControllerState+_axis_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AndroidGameControllerState__axis_e__FixedBuffer {
    pub FixedElementField: f32,
}
#[cfg(
    feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidGameControllerState+_axis_e__FixedBuffer"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Android::LowLevel::AndroidGameControllerState__axis_e__FixedBuffer
    => "UnityEngine.InputSystem.Android.LowLevel"
    ."AndroidGameControllerState/<axis>e__FixedBuffer"
);
#[cfg(
    feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidGameControllerState+_axis_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Android::LowLevel::AndroidGameControllerState__axis_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidGameControllerState+_axis_e__FixedBuffer"
)]
impl crate::UnityEngine::InputSystem::Android::LowLevel::AndroidGameControllerState__axis_e__FixedBuffer {}
#[cfg(
    feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidGameControllerState+_buttons_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AndroidGameControllerState__buttons_e__FixedBuffer {
    pub FixedElementField: u32,
}
#[cfg(
    feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidGameControllerState+_buttons_e__FixedBuffer"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Android::LowLevel::AndroidGameControllerState__buttons_e__FixedBuffer
    => "UnityEngine.InputSystem.Android.LowLevel"
    ."AndroidGameControllerState/<buttons>e__FixedBuffer"
);
#[cfg(
    feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidGameControllerState+_buttons_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Android::LowLevel::AndroidGameControllerState__buttons_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidGameControllerState+_buttons_e__FixedBuffer"
)]
impl crate::UnityEngine::InputSystem::Android::LowLevel::AndroidGameControllerState__buttons_e__FixedBuffer {}
