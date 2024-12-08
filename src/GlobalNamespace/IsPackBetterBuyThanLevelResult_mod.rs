#[cfg(feature = "IsPackBetterBuyThanLevelResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsPackBetterBuyThanLevelResult {
    Failed = 2i32,
    LevelIsBetter = 1i32,
    PackIsBetter = 0i32,
}
#[cfg(feature = "IsPackBetterBuyThanLevelResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for IsPackBetterBuyThanLevelResult => ""
    ."IsPackBetterBuyThanLevelResult"
);
