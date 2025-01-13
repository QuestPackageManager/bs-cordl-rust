#[cfg(feature = "MultiplayerResultsPyramidViewAvatar")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerResultsPyramidViewAvatar {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _badgeDirector: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Playables::PlayableDirector,
    >,
    pub _ghostFirstTrackName: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _ghostSecondTrackName: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _standWithAvatarTransform: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Transform,
    >,
    pub _perPositionRotation: f32,
    pub _localPlayerColor: crate::UnityEngine::Color,
    pub _positionText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _nameText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _nameBackground: quest_hook::libil2cpp::Gc<crate::HMUI::ImageView>,
    pub _badgeCanvas: quest_hook::libil2cpp::Gc<crate::UnityEngine::CanvasGroup>,
    pub _badgeTitles: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
        >,
    >,
    pub _badgeImages: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::HMUI::ImageView>,
        >,
    >,
    pub _badgeSubtitleText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _badgeSubtitleCanvas: quest_hook::libil2cpp::Gc<crate::UnityEngine::CanvasGroup>,
    pub _ghostDuplicationEffect: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GhostDuplicationEffect,
    >,
    pub _ghostAppear: crate::GlobalNamespace::GhostDuplicationEffect_GhostEffectParams,
    pub _ghostReceive: crate::GlobalNamespace::GhostDuplicationEffect_GhostEffectParams,
    pub _trophyImage: quest_hook::libil2cpp::Gc<crate::HMUI::ImageView>,
    pub _firstPlaceTrophy: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub _secondPlaceTrophy: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub _thirdPlaceTrophy: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub _firstPlaceColor: crate::UnityEngine::Color,
    pub _personalBestVisual: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _playerDataModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerDataModel,
    >,
    pub _beatmapKey: crate::GlobalNamespace::BeatmapKey,
    pub _riseTween: quest_hook::libil2cpp::Gc<crate::Tweening::Vector3Tween>,
    pub _avatarRiseTween: quest_hook::libil2cpp::Gc<crate::Tweening::Vector3Tween>,
    pub _badgePositionTween: quest_hook::libil2cpp::Gc<crate::Tweening::Vector3Tween>,
    pub _badgeOpacityTween: quest_hook::libil2cpp::Gc<crate::Tweening::FloatTween>,
    pub _nameOpacityTween: quest_hook::libil2cpp::Gc<crate::Tweening::FloatTween>,
    pub _namePositionTween: quest_hook::libil2cpp::Gc<crate::Tweening::Vector3Tween>,
    pub _localGlowTween: quest_hook::libil2cpp::Gc<crate::Tweening::ColorTween>,
    pub _titleMakingSpaceForBadgeTween: quest_hook::libil2cpp::Gc<
        crate::Tweening::Vector3Tween,
    >,
    pub _originalBadgeLocalPos: crate::UnityEngine::Vector3,
    pub _connectedPlayer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IConnectedPlayer,
    >,
}
#[cfg(feature = "MultiplayerResultsPyramidViewAvatar")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MultiplayerResultsPyramidViewAvatar {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MultiplayerResultsPyramidViewAvatar";
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
#[cfg(feature = "MultiplayerResultsPyramidViewAvatar")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerResultsPyramidViewAvatar {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerResultsPyramidViewAvatar")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerResultsPyramidViewAvatar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerResultsPyramidViewAvatar")]
impl crate::GlobalNamespace::MultiplayerResultsPyramidViewAvatar {
    #[cfg(feature = "MultiplayerResultsPyramidViewAvatar+Factory")]
    pub type Factory = crate::GlobalNamespace::MultiplayerResultsPyramidViewAvatar_Factory;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Setup(
        &mut self,
        resultData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerPlayerResultsData,
        >,
        position: i32,
        playerCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Setup", (resultData, position, playerCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetupBadgeTimeline(
        &mut self,
        startTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        midTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupBadgeTimeline", (startTransform, midTransform))?;
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
    pub fn get_badgeDirector(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Playables::PlayableDirector>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Playables::PlayableDirector,
        > = __cordl_object.invoke("get_badgeDirector", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerResultsPyramidViewAvatar")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerResultsPyramidViewAvatar {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerResultsPyramidViewAvatar+Factory")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerResultsPyramidViewAvatar_Factory {
    __cordl_parent: crate::Zenject::PlaceholderFactory_2<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerResultsPyramidViewAvatar,
        >,
    >,
}
#[cfg(feature = "MultiplayerResultsPyramidViewAvatar+Factory")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MultiplayerResultsPyramidViewAvatar_Factory {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MultiplayerResultsPyramidViewAvatar/Factory";
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
#[cfg(feature = "MultiplayerResultsPyramidViewAvatar+Factory")]
impl std::ops::Deref
for crate::GlobalNamespace::MultiplayerResultsPyramidViewAvatar_Factory {
    type Target = crate::Zenject::PlaceholderFactory_2<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerResultsPyramidViewAvatar,
        >,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerResultsPyramidViewAvatar+Factory")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerResultsPyramidViewAvatar_Factory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerResultsPyramidViewAvatar+Factory")]
impl crate::GlobalNamespace::MultiplayerResultsPyramidViewAvatar_Factory {
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerResultsPyramidViewAvatar+Factory")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerResultsPyramidViewAvatar_Factory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
