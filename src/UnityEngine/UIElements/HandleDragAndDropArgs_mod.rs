#[cfg(feature = "UnityEngine+UIElements+HandleDragAndDropArgs")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct HandleDragAndDropArgs {
    pub m_DragAndDropArgs: crate::UnityEngine::UIElements::DragAndDropArgs,
    pub _position_k__BackingField: crate::UnityEngine::Vector2,
}
#[cfg(feature = "UnityEngine+UIElements+HandleDragAndDropArgs")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::HandleDragAndDropArgs
    => "UnityEngine.UIElements"."HandleDragAndDropArgs"
);
#[cfg(feature = "UnityEngine+UIElements+HandleDragAndDropArgs")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::HandleDragAndDropArgs {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+HandleDragAndDropArgs")]
impl crate::UnityEngine::UIElements::HandleDragAndDropArgs {
    pub fn _ctor(
        &mut self,
        position: crate::UnityEngine::Vector2,
        dragAndDropArgs: crate::UnityEngine::UIElements::DragAndDropArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (position, dragAndDropArgs),
        )?;
        Ok(__cordl_ret.into())
    }
}
