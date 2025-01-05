#[cfg(feature = "UnityEngine+InputSystem+Utilities+Vector2MagnitudeComparer")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Vector2MagnitudeComparer {}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Vector2MagnitudeComparer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Utilities::Vector2MagnitudeComparer =>
    "UnityEngine.InputSystem.Utilities"."Vector2MagnitudeComparer"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Vector2MagnitudeComparer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Utilities::Vector2MagnitudeComparer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Vector2MagnitudeComparer")]
impl crate::UnityEngine::InputSystem::Utilities::Vector2MagnitudeComparer {
    pub fn Compare(
        &mut self,
        x: crate::UnityEngine::Vector2,
        y: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Compare",
            (x, y),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Vector2MagnitudeComparer")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector2>>
for crate::UnityEngine::InputSystem::Utilities::Vector2MagnitudeComparer {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector2> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Vector2MagnitudeComparer")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector2>>
for crate::UnityEngine::InputSystem::Utilities::Vector2MagnitudeComparer {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector2> {
        todo!()
    }
}
