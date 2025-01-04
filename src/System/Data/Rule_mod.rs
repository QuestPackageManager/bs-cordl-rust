#[cfg(feature = "System+Data+Rule")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Rule {
    #[default]
    Cascade = 1i32,
    None = 0i32,
    SetDefault = 3i32,
    SetNull = 2i32,
}
#[cfg(feature = "System+Data+Rule")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::Rule => "System.Data"."Rule"
);
