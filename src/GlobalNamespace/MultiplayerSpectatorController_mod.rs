#[cfg(feature = "MultiplayerSpectatorController")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerSpectatorController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _spotManager: *mut MultiplayerSpectatingSpotManager,
    pub _songController: *mut MultiplayerLocalInactivePlayerSongSyncController,
    pub _multiplayerController: *mut MultiplayerController,
    pub _fadeInOutController: *mut FadeInOutController,
    pub spectatingSpotDidChangeEvent: *mut crate::System::Action_1<
        *mut IMultiplayerSpectatingSpot,
    >,
    pub _transform: *mut crate::UnityEngine::Transform,
    pub _currentSpot: *mut IMultiplayerSpectatingSpot,
}
#[cfg(feature = "MultiplayerSpectatorController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerSpectatorController => ""
    ."MultiplayerSpectatorController"
);
#[cfg(feature = "MultiplayerSpectatorController")]
impl std::ops::Deref for MultiplayerSpectatorController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerSpectatorController")]
impl std::ops::DerefMut for MultiplayerSpectatorController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerSpectatorController")]
impl MultiplayerSpectatorController {
    #[cfg(
        feature = "MultiplayerSpectatorController+_SwitchToDefaultSpotWithFadeCoroutine_d__17"
    )]
    pub type _SwitchToDefaultSpotWithFadeCoroutine_d__17 = crate::GlobalNamespace::MultiplayerSpectatorController__SwitchToDefaultSpotWithFadeCoroutine_d__17;
    #[cfg(
        feature = "MultiplayerSpectatorController+_SwitchToDefaultSpotCoroutine_d__18"
    )]
    pub type _SwitchToDefaultSpotCoroutine_d__18 = crate::GlobalNamespace::MultiplayerSpectatorController__SwitchToDefaultSpotCoroutine_d__18;
    pub fn add_spectatingSpotDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut IMultiplayerSpectatingSpot>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_spectatingSpotDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn SwitchToDefaultSpotWithFadeCoroutine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("SwitchToDefaultSpotWithFadeCoroutine", ())?;
        Ok(__cordl_ret)
    }
    pub fn SwitchToDefaultSpotCoroutine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("SwitchToDefaultSpotCoroutine", ())?;
        Ok(__cordl_ret)
    }
    pub fn SwitchToNext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SwitchToNext", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_currentSpot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut IMultiplayerSpectatingSpot> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut IMultiplayerSpectatingSpot = __cordl_object
            .invoke("get_currentSpot", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn SwitchToDefaultSpot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SwitchToDefaultSpot", ())?;
        Ok(__cordl_ret)
    }
    pub fn SwitchToSpectatingSpot(
        &mut self,
        spectatingSpot: *mut IMultiplayerSpectatingSpot,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SwitchToSpectatingSpot", (spectatingSpot))?;
        Ok(__cordl_ret)
    }
    pub fn HandleStateChanged(
        &mut self,
        state: crate::GlobalNamespace::MultiplayerController_State,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleStateChanged", (state))?;
        Ok(__cordl_ret)
    }
    pub fn SwitchToPrev(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SwitchToPrev", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_spectatingSpotDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut IMultiplayerSpectatingSpot>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_spectatingSpotDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MultiplayerSpectatorController")]
impl quest_hook::libil2cpp::ObjectType for MultiplayerSpectatorController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
