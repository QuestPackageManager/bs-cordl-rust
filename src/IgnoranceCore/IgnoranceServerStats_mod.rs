#[cfg(feature = "IgnoranceCore+IgnoranceServerStats")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct IgnoranceServerStats {
    pub BytesReceived: u64,
    pub BytesSent: u64,
    pub PacketsReceived: u64,
    pub PacketsSent: u64,
    pub PeersCount: u64,
    pub PeerStats: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            i32,
            crate::IgnoranceCore::IgnoranceClientStats,
        >,
    >,
}
#[cfg(feature = "IgnoranceCore+IgnoranceServerStats")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::IgnoranceCore::IgnoranceServerStats =>
    "IgnoranceCore"."IgnoranceServerStats"
);
#[cfg(feature = "IgnoranceCore+IgnoranceServerStats")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::IgnoranceCore::IgnoranceServerStats {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "IgnoranceCore+IgnoranceServerStats")]
impl crate::IgnoranceCore::IgnoranceServerStats {}
