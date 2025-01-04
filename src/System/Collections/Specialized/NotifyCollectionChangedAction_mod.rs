#[cfg(feature = "System+Collections+Specialized+NotifyCollectionChangedAction")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NotifyCollectionChangedAction {
    #[default]
    Add = 0i32,
    Move = 3i32,
    Remove = 1i32,
    Replace = 2i32,
    Reset = 4i32,
}
#[cfg(feature = "System+Collections+Specialized+NotifyCollectionChangedAction")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Collections::Specialized::NotifyCollectionChangedAction =>
    "System.Collections.Specialized"."NotifyCollectionChangedAction"
);
