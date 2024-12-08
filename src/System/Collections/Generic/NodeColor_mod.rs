#[cfg(feature = "System+Collections+Generic+NodeColor")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeColor {
    Black = 0u8,
    Red = 1u8,
}
#[cfg(feature = "System+Collections+Generic+NodeColor")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Collections::Generic::NodeColor =>
    "System.Collections.Generic"."NodeColor"
);
