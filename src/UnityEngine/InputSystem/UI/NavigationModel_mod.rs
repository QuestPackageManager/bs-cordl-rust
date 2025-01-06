#[cfg(feature = "UnityEngine+InputSystem+UI+NavigationModel")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NavigationModel {
    pub _cordl_move: crate::UnityEngine::Vector2,
    pub consecutiveMoveCount: i32,
    pub lastMoveDirection: crate::UnityEngine::EventSystems::MoveDirection,
    pub lastMoveTime: f32,
    pub eventData: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::AxisEventData,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+UI+NavigationModel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::UI::NavigationModel =>
    "UnityEngine.InputSystem.UI"."NavigationModel"
);
#[cfg(feature = "UnityEngine+InputSystem+UI+NavigationModel")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::UI::NavigationModel {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+UI+NavigationModel")]
impl crate::UnityEngine::InputSystem::UI::NavigationModel {
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Reset",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
