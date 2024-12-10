#[cfg(feature = "BGNet+Core+GameLift+GameLiftPlayerSessionProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct GameLiftPlayerSessionProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _networkConfig: *mut crate::GlobalNamespace::INetworkConfig,
    pub _graphAPIClient: *mut crate::GlobalNamespace::GraphAPIClient,
    pub _xPlatformAuthFeatureFlag: *mut crate::GlobalNamespace::XPlatformAuthFeatureFlag,
    pub _pingAverages: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        *mut crate::GlobalNamespace::RollingAverage,
    >,
    pub _pingCount: i32,
    pub _lastPingTime: i64,
}
#[cfg(feature = "BGNet+Core+GameLift+GameLiftPlayerSessionProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BGNet::Core::GameLift::GameLiftPlayerSessionProvider => "BGNet.Core.GameLift"
    ."GameLiftPlayerSessionProvider"
);
#[cfg(feature = "BGNet+Core+GameLift+GameLiftPlayerSessionProvider")]
impl std::ops::Deref for crate::BGNet::Core::GameLift::GameLiftPlayerSessionProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGNet+Core+GameLift+GameLiftPlayerSessionProvider")]
impl std::ops::DerefMut for crate::BGNet::Core::GameLift::GameLiftPlayerSessionProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGNet+Core+GameLift+GameLiftPlayerSessionProvider")]
impl crate::BGNet::Core::GameLift::GameLiftPlayerSessionProvider {
    pub const kCancelMatchmakingTicketPath: &'static str = "beat_saber_multiplayer_cancel_matchmaking_ticket";
    pub const kGetMatchmakingInstancePath: &'static str = "beat_saber_get_multiplayer_instance";
    pub const kMatchmakingTimeoutMs: i32 = 120000i32;
    pub const kMaxPingCount: i32 = 10i32;
    pub const kPingFrequencyMs: i32 = 3000i32;
    #[cfg(
        feature = "BGNet+Core+GameLift+GameLiftPlayerSessionProvider+_GetGameLiftPlayerSessionInfo_d__14"
    )]
    pub type _GetGameLiftPlayerSessionInfo_d__14 = crate::BGNet::Core::GameLift::GameLiftPlayerSessionProvider__GetGameLiftPlayerSessionInfo_d__14;
    #[cfg(
        feature = "BGNet+Core+GameLift+GameLiftPlayerSessionProvider+_PingAllAwsGameLiftRegions_d__15"
    )]
    pub type _PingAllAwsGameLiftRegions_d__15 = crate::BGNet::Core::GameLift::GameLiftPlayerSessionProvider__PingAllAwsGameLiftRegions_d__15;
    #[cfg(
        feature = "BGNet+Core+GameLift+GameLiftPlayerSessionProvider+_PingRegionAsync_d__17"
    )]
    pub type _PingRegionAsync_d__17 = crate::BGNet::Core::GameLift::GameLiftPlayerSessionProvider__PingRegionAsync_d__17;
    #[cfg(feature = "BGNet+Core+GameLift+GameLiftPlayerSessionProvider+__c")]
    pub type __c = crate::BGNet::Core::GameLift::GameLiftPlayerSessionProvider___c;
    pub fn GetAverageLatencies(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                i64,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                i64,
            >,
        > = __cordl_object.invoke("GetAverageLatencies", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGameLiftPlayerSessionInfo(
        &mut self,
        authenticationTokenProvider: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IAuthenticationTokenProvider,
        >,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        beatmapLevelSelectionMask: crate::GlobalNamespace::BeatmapLevelSelectionMask,
        gameplayServerConfiguration: crate::GlobalNamespace::GameplayServerConfiguration,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        code: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::BGNet::Core::GameLift::PlayerSessionInfo,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::BGNet::Core::GameLift::PlayerSessionInfo,
            >,
        > = __cordl_object
            .invoke(
                "GetGameLiftPlayerSessionInfo",
                (
                    authenticationTokenProvider,
                    userId,
                    beatmapLevelSelectionMask,
                    gameplayServerConfiguration,
                    secret,
                    code,
                    cancellationToken,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetXPlatformAccessToken(
        &mut self,
        authenticationTokenProvider: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IAuthenticationTokenProvider,
        >,
        cancellationToken: crate::System::Threading::CancellationToken,
        skipCache: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::XPlatformAccessTokenData,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::XPlatformAccessTokenData,
            >,
        > = __cordl_object
            .invoke(
                "GetXPlatformAccessToken",
                (authenticationTokenProvider, cancellationToken, skipCache),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        networkConfig: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INetworkConfig>,
        xPlatformAuthFeatureFlag: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::XPlatformAuthFeatureFlag,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (networkConfig, xPlatformAuthFeatureFlag))?;
        Ok(__cordl_object.into())
    }
    pub fn PingAllAwsGameLiftRegions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PingAllAwsGameLiftRegions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PingRegionAsync(
        &mut self,
        awsRegion: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::System::ValueTuple_2<
                    *mut quest_hook::libil2cpp::Il2CppString,
                    i64,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::System::ValueTuple_2<
                    *mut quest_hook::libil2cpp::Il2CppString,
                    i64,
                >,
            >,
        > = __cordl_object.invoke("PingRegionAsync", (awsRegion))?;
        Ok(__cordl_ret.into())
    }
    pub fn PollUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PollUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _PingAllAwsGameLiftRegions_b__15_0(
        &mut self,
        region: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::System::ValueTuple_2<
                    *mut quest_hook::libil2cpp::Il2CppString,
                    i64,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::System::ValueTuple_2<
                    *mut quest_hook::libil2cpp::Il2CppString,
                    i64,
                >,
            >,
        > = __cordl_object.invoke("<PingAllAwsGameLiftRegions>b__15_0", (region))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        networkConfig: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INetworkConfig>,
        xPlatformAuthFeatureFlag: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::XPlatformAuthFeatureFlag,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (networkConfig, xPlatformAuthFeatureFlag))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGNet+Core+GameLift+GameLiftPlayerSessionProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGNet::Core::GameLift::GameLiftPlayerSessionProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
