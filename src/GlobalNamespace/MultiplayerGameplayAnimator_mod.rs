#[cfg(feature = "MultiplayerGameplayAnimator")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerGameplayAnimator {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _activeLightsColor: *mut ColorSO,
    pub _leadingLightsColor: *mut ColorSO,
    pub _failedLightsColor: *mut ColorSO,
    pub _leadingSwitchCrossFadeDuration: f32,
    pub _gameplayLightsAnimators: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut LightsAnimator,
    >,
    pub _allLightsAnimators: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut LightsAnimator,
    >,
    pub _leadPlayerProvider: *mut MultiplayerLeadPlayerProvider,
    pub _multiplayerController: *mut MultiplayerController,
    pub tweeningManager: *mut crate::Tweening::TimeTweeningManager,
    pub connectedPlayer: *mut IConnectedPlayer,
}
#[cfg(feature = "MultiplayerGameplayAnimator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerGameplayAnimator => ""
    ."MultiplayerGameplayAnimator"
);
#[cfg(feature = "MultiplayerGameplayAnimator")]
impl std::ops::Deref for MultiplayerGameplayAnimator {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerGameplayAnimator")]
impl std::ops::DerefMut for MultiplayerGameplayAnimator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerGameplayAnimator")]
impl MultiplayerGameplayAnimator {
    pub fn AnimateNewLeaderSelected(
        &mut self,
        isLeading: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AnimateNewLeaderSelected", (isLeading))?;
        Ok(__cordl_ret)
    }
    pub fn HandleNewLeaderWasSelected(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNewLeaderWasSelected", (userId))?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MultiplayerGameplayAnimator")]
impl quest_hook::libil2cpp::ObjectType for MultiplayerGameplayAnimator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}