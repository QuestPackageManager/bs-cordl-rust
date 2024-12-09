#[cfg(feature = "BGNet+Core+GameLift+IGameLiftPlayerSessionProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct IGameLiftPlayerSessionProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BGNet+Core+GameLift+IGameLiftPlayerSessionProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BGNet::Core::GameLift::IGameLiftPlayerSessionProvider => "BGNet.Core.GameLift"
    ."IGameLiftPlayerSessionProvider"
);
#[cfg(feature = "BGNet+Core+GameLift+IGameLiftPlayerSessionProvider")]
impl std::ops::Deref for crate::BGNet::Core::GameLift::IGameLiftPlayerSessionProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGNet+Core+GameLift+IGameLiftPlayerSessionProvider")]
impl std::ops::DerefMut
for crate::BGNet::Core::GameLift::IGameLiftPlayerSessionProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGNet+Core+GameLift+IGameLiftPlayerSessionProvider")]
impl crate::BGNet::Core::GameLift::IGameLiftPlayerSessionProvider {
    pub fn GetGameLiftPlayerSessionInfo(
        &mut self,
        authenticationTokenProvider: *mut crate::GlobalNamespace::IAuthenticationTokenProvider,
        userId: *mut quest_hook::libil2cpp::Il2CppString,
        beatmapLevelSelectionMask: crate::GlobalNamespace::BeatmapLevelSelectionMask,
        gameplayServerConfiguration: crate::GlobalNamespace::GameplayServerConfiguration,
        secret: *mut quest_hook::libil2cpp::Il2CppString,
        code: *mut quest_hook::libil2cpp::Il2CppString,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::BGNet::Core::GameLift::PlayerSessionInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::BGNet::Core::GameLift::PlayerSessionInfo,
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
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "BGNet+Core+GameLift+IGameLiftPlayerSessionProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGNet::Core::GameLift::IGameLiftPlayerSessionProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
