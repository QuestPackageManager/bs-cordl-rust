#[cfg(feature = "PackBuyOption")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackBuyOption {
    Default = 0i32,
    DisableBuyOption = 1i32,
}
#[cfg(feature = "PackBuyOption")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PackBuyOption => ""
    ."PackBuyOption"
);
