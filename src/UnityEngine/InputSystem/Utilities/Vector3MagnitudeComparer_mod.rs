#[cfg(feature = "UnityEngine+InputSystem+Utilities+Vector3MagnitudeComparer")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Vector3MagnitudeComparer {}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Vector3MagnitudeComparer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Utilities::Vector3MagnitudeComparer =>
    "UnityEngine.InputSystem.Utilities"."Vector3MagnitudeComparer"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Vector3MagnitudeComparer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Utilities::Vector3MagnitudeComparer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Vector3MagnitudeComparer")]
impl crate::UnityEngine::InputSystem::Utilities::Vector3MagnitudeComparer {
    pub fn Compare(
        &mut self,
        x: crate::UnityEngine::Vector3,
        y: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Compare",
            (x, y),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Vector3MagnitudeComparer")]
impl AsRef<crate::System::Collections::Generic::IComparer_1<crate::UnityEngine::Vector3>>
for crate::UnityEngine::InputSystem::Utilities::Vector3MagnitudeComparer {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IComparer_1<crate::UnityEngine::Vector3> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Vector3MagnitudeComparer")]
impl AsMut<crate::System::Collections::Generic::IComparer_1<crate::UnityEngine::Vector3>>
for crate::UnityEngine::InputSystem::Utilities::Vector3MagnitudeComparer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IComparer_1<
        crate::UnityEngine::Vector3,
    > {
        todo!()
    }
}
