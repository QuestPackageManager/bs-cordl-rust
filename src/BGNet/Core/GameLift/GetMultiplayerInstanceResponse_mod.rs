#[cfg(feature = "BGNet+Core+GameLift+GetMultiplayerInstanceResponse")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GetMultiplayerInstanceResponse {
    pub errorCode: MultiplayerPlacementErrorCode,
    pub playerSessionInfo: *mut crate::BGNet::Core::GameLift::PlayerSessionInfo,
    pub pollIntervalMs: i32,
    pub ticketId: *mut crate::System::String,
    pub ticketStatus: *mut crate::System::String,
    pub placementId: *mut crate::System::String,
    pub placementStatus: *mut crate::System::String,
}
#[cfg(feature = "BGNet+Core+GameLift+GetMultiplayerInstanceResponse")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BGNet::Core::GameLift::GetMultiplayerInstanceResponse => "BGNet.Core.GameLift"
    ."GetMultiplayerInstanceResponse"
);
#[cfg(feature = "BGNet+Core+GameLift+GetMultiplayerInstanceResponse")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BGNet::Core::GameLift::GetMultiplayerInstanceResponse {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BGNet+Core+GameLift+GetMultiplayerInstanceResponse")]
impl crate::BGNet::Core::GameLift::GetMultiplayerInstanceResponse {
    pub fn _ctor(
        &mut self,
        errorCode: MultiplayerPlacementErrorCode,
        playerSessionInfo: *mut crate::BGNet::Core::GameLift::PlayerSessionInfo,
        pollIntervalMs: i32,
        ticketId: *mut crate::System::String,
        ticketStatus: *mut crate::System::String,
        placementId: *mut crate::System::String,
        placementStatus: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (
                errorCode,
                playerSessionInfo,
                pollIntervalMs,
                ticketId,
                ticketStatus,
                placementId,
                placementStatus,
            ),
        )?;
        Ok(__cordl_ret)
    }
}
