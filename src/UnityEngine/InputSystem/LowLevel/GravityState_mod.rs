#[cfg(feature = "UnityEngine+InputSystem+LowLevel+GravityState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GravityState {
    pub gravity: crate::UnityEngine::Vector3,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+GravityState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::LowLevel::GravityState
    => "UnityEngine.InputSystem.LowLevel"."GravityState"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+GravityState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::GravityState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+GravityState")]
impl crate::UnityEngine::InputSystem::LowLevel::GravityState {
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
