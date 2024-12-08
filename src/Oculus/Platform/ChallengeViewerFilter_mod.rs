#[cfg(feature = "Oculus+Platform+ChallengeViewerFilter")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChallengeViewerFilter {
    AllVisible = 1i32,
    Invited = 3i32,
    Participating = 2i32,
    ParticipatingOrInvited = 4i32,
    Unknown = 0i32,
}
#[cfg(feature = "Oculus+Platform+ChallengeViewerFilter")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::ChallengeViewerFilter =>
    "Oculus.Platform"."ChallengeViewerFilter"
);
