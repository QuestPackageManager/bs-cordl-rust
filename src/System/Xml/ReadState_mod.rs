#[cfg(feature = "System+Xml+ReadState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ReadState {
    #[default]
    Closed = 4i32,
    EndOfFile = 3i32,
    Error = 2i32,
    Initial = 0i32,
    Interactive = 1i32,
}
#[cfg(feature = "System+Xml+ReadState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::ReadState => "System.Xml"
    ."ReadState"
);
