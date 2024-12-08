#[cfg(feature = "MultiplayerConnectedPlayerFacade+Factory")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerConnectedPlayerFacade_Factory {
    __cordl_parent: crate::Zenject::PlaceholderFactory_3<
        *mut crate::GlobalNamespace::IConnectedPlayer,
        crate::GlobalNamespace::MultiplayerPlayerStartState,
        *mut crate::GlobalNamespace::MultiplayerConnectedPlayerFacade,
    >,
}
#[cfg(feature = "MultiplayerConnectedPlayerFacade+Factory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerConnectedPlayerFacade_Factory => ""
    ."MultiplayerConnectedPlayerFacade/Factory"
);
#[cfg(feature = "MultiplayerConnectedPlayerFacade+Factory")]
impl std::ops::Deref
for crate::GlobalNamespace::MultiplayerConnectedPlayerFacade_Factory {
    type Target = crate::Zenject::PlaceholderFactory_3<
        *mut crate::GlobalNamespace::IConnectedPlayer,
        crate::GlobalNamespace::MultiplayerPlayerStartState,
        *mut crate::GlobalNamespace::MultiplayerConnectedPlayerFacade,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerFacade+Factory")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerConnectedPlayerFacade_Factory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerFacade+Factory")]
impl crate::GlobalNamespace::MultiplayerConnectedPlayerFacade_Factory {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerFacade+Factory")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerConnectedPlayerFacade_Factory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerFacade")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerConnectedPlayerFacade {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _outroAnimator: *mut crate::UnityEngine::GameObject,
    pub _bigAvatarAnimator: *mut crate::GlobalNamespace::MultiplayerBigAvatarAnimator,
    pub _bigAvatarDisappearDuration: f32,
    pub _bigAvatarDisappearEasing: crate::GlobalNamespace::EaseType,
    pub _songTimeSyncController: *mut crate::GlobalNamespace::MultiplayerConnectedPlayerSongTimeSyncController,
    pub _introAnimator: *mut crate::GlobalNamespace::MultiplayerConnectedPlayerIntroAnimator,
    pub _beatmapObjectManager: *mut crate::GlobalNamespace::BeatmapObjectManager,
    pub _scoreDiffText: *mut crate::GlobalNamespace::MultiplayerScoreDiffText,
    pub _beatmapObjectEventManager: *mut crate::GlobalNamespace::IConnectedPlayerBeatmapObjectEventManager,
}
#[cfg(feature = "MultiplayerConnectedPlayerFacade")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerConnectedPlayerFacade => ""
    ."MultiplayerConnectedPlayerFacade"
);
#[cfg(feature = "MultiplayerConnectedPlayerFacade")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerConnectedPlayerFacade {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerFacade")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerConnectedPlayerFacade {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerFacade")]
impl crate::GlobalNamespace::MultiplayerConnectedPlayerFacade {
    #[cfg(feature = "MultiplayerConnectedPlayerFacade+Factory")]
    pub type Factory = crate::GlobalNamespace::MultiplayerConnectedPlayerFacade_Factory;
    pub fn HideBigAvatar(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HideBigAvatar", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn PauseSpawning(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PauseSpawning", ())?;
        Ok(__cordl_ret)
    }
    pub fn ResumeSpawning(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResumeSpawning", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetSongStartSyncTime(
        &mut self,
        songStartSyncTime: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSongStartSyncTime", (songStartSyncTime))?;
        Ok(__cordl_ret)
    }
    pub fn __ForceStopSong(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("__ForceStopSong", ())?;
        Ok(__cordl_ret)
    }
    pub fn __GetPlayerAvatar(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::BeatSaber::AvatarCore::Avatar> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BeatSaber::AvatarCore::Avatar = __cordl_object
            .invoke("__GetPlayerAvatar", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_introAnimator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::MultiplayerConnectedPlayerIntroAnimator,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::MultiplayerConnectedPlayerIntroAnimator = __cordl_object
            .invoke("get_introAnimator", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_outroAnimator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("get_outroAnimator", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_scoreDiffText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::MultiplayerScoreDiffText,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::MultiplayerScoreDiffText = __cordl_object
            .invoke("get_scoreDiffText", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerFacade")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerConnectedPlayerFacade {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
