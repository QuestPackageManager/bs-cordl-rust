#[cfg(feature = "System+Collections+Generic+InsertionBehavior")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InsertionBehavior {
    #[default]
    None = 0u8,
    OverwriteExisting = 1u8,
    ThrowOnExisting = 2u8,
}
#[cfg(feature = "System+Collections+Generic+InsertionBehavior")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Collections::Generic::InsertionBehavior
    => "System.Collections.Generic"."InsertionBehavior"
);
