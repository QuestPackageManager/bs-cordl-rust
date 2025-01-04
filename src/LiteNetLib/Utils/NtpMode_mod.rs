#[cfg(feature = "LiteNetLib+Utils+NtpMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NtpMode {
    #[default]
    Client = 3i32,
    Server = 4i32,
}
#[cfg(feature = "LiteNetLib+Utils+NtpMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::Utils::NtpMode => "LiteNetLib.Utils"
    ."NtpMode"
);
