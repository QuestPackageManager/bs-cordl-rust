#[cfg(feature = "IgnoranceCore+IgnoranceIncomingPacket")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct IgnoranceIncomingPacket {
    pub Channel: u8,
    pub NativePeerId: u32,
    pub Payload: crate::ENet::Packet,
}
#[cfg(feature = "IgnoranceCore+IgnoranceIncomingPacket")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::IgnoranceCore::IgnoranceIncomingPacket =>
    "IgnoranceCore"."IgnoranceIncomingPacket"
);
#[cfg(feature = "IgnoranceCore+IgnoranceIncomingPacket")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::IgnoranceCore::IgnoranceIncomingPacket {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "IgnoranceCore+IgnoranceIncomingPacket")]
impl crate::IgnoranceCore::IgnoranceIncomingPacket {}
