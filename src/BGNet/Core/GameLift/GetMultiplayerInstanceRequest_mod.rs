#[cfg(feature = "BGNet+Core+GameLift+GetMultiplayerInstanceRequest")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GetMultiplayerInstanceRequest {
    pub version: *mut crate::System::String,
    pub serviceEnvironment: crate::GlobalNamespace::ServiceEnvironment,
    pub singleUseAuthToken: *mut crate::System::String,
    pub beatmapLevelSelectionMask: crate::GlobalNamespace::BeatmapLevelSelectionMask,
    pub gameplayServerConfiguration: crate::GlobalNamespace::GameplayServerConfiguration,
    pub userId: *mut crate::System::String,
    pub privateGameSecret: *mut crate::System::String,
    pub privateGameCode: *mut crate::System::String,
    pub platform: crate::GlobalNamespace::AuthenticationToken_Platform,
    pub authUserId: *mut crate::System::String,
    pub gameliftRegionLatencies: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        i64,
    >,
    pub ticketId: *mut crate::System::String,
    pub placementId: *mut crate::System::String,
}
#[cfg(feature = "BGNet+Core+GameLift+GetMultiplayerInstanceRequest")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BGNet::Core::GameLift::GetMultiplayerInstanceRequest => "BGNet.Core.GameLift"
    ."GetMultiplayerInstanceRequest"
);
#[cfg(feature = "BGNet+Core+GameLift+GetMultiplayerInstanceRequest")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BGNet::Core::GameLift::GetMultiplayerInstanceRequest {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BGNet+Core+GameLift+GetMultiplayerInstanceRequest")]
impl crate::BGNet::Core::GameLift::GetMultiplayerInstanceRequest {
    pub fn _ctor(
        &mut self,
        version: *mut crate::System::String,
        serviceEnvironment: crate::GlobalNamespace::ServiceEnvironment,
        userId: *mut crate::System::String,
        beatmapLevelSelectionMask: crate::GlobalNamespace::BeatmapLevelSelectionMask,
        gameplayServerConfiguration: crate::GlobalNamespace::GameplayServerConfiguration,
        platform: crate::GlobalNamespace::AuthenticationToken_Platform,
        authUserId: *mut crate::System::String,
        singleUseAuthToken: *mut crate::System::String,
        privateGameSecret: *mut crate::System::String,
        privateGameCode: *mut crate::System::String,
        gameliftRegionLatencies: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            i64,
        >,
        ticketId: *mut crate::System::String,
        placementId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (
                version,
                serviceEnvironment,
                userId,
                beatmapLevelSelectionMask,
                gameplayServerConfiguration,
                platform,
                authUserId,
                singleUseAuthToken,
                privateGameSecret,
                privateGameCode,
                gameliftRegionLatencies,
                ticketId,
                placementId,
            ),
        )?;
        Ok(__cordl_ret)
    }
}
