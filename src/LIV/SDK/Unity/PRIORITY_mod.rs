#[cfg(feature = "LIV+SDK+Unity+PRIORITY")]
#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PRIORITY {
    GAME = 63i8,
    NONE = 0i8,
}
#[cfg(feature = "LIV+SDK+Unity+PRIORITY")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LIV::SDK::Unity::PRIORITY => "LIV.SDK.Unity"
    ."PRIORITY"
);
