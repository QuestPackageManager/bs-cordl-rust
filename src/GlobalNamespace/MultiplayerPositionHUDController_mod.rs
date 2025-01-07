#[cfg(feature = "MultiplayerPositionHUDController")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerPositionHUDController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _playerCountText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _positionText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _canvasGroup: quest_hook::libil2cpp::Gc<crate::UnityEngine::CanvasGroup>,
    pub _firstPlayerAnimationGo: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::GameObject,
    >,
    pub _scoreProvider: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerScoreProvider,
    >,
    pub _playersManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerPlayersManager,
    >,
    pub _initData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::CoreGameHUDController_InitData,
    >,
    pub _prevPosition: i32,
}
#[cfg(feature = "MultiplayerPositionHUDController")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MultiplayerPositionHUDController {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MultiplayerPositionHUDController";
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
