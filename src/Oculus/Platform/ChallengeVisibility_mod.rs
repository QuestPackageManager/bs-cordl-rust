#[cfg(feature = "Oculus+Platform+ChallengeVisibility")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ChallengeVisibility {
    #[default]
    InviteOnly = 1i32,
    Private = 3i32,
    Public = 2i32,
    Unknown = 0i32,
}
#[cfg(feature = "Oculus+Platform+ChallengeVisibility")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::ChallengeVisibility =>
    "Oculus.Platform"."ChallengeVisibility"
);
