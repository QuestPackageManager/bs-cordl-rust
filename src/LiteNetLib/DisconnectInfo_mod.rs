#[cfg(feature = "LiteNetLib+DisconnectInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DisconnectInfo {
    pub Reason: crate::LiteNetLib::DisconnectReason,
    pub SocketErrorCode: crate::System::Net::Sockets::SocketError,
    pub AdditionalData: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacketReader>,
}
#[cfg(feature = "LiteNetLib+DisconnectInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::DisconnectInfo => "LiteNetLib"
    ."DisconnectInfo"
);
#[cfg(feature = "LiteNetLib+DisconnectInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::LiteNetLib::DisconnectInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LiteNetLib+DisconnectInfo")]
impl crate::LiteNetLib::DisconnectInfo {}
