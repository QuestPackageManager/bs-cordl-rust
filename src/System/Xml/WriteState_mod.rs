#[cfg(feature = "System+Xml+WriteState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WriteState {
    #[default]
    Attribute = 3i32,
    Closed = 5i32,
    Content = 4i32,
    Element = 2i32,
    Error = 6i32,
    Prolog = 1i32,
    Start = 0i32,
}
#[cfg(feature = "System+Xml+WriteState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::WriteState => "System.Xml"
    ."WriteState"
);
