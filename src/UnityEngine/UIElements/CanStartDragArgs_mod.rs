#[cfg(feature = "UnityEngine+UIElements+CanStartDragArgs")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct CanStartDragArgs {
    pub draggedElement: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub id: i32,
    pub selectedIds: quest_hook::libil2cpp::Gc<i32>,
}
#[cfg(feature = "UnityEngine+UIElements+CanStartDragArgs")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::CanStartDragArgs =>
    "UnityEngine.UIElements"."CanStartDragArgs"
);
#[cfg(feature = "UnityEngine+UIElements+CanStartDragArgs")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::CanStartDragArgs {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+CanStartDragArgs")]
impl crate::UnityEngine::UIElements::CanStartDragArgs {
    pub fn _ctor(
        &mut self,
        draggedElement: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        id: i32,
        selectedIds: quest_hook::libil2cpp::Gc<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (draggedElement, id, selectedIds),
        )?;
        Ok(__cordl_ret.into())
    }
}
