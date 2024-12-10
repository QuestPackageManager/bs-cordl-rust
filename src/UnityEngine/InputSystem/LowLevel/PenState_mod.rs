#[cfg(feature = "UnityEngine+InputSystem+LowLevel+PenState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PenState {
    padding: [u8; 36usize],
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+PenState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::LowLevel::PenState =>
    "UnityEngine.InputSystem.LowLevel"."PenState"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+PenState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::PenState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+PenState")]
impl crate::UnityEngine::InputSystem::LowLevel::PenState {
    pub fn WithButton(
        &mut self,
        button: crate::UnityEngine::InputSystem::PenButton,
        state: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::PenState,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::PenState = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithButton",
            (button, state),
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
