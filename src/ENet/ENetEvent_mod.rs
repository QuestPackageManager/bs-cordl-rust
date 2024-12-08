#[cfg(feature = "ENet+ENetEvent")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ENetEvent {
    pub _cordl_type: crate::ENet::EventType,
    pub peer: crate::System::IntPtr,
    pub channelID: u8,
    pub data: u32,
    pub packet: crate::System::IntPtr,
}
#[cfg(feature = "ENet+ENetEvent")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::ENet::ENetEvent => "ENet"."ENetEvent"
);
#[cfg(feature = "ENet+ENetEvent")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::ENet::ENetEvent {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "ENet+ENetEvent")]
impl crate::ENet::ENetEvent {}
