#[cfg(feature = "System+Xml+Schema+Compositor")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compositor {
    Import = 2i32,
    Include = 1i32,
    Redefine = 3i32,
    Root = 0i32,
}
#[cfg(feature = "System+Xml+Schema+Compositor")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::Compositor =>
    "System.Xml.Schema"."Compositor"
);
