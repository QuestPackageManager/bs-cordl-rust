#[cfg(feature = "System+Net+CloseExState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloseExState {
    Abort = 1i32,
    Normal = 0i32,
    Silent = 2i32,
}
#[cfg(feature = "System+Net+CloseExState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::CloseExState => "System.Net"
    ."CloseExState"
);
