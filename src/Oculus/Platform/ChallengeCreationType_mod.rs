#[cfg(feature = "Oculus+Platform+ChallengeCreationType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChallengeCreationType {
    DeveloperCreated = 2i32,
    Unknown = 0i32,
    UserCreated = 1i32,
}
#[cfg(feature = "Oculus+Platform+ChallengeCreationType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::ChallengeCreationType =>
    "Oculus.Platform"."ChallengeCreationType"
);
