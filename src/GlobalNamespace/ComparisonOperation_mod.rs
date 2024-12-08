#[cfg(feature = "ComparisonOperation")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonOperation {
    Equal = 0i32,
    NotEqual = 1i32,
}
#[cfg(feature = "ComparisonOperation")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ComparisonOperation => ""
    ."ComparisonOperation"
);
