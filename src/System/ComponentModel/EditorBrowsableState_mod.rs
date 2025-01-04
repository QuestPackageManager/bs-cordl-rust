#[cfg(feature = "System+ComponentModel+EditorBrowsableState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EditorBrowsableState {
    #[default]
    Advanced = 2i32,
    Always = 0i32,
    Never = 1i32,
}
#[cfg(feature = "System+ComponentModel+EditorBrowsableState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::EditorBrowsableState =>
    "System.ComponentModel"."EditorBrowsableState"
);
