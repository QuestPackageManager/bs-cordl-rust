#[cfg(feature = "cordl_class_MultiplayerLocalInactivePlayerFacade")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLocalInactivePlayerFacade {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _inactivePlayerSongSyncController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerLocalInactivePlayerSongSyncController,
    >,
    pub _spectatorController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerSpectatorController,
    >,
    pub _introAnimator: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Playables::PlayableDirector,
    >,
    pub _outroAnimator: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerLocalInactivePlayerOutroAnimator,
    >,
    pub playerDidFinishEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::MultiplayerLevelCompletionResults,
            >,
        >,
    >,
    pub playerNetworkDidFailedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::MultiplayerLevelCompletionResults,
            >,
        >,
    >,
}
#[cfg(feature = "cordl_class_MultiplayerLocalInactivePlayerFacade")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MultiplayerLocalInactivePlayerFacade";
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
#[cfg(feature = "MultiplayerLocalInactivePlayerFacade")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalInactivePlayerFacade")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalInactivePlayerFacade")]
impl crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade {
    #[cfg(feature = "MultiplayerLocalInactivePlayerFacade+Factory")]
    pub type Factory = crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade_Factory;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ReportPlayerDidFinish(
        &mut self,
        results: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::MultiplayerLevelCompletionResults,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ReportPlayerDidFinish")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReportPlayerDidFinish", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (results))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReportPlayerNetworkDidFailed(
        &mut self,
        results: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::MultiplayerLevelCompletionResults,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ReportPlayerNetworkDidFailed")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReportPlayerNetworkDidFailed", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (results))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_playerDidFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MultiplayerLevelCompletionResults,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::MultiplayerLevelCompletionResults,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("add_playerDidFinishEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "add_playerDidFinishEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_playerNetworkDidFailedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MultiplayerLevelCompletionResults,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::MultiplayerLevelCompletionResults,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("add_playerNetworkDidFailedEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "add_playerNetworkDidFailedEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_introAnimator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                        0usize,
                    >("get_introAnimator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_introAnimator", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_introPlayableDirector(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Playables::PlayableDirector>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Playables::PlayableDirector,
                        >,
                        0usize,
                    >("get_introPlayableDirector")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_introPlayableDirector", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Playables::PlayableDirector,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_outroAnimator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLocalInactivePlayerOutroAnimator,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::MultiplayerLocalInactivePlayerOutroAnimator,
                        >,
                        0usize,
                    >("get_outroAnimator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_outroAnimator", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLocalInactivePlayerOutroAnimator,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_songController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IStartSeekSongController>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::IStartSeekSongController,
                        >,
                        0usize,
                    >("get_songController")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_songController", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IStartSeekSongController,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_spectatorController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MultiplayerSpectatorController>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::MultiplayerSpectatorController,
                        >,
                        0usize,
                    >("get_spectatorController")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_spectatorController", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerSpectatorController,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn remove_playerDidFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MultiplayerLevelCompletionResults,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::MultiplayerLevelCompletionResults,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("remove_playerDidFinishEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "remove_playerDidFinishEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_playerNetworkDidFailedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MultiplayerLevelCompletionResults,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::MultiplayerLevelCompletionResults,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("remove_playerNetworkDidFailedEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "remove_playerNetworkDidFailedEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_MultiplayerLocalInactivePlayerFacade")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerLocalInactivePlayerFacade")]
impl AsRef<crate::GlobalNamespace::IMultiplayerLevelEndActionsListener>
for crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade {
    fn as_ref(&self) -> &crate::GlobalNamespace::IMultiplayerLevelEndActionsListener {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultiplayerLocalInactivePlayerFacade")]
impl AsMut<crate::GlobalNamespace::IMultiplayerLevelEndActionsListener>
for crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::IMultiplayerLevelEndActionsListener {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultiplayerLocalInactivePlayerFacade")]
impl AsRef<crate::GlobalNamespace::IMultiplayerLevelEndActionsPublisher>
for crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade {
    fn as_ref(&self) -> &crate::GlobalNamespace::IMultiplayerLevelEndActionsPublisher {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultiplayerLocalInactivePlayerFacade")]
impl AsMut<crate::GlobalNamespace::IMultiplayerLevelEndActionsPublisher>
for crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::IMultiplayerLevelEndActionsPublisher {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultiplayerLocalInactivePlayerFacade")]
impl AsRef<crate::GlobalNamespace::IStartSeekSongControllerProvider>
for crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade {
    fn as_ref(&self) -> &crate::GlobalNamespace::IStartSeekSongControllerProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultiplayerLocalInactivePlayerFacade")]
impl AsMut<crate::GlobalNamespace::IStartSeekSongControllerProvider>
for crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::IStartSeekSongControllerProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "cordl_class_MultiplayerLocalInactivePlayerFacade+Factory")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLocalInactivePlayerFacade_Factory {
    __cordl_parent: crate::Zenject::PlaceholderFactory_2<
        crate::GlobalNamespace::MultiplayerPlayerStartState,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade,
        >,
    >,
}
#[cfg(feature = "cordl_class_MultiplayerLocalInactivePlayerFacade+Factory")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade_Factory {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MultiplayerLocalInactivePlayerFacade/Factory";
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
#[cfg(feature = "MultiplayerLocalInactivePlayerFacade+Factory")]
impl std::ops::Deref
for crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade_Factory {
    type Target = crate::Zenject::PlaceholderFactory_2<
        crate::GlobalNamespace::MultiplayerPlayerStartState,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade,
        >,
    >;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalInactivePlayerFacade+Factory")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade_Factory {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalInactivePlayerFacade+Factory")]
impl crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade_Factory {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_MultiplayerLocalInactivePlayerFacade+Factory")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade_Factory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
