#[cfg(feature = "System+Net+Sockets+TransmitFileOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransmitFileOptions {
    Disconnect = 1i32,
    ReuseSocket = 2i32,
    UseDefaultWorkerThread = 0i32,
    UseKernelApc = 32i32,
    UseSystemThread = 16i32,
    WriteBehind = 4i32,
}
#[cfg(feature = "System+Net+Sockets+TransmitFileOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Sockets::TransmitFileOptions =>
    "System.Net.Sockets"."TransmitFileOptions"
);
