#[cfg(feature = "System+ComponentModel+RefreshProperties")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RefreshProperties {
    #[default]
    All = 1i32,
    None = 0i32,
    Repaint = 2i32,
}
#[cfg(feature = "System+ComponentModel+RefreshProperties")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::RefreshProperties =>
    "System.ComponentModel"."RefreshProperties"
);
