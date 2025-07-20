#[cfg(feature = "BGNet+Core+GameLift+IGameLiftPlayerSessionProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct IGameLiftPlayerSessionProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BGNet+Core+GameLift+IGameLiftPlayerSessionProvider")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BGNet::Core::GameLift::IGameLiftPlayerSessionProvider {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BGNet.Core.GameLift";
    const CLASS_NAME: &'static str = "IGameLiftPlayerSessionProvider";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "BGNet+Core+GameLift+IGameLiftPlayerSessionProvider")]
impl std::ops::Deref for crate::BGNet::Core::GameLift::IGameLiftPlayerSessionProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGNet+Core+GameLift+IGameLiftPlayerSessionProvider")]
impl std::ops::DerefMut
for crate::BGNet::Core::GameLift::IGameLiftPlayerSessionProvider {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGNet+Core+GameLift+IGameLiftPlayerSessionProvider")]
impl crate::BGNet::Core::GameLift::IGameLiftPlayerSessionProvider {
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
                quest_hook::libil2cpp::Gc<
                    crate::BGNet::Core::GameLift::PlayerSessionInfo,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IAuthenticationTokenProvider,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::GlobalNamespace::BeatmapLevelSelectionMask,
                            crate::GlobalNamespace::GameplayServerConfiguration,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::System::Threading::CancellationToken,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::BGNet::Core::GameLift::PlayerSessionInfo,
                                >,
                            >,
                        >,
                        7usize,
                    >("GetGameLiftPlayerSessionInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetGameLiftPlayerSessionInfo", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<
                    crate::BGNet::Core::GameLift::PlayerSessionInfo,
                >,
            >,
        > = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        authenticationTokenProvider,
                        userId,
                        beatmapLevelSelectionMask,
                        gameplayServerConfiguration,
                        secret,
                        code,
                        cancellationToken,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
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
#[cfg(feature = "BGNet+Core+GameLift+IGameLiftPlayerSessionProvider")]
impl AsRef<crate::GlobalNamespace::IPollable>
for crate::BGNet::Core::GameLift::IGameLiftPlayerSessionProvider {
    fn as_ref(&self) -> &crate::GlobalNamespace::IPollable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BGNet+Core+GameLift+IGameLiftPlayerSessionProvider")]
impl AsMut<crate::GlobalNamespace::IPollable>
for crate::BGNet::Core::GameLift::IGameLiftPlayerSessionProvider {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IPollable {
        unsafe { std::mem::transmute(self) }
    }
}
