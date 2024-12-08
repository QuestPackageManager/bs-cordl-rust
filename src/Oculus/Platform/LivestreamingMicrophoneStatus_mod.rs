#[cfg(feature = "Oculus+Platform+LivestreamingMicrophoneStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LivestreamingMicrophoneStatus {
    MicrophoneOff = 2i32,
    MicrophoneOn = 1i32,
    Unknown = 0i32,
}
#[cfg(feature = "Oculus+Platform+LivestreamingMicrophoneStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::LivestreamingMicrophoneStatus
    => "Oculus.Platform"."LivestreamingMicrophoneStatus"
);
