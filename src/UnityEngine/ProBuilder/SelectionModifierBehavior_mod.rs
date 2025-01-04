#[cfg(feature = "UnityEngine+ProBuilder+SelectionModifierBehavior")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectionModifierBehavior {
    #[default]
    Add = 0i32,
    Difference = 2i32,
    Subtract = 1i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+SelectionModifierBehavior")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::SelectionModifierBehavior => "UnityEngine.ProBuilder"
    ."SelectionModifierBehavior"
);
