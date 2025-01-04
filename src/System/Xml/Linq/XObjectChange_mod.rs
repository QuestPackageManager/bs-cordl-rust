#[cfg(feature = "System+Xml+Linq+XObjectChange")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XObjectChange {
    #[default]
    Add = 0i32,
    Name = 2i32,
    Remove = 1i32,
    Value = 3i32,
}
#[cfg(feature = "System+Xml+Linq+XObjectChange")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Linq::XObjectChange =>
    "System.Xml.Linq"."XObjectChange"
);
