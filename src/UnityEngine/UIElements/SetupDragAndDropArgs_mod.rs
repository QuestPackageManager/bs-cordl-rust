#[cfg(feature = "UnityEngine+UIElements+SetupDragAndDropArgs")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct SetupDragAndDropArgs {
    pub draggedElement: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub selectedIds: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IEnumerable_1<i32>,
    >,
    pub startDragArgs: crate::UnityEngine::UIElements::StartDragArgs,
}
#[cfg(feature = "UnityEngine+UIElements+SetupDragAndDropArgs")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::SetupDragAndDropArgs =>
    "UnityEngine.UIElements"."SetupDragAndDropArgs"
);
#[cfg(feature = "UnityEngine+UIElements+SetupDragAndDropArgs")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::SetupDragAndDropArgs {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+SetupDragAndDropArgs")]
impl crate::UnityEngine::UIElements::SetupDragAndDropArgs {
    pub fn _ctor(
        &mut self,
        draggedElement: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        selectedIds: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
        startDragArgs: crate::UnityEngine::UIElements::StartDragArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (draggedElement, selectedIds, startDragArgs),
        )?;
        Ok(__cordl_ret.into())
    }
}
