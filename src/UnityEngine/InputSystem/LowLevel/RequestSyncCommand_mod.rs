#[cfg(feature = "UnityEngine+InputSystem+LowLevel+RequestSyncCommand")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RequestSyncCommand {
    padding: [u8; 8usize],
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+RequestSyncCommand")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::RequestSyncCommand =>
    "UnityEngine.InputSystem.LowLevel"."RequestSyncCommand"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+RequestSyncCommand")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::RequestSyncCommand {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+RequestSyncCommand")]
impl crate::UnityEngine::InputSystem::LowLevel::RequestSyncCommand {
    pub const kSize: i32 = 8i32;
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