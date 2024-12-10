#[cfg(feature = "UnityEngine+InputSystem+LowLevel+PointerState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PointerState {
    pub pointerId: u32,
    pub position: crate::UnityEngine::Vector2,
    pub delta: crate::UnityEngine::Vector2,
    pub pressure: f32,
    pub radius: crate::UnityEngine::Vector2,
    pub buttons: u16,
    pub displayIndex: u16,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+PointerState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::LowLevel::PointerState
    => "UnityEngine.InputSystem.LowLevel"."PointerState"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+PointerState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::PointerState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+PointerState")]
impl crate::UnityEngine::InputSystem::LowLevel::PointerState {
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
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+PointerState")]
impl AsRef<crate::UnityEngine::InputSystem::LowLevel::IInputStateTypeInfo>
for crate::UnityEngine::InputSystem::LowLevel::PointerState {
    fn as_ref(&self) -> &crate::UnityEngine::InputSystem::LowLevel::IInputStateTypeInfo {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+PointerState")]
impl AsMut<crate::UnityEngine::InputSystem::LowLevel::IInputStateTypeInfo>
for crate::UnityEngine::InputSystem::LowLevel::PointerState {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::InputSystem::LowLevel::IInputStateTypeInfo {
        todo!()
    }
}
