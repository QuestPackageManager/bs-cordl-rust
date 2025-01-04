#[cfg(feature = "System+Net+ReadState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ReadState {
    #[default]
    Aborted = 4i32,
    Content = 3i32,
    Headers = 2i32,
    None = 0i32,
    Status = 1i32,
}
#[cfg(feature = "System+Net+ReadState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::ReadState => "System.Net"
    ."ReadState"
);
