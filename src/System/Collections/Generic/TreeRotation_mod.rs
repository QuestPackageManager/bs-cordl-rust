#[cfg(feature = "System+Collections+Generic+TreeRotation")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TreeRotation {
    #[default]
    Left = 0u8,
    LeftRight = 1u8,
    Right = 2u8,
    RightLeft = 3u8,
}
#[cfg(feature = "System+Collections+Generic+TreeRotation")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Collections::Generic::TreeRotation =>
    "System.Collections.Generic"."TreeRotation"
);
