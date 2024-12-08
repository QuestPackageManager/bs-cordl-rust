#[cfg(feature = "MultiplayerLocalInactivePlayerFacade+Factory")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLocalInactivePlayerFacade_Factory {
    __cordl_parent: crate::Zenject::PlaceholderFactory_2<
        MultiplayerPlayerStartState,
        *mut MultiplayerLocalInactivePlayerFacade,
    >,
}
#[cfg(feature = "MultiplayerLocalInactivePlayerFacade+Factory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerLocalInactivePlayerFacade_Factory => ""
    ."MultiplayerLocalInactivePlayerFacade/Factory"
);
#[cfg(feature = "MultiplayerLocalInactivePlayerFacade+Factory")]
impl std::ops::Deref
for crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade_Factory {
    type Target = crate::Zenject::PlaceholderFactory_2<
        MultiplayerPlayerStartState,
        *mut MultiplayerLocalInactivePlayerFacade,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalInactivePlayerFacade+Factory")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade_Factory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalInactivePlayerFacade+Factory")]
impl crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade_Factory {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MultiplayerLocalInactivePlayerFacade+Factory")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade_Factory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerLocalInactivePlayerFacade")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLocalInactivePlayerFacade {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _inactivePlayerSongSyncController: *mut MultiplayerLocalInactivePlayerSongSyncController,
    pub _spectatorController: *mut MultiplayerSpectatorController,
    pub _introAnimator: *mut crate::UnityEngine::Playables::PlayableDirector,
    pub _outroAnimator: *mut MultiplayerLocalInactivePlayerOutroAnimator,
    pub playerDidFinishEvent: *mut crate::System::Action_1<
        *mut MultiplayerLevelCompletionResults,
    >,
    pub playerNetworkDidFailedEvent: *mut crate::System::Action_1<
        *mut MultiplayerLevelCompletionResults,
    >,
}
#[cfg(feature = "MultiplayerLocalInactivePlayerFacade")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerLocalInactivePlayerFacade => ""
    ."MultiplayerLocalInactivePlayerFacade"
);
#[cfg(feature = "MultiplayerLocalInactivePlayerFacade")]
impl std::ops::Deref for MultiplayerLocalInactivePlayerFacade {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalInactivePlayerFacade")]
impl std::ops::DerefMut for MultiplayerLocalInactivePlayerFacade {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalInactivePlayerFacade")]
impl MultiplayerLocalInactivePlayerFacade {
    #[cfg(feature = "MultiplayerLocalInactivePlayerFacade+Factory")]
    pub type Factory = crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade_Factory;
    pub fn get_songController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut IStartSeekSongController> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut IStartSeekSongController = __cordl_object
            .invoke("get_songController", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_introAnimator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("get_introAnimator", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_playerDidFinishEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut MultiplayerLevelCompletionResults>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playerDidFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_outroAnimator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut MultiplayerLocalInactivePlayerOutroAnimator,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut MultiplayerLocalInactivePlayerOutroAnimator = __cordl_object
            .invoke("get_outroAnimator", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_playerNetworkDidFailedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut MultiplayerLevelCompletionResults>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playerNetworkDidFailedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_playerNetworkDidFailedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut MultiplayerLevelCompletionResults>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playerNetworkDidFailedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ReportPlayerDidFinish(
        &mut self,
        results: *mut MultiplayerLevelCompletionResults,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReportPlayerDidFinish", (results))?;
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
    pub fn get_introPlayableDirector(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::Playables::PlayableDirector,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Playables::PlayableDirector = __cordl_object
            .invoke("get_introPlayableDirector", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReportPlayerNetworkDidFailed(
        &mut self,
        results: *mut MultiplayerLevelCompletionResults,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReportPlayerNetworkDidFailed", (results))?;
        Ok(__cordl_ret)
    }
    pub fn get_spectatorController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut MultiplayerSpectatorController> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut MultiplayerSpectatorController = __cordl_object
            .invoke("get_spectatorController", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_playerDidFinishEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut MultiplayerLevelCompletionResults>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playerDidFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MultiplayerLocalInactivePlayerFacade")]
impl quest_hook::libil2cpp::ObjectType for MultiplayerLocalInactivePlayerFacade {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
