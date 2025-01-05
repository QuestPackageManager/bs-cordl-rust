#[cfg(feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidSensorState")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct AndroidSensorState {
    pub data: crate::UnityEngine::InputSystem::Android::LowLevel::AndroidSensorState__data_e__FixedBuffer,
}
#[cfg(feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidSensorState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Android::LowLevel::AndroidSensorState =>
    "UnityEngine.InputSystem.Android.LowLevel"."AndroidSensorState"
);
#[cfg(feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidSensorState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Android::LowLevel::AndroidSensorState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidSensorState")]
impl crate::UnityEngine::InputSystem::Android::LowLevel::AndroidSensorState {
    #[cfg(
        feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidSensorState+_data_e__FixedBuffer"
    )]
    pub type _data_e__FixedBuffer = crate::UnityEngine::InputSystem::Android::LowLevel::AndroidSensorState__data_e__FixedBuffer;
    pub fn WithData(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Android::LowLevel::AndroidSensorState,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Android::LowLevel::AndroidSensorState = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithData",
            (data),
        )?;
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
}
#[cfg(feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidSensorState")]
impl AsRef<crate::UnityEngine::InputSystem::LowLevel::IInputStateTypeInfo>
for crate::UnityEngine::InputSystem::Android::LowLevel::AndroidSensorState {
    fn as_ref(&self) -> &crate::UnityEngine::InputSystem::LowLevel::IInputStateTypeInfo {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidSensorState")]
impl AsMut<crate::UnityEngine::InputSystem::LowLevel::IInputStateTypeInfo>
for crate::UnityEngine::InputSystem::Android::LowLevel::AndroidSensorState {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::InputSystem::LowLevel::IInputStateTypeInfo {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidSensorState+_data_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct AndroidSensorState__data_e__FixedBuffer {
    pub FixedElementField: f32,
}
#[cfg(
    feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidSensorState+_data_e__FixedBuffer"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Android::LowLevel::AndroidSensorState__data_e__FixedBuffer
    => "UnityEngine.InputSystem.Android.LowLevel"
    ."AndroidSensorState/<data>e__FixedBuffer"
);
#[cfg(
    feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidSensorState+_data_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Android::LowLevel::AndroidSensorState__data_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidSensorState+_data_e__FixedBuffer"
)]
impl crate::UnityEngine::InputSystem::Android::LowLevel::AndroidSensorState__data_e__FixedBuffer {}
