#[cfg(feature = "MultiplayerGameplayAnimator")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerGameplayAnimator {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _activeLightsColor: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSO>,
    pub _leadingLightsColor: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSO>,
    pub _failedLightsColor: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSO>,
    pub _leadingSwitchCrossFadeDuration: f32,
    pub _gameplayLightsAnimators: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::GlobalNamespace::LightsAnimator>,
    >,
    pub _allLightsAnimators: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::GlobalNamespace::LightsAnimator>,
    >,
    pub _leadPlayerProvider: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerLeadPlayerProvider,
    >,
    pub _multiplayerController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerController,
    >,
    pub tweeningManager: quest_hook::libil2cpp::Gc<crate::Tweening::TimeTweeningManager>,
    pub connectedPlayer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IConnectedPlayer,
    >,
}
#[cfg(feature = "MultiplayerGameplayAnimator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerGameplayAnimator =>
    ""."MultiplayerGameplayAnimator"
);
#[cfg(feature = "MultiplayerGameplayAnimator")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerGameplayAnimator {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerGameplayAnimator")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerGameplayAnimator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerGameplayAnimator")]
impl crate::GlobalNamespace::MultiplayerGameplayAnimator {
    pub fn AnimateNewLeaderSelected(
        &mut self,
        isLeading: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AnimateNewLeaderSelected", (isLeading))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleNewLeaderWasSelected(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNewLeaderWasSelected", (userId))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerGameplayAnimator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerGameplayAnimator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
