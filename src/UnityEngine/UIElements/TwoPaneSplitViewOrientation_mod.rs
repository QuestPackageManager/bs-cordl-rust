#[cfg(feature = "UnityEngine+UIElements+TwoPaneSplitViewOrientation")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TwoPaneSplitViewOrientation {
    Horizontal = 0i32,
    Vertical = 1i32,
}
#[cfg(feature = "UnityEngine+UIElements+TwoPaneSplitViewOrientation")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::TwoPaneSplitViewOrientation => "UnityEngine.UIElements"
    ."TwoPaneSplitViewOrientation"
);
