#[cfg(feature = "MultiplayerPositionHUDController")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerPositionHUDController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _playerCountText: *mut crate::TMPro::TextMeshProUGUI,
    pub _positionText: *mut crate::TMPro::TextMeshProUGUI,
    pub _canvasGroup: *mut crate::UnityEngine::CanvasGroup,
    pub _firstPlayerAnimationGo: *mut crate::UnityEngine::GameObject,
    pub _scoreProvider: *mut crate::GlobalNamespace::MultiplayerScoreProvider,
    pub _playersManager: *mut crate::GlobalNamespace::MultiplayerPlayersManager,
    pub _initData: *mut crate::GlobalNamespace::CoreGameHUDController_InitData,
    pub _prevPosition: i32,
}
#[cfg(feature = "MultiplayerPositionHUDController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerPositionHUDController => ""
    ."MultiplayerPositionHUDController"
);
#[cfg(feature = "MultiplayerPositionHUDController")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerPositionHUDController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerPositionHUDController")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerPositionHUDController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerPositionHUDController")]
impl crate::GlobalNamespace::MultiplayerPositionHUDController {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
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
    pub fn set_alpha(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_alpha", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerPositionHUDController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerPositionHUDController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
