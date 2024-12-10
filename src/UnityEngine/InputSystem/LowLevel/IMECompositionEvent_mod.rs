#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IMECompositionEvent")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct IMECompositionEvent {
    padding: [u8; 152usize],
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IMECompositionEvent")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::IMECompositionEvent =>
    "UnityEngine.InputSystem.LowLevel"."IMECompositionEvent"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IMECompositionEvent")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::IMECompositionEvent {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IMECompositionEvent")]
impl crate::UnityEngine::InputSystem::LowLevel::IMECompositionEvent {
    pub const Type: i32 = 1229800787i32;
    pub const kIMECharBufferSize: i32 = 64i32;
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
