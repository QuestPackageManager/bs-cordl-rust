#[cfg(feature = "MultiplayerLocalActivePlayerGameplayAnimator")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLocalActivePlayerGameplayAnimator {
    __cordl_parent: crate::GlobalNamespace::MultiplayerGameplayAnimator,
    pub _coreGameHUDController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::CoreGameHUDController,
    >,
    pub _multiplayerPositionHUDController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerPositionHUDController,
    >,
    pub _levelEndActionsPublisher: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IMultiplayerLevelEndActionsPublisher,
    >,
    pub _fadeOutHUDTween: quest_hook::libil2cpp::Gc<crate::Tweening::FloatTween>,
    pub _initialized: bool,
}
#[cfg(feature = "MultiplayerLocalActivePlayerGameplayAnimator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerLocalActivePlayerGameplayAnimator => ""
    ."MultiplayerLocalActivePlayerGameplayAnimator"
);
#[cfg(feature = "MultiplayerLocalActivePlayerGameplayAnimator")]
impl std::ops::Deref
for crate::GlobalNamespace::MultiplayerLocalActivePlayerGameplayAnimator {
    type Target = crate::GlobalNamespace::MultiplayerGameplayAnimator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalActivePlayerGameplayAnimator")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerLocalActivePlayerGameplayAnimator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalActivePlayerGameplayAnimator")]
impl crate::GlobalNamespace::MultiplayerLocalActivePlayerGameplayAnimator {
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
    pub fn HandlePlayerDidFinish(
        &mut self,
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerDidFinish", (levelCompletionResults))?;
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
    pub fn InitializeIfNeeded(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeIfNeeded", ())?;
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
    pub fn TransitionIntoFailedState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TransitionIntoFailedState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _InitializeIfNeeded_b__6_0(
        &mut self,
        f: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<InitializeIfNeeded>b__6_0", (f))?;
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
#[cfg(feature = "MultiplayerLocalActivePlayerGameplayAnimator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerLocalActivePlayerGameplayAnimator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
