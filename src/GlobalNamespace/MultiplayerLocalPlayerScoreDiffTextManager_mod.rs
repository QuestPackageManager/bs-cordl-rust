#[cfg(feature = "MultiplayerLocalPlayerScoreDiffTextManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLocalPlayerScoreDiffTextManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _scoreDiffText: *mut MultiplayerScoreDiffText,
    pub _multiplayerController: *mut MultiplayerController,
    pub _scoreProvider: *mut MultiplayerScoreProvider,
    pub _hudInitData: *mut crate::GlobalNamespace::CoreGameHUDController_InitData,
    pub _multiplayerSessionManager: *mut IMultiplayerSessionManager,
    pub _timeToNextUpdate: f32,
    pub _wasLocalPlayerLeader: crate::System::Nullable_1<bool>,
}
#[cfg(feature = "MultiplayerLocalPlayerScoreDiffTextManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerLocalPlayerScoreDiffTextManager => ""
    ."MultiplayerLocalPlayerScoreDiffTextManager"
);
#[cfg(feature = "MultiplayerLocalPlayerScoreDiffTextManager")]
impl std::ops::Deref for MultiplayerLocalPlayerScoreDiffTextManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalPlayerScoreDiffTextManager")]
impl std::ops::DerefMut for MultiplayerLocalPlayerScoreDiffTextManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalPlayerScoreDiffTextManager")]
impl MultiplayerLocalPlayerScoreDiffTextManager {
    pub const kUpdateInterval: f32 = 0.5f32;
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
    pub fn HandleStateChanged(
        &mut self,
        newState: crate::GlobalNamespace::MultiplayerController_State,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleStateChanged", (newState))?;
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
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
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
#[cfg(feature = "MultiplayerLocalPlayerScoreDiffTextManager")]
impl quest_hook::libil2cpp::ObjectType for MultiplayerLocalPlayerScoreDiffTextManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
