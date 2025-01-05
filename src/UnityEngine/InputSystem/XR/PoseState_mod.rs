#[cfg(feature = "UnityEngine+InputSystem+XR+PoseState")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PoseState {
    padding: [u8; 60usize],
}
#[cfg(feature = "UnityEngine+InputSystem+XR+PoseState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::XR::PoseState =>
    "UnityEngine.InputSystem.XR"."PoseState"
);
#[cfg(feature = "UnityEngine+InputSystem+XR+PoseState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::XR::PoseState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+PoseState")]
impl crate::UnityEngine::InputSystem::XR::PoseState {
    pub const kSizeInBytes: i32 = 60i32;
    pub fn _ctor(
        &mut self,
        isTracked: bool,
        trackingState: crate::UnityEngine::XR::InputTrackingState,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        velocity: crate::UnityEngine::Vector3,
        angularVelocity: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (isTracked, trackingState, position, rotation, velocity, angularVelocity),
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
#[cfg(feature = "UnityEngine+InputSystem+XR+PoseState")]
impl AsRef<crate::UnityEngine::InputSystem::LowLevel::IInputStateTypeInfo>
for crate::UnityEngine::InputSystem::XR::PoseState {
    fn as_ref(&self) -> &crate::UnityEngine::InputSystem::LowLevel::IInputStateTypeInfo {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+PoseState")]
impl AsMut<crate::UnityEngine::InputSystem::LowLevel::IInputStateTypeInfo>
for crate::UnityEngine::InputSystem::XR::PoseState {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::InputSystem::LowLevel::IInputStateTypeInfo {
        todo!()
    }
}
