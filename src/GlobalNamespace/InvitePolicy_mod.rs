#[cfg(feature = "InvitePolicy")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InvitePolicy {
    #[default]
    AnyoneCanInvite = 1i32,
    NobodyCanInvite = 2i32,
    OnlyConnectionOwnerCanInvite = 0i32,
}
#[cfg(feature = "InvitePolicy")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::InvitePolicy => ""
    ."InvitePolicy"
);
