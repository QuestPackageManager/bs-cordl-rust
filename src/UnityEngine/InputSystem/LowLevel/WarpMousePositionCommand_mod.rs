#[cfg(feature = "UnityEngine+InputSystem+LowLevel+WarpMousePositionCommand")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct WarpMousePositionCommand {
    padding: [u8; 16usize],
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+WarpMousePositionCommand")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::WarpMousePositionCommand =>
    "UnityEngine.InputSystem.LowLevel"."WarpMousePositionCommand"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+WarpMousePositionCommand")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::WarpMousePositionCommand {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+WarpMousePositionCommand")]
impl crate::UnityEngine::InputSystem::LowLevel::WarpMousePositionCommand {
    pub const kSize: i32 = 16i32;
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
        Ok(__cordl_ret)
    }
}
