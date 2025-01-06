#[cfg(feature = "IgnoranceCore+IgnoranceClientStats")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct IgnoranceClientStats {
    pub RTT: u32,
    pub BytesReceived: u64,
    pub BytesSent: u64,
    pub PacketsReceived: u64,
    pub PacketsSent: u64,
    pub PacketsLost: u64,
}
#[cfg(feature = "IgnoranceCore+IgnoranceClientStats")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::IgnoranceCore::IgnoranceClientStats =>
    "IgnoranceCore"."IgnoranceClientStats"
);
#[cfg(feature = "IgnoranceCore+IgnoranceClientStats")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::IgnoranceCore::IgnoranceClientStats {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "IgnoranceCore+IgnoranceClientStats")]
impl crate::IgnoranceCore::IgnoranceClientStats {}
