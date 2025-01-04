#[cfg(feature = "UnityEngine+LineAlignment")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineAlignment {
    #[default]
    Local = 1i32,
    View = 0i32,
}
#[cfg(feature = "UnityEngine+LineAlignment")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::LineAlignment => "UnityEngine"
    ."LineAlignment"
);
