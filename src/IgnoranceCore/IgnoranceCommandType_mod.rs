#[cfg(feature = "IgnoranceCore+IgnoranceCommandType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IgnoranceCommandType {
    ClientStatusRequest = 1i32,
    ClientWantsToStop = 0i32,
    ServerKickPeer = 2i32,
    ServerStatusRequest = 3i32,
}
#[cfg(feature = "IgnoranceCore+IgnoranceCommandType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::IgnoranceCore::IgnoranceCommandType =>
    "IgnoranceCore"."IgnoranceCommandType"
);
