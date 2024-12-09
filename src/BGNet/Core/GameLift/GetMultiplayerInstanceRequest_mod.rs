#[cfg(feature = "BGNet+Core+GameLift+GetMultiplayerInstanceRequest")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GetMultiplayerInstanceRequest {
    pub version: *mut quest_hook::libil2cpp::Il2CppString,
    pub serviceEnvironment: crate::GlobalNamespace::ServiceEnvironment,
    pub singleUseAuthToken: *mut quest_hook::libil2cpp::Il2CppString,
    pub beatmapLevelSelectionMask: crate::GlobalNamespace::BeatmapLevelSelectionMask,
    pub gameplayServerConfiguration: crate::GlobalNamespace::GameplayServerConfiguration,
    pub userId: *mut quest_hook::libil2cpp::Il2CppString,
    pub privateGameSecret: *mut quest_hook::libil2cpp::Il2CppString,
    pub privateGameCode: *mut quest_hook::libil2cpp::Il2CppString,
    pub platform: crate::GlobalNamespace::AuthenticationToken_Platform,
    pub authUserId: *mut quest_hook::libil2cpp::Il2CppString,
    pub gameliftRegionLatencies: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        i64,
    >,
    pub ticketId: *mut quest_hook::libil2cpp::Il2CppString,
    pub placementId: *mut quest_hook::libil2cpp::Il2CppString,
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
        version: *mut quest_hook::libil2cpp::Il2CppString,
        serviceEnvironment: crate::GlobalNamespace::ServiceEnvironment,
        userId: *mut quest_hook::libil2cpp::Il2CppString,
        beatmapLevelSelectionMask: crate::GlobalNamespace::BeatmapLevelSelectionMask,
        gameplayServerConfiguration: crate::GlobalNamespace::GameplayServerConfiguration,
        platform: crate::GlobalNamespace::AuthenticationToken_Platform,
        authUserId: *mut quest_hook::libil2cpp::Il2CppString,
        singleUseAuthToken: *mut quest_hook::libil2cpp::Il2CppString,
        privateGameSecret: *mut quest_hook::libil2cpp::Il2CppString,
        privateGameCode: *mut quest_hook::libil2cpp::Il2CppString,
        gameliftRegionLatencies: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            i64,
        >,
        ticketId: *mut quest_hook::libil2cpp::Il2CppString,
        placementId: *mut quest_hook::libil2cpp::Il2CppString,
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
