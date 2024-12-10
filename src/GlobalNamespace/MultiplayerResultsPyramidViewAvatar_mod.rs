#[cfg(feature = "MultiplayerResultsPyramidViewAvatar")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerResultsPyramidViewAvatar {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _badgeDirector: *mut crate::UnityEngine::Playables::PlayableDirector,
    pub _ghostFirstTrackName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _ghostSecondTrackName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _standWithAvatarTransform: *mut crate::UnityEngine::Transform,
    pub _perPositionRotation: f32,
    pub _localPlayerColor: crate::UnityEngine::Color,
    pub _positionText: *mut crate::TMPro::TextMeshProUGUI,
    pub _nameText: *mut crate::TMPro::TextMeshProUGUI,
    pub _nameBackground: *mut crate::HMUI::ImageView,
    pub _badgeCanvas: *mut crate::UnityEngine::CanvasGroup,
    pub _badgeTitles: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::TMPro::TextMeshProUGUI,
    >,
    pub _badgeImages: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::HMUI::ImageView,
    >,
    pub _badgeSubtitleText: *mut crate::TMPro::TextMeshProUGUI,
    pub _badgeSubtitleCanvas: *mut crate::UnityEngine::CanvasGroup,
    pub _ghostDuplicationEffect: *mut crate::GlobalNamespace::GhostDuplicationEffect,
    pub _ghostAppear: crate::GlobalNamespace::GhostDuplicationEffect_GhostEffectParams,
    pub _ghostReceive: crate::GlobalNamespace::GhostDuplicationEffect_GhostEffectParams,
    pub _trophyImage: *mut crate::HMUI::ImageView,
    pub _firstPlaceTrophy: *mut crate::UnityEngine::Sprite,
    pub _secondPlaceTrophy: *mut crate::UnityEngine::Sprite,
    pub _thirdPlaceTrophy: *mut crate::UnityEngine::Sprite,
    pub _firstPlaceColor: crate::UnityEngine::Color,
    pub _personalBestVisual: *mut crate::UnityEngine::GameObject,
    pub _playerDataModel: *mut crate::GlobalNamespace::PlayerDataModel,
    pub _beatmapKey: crate::GlobalNamespace::BeatmapKey,
    pub _riseTween: *mut crate::Tweening::Vector3Tween,
    pub _avatarRiseTween: *mut crate::Tweening::Vector3Tween,
    pub _badgePositionTween: *mut crate::Tweening::Vector3Tween,
    pub _badgeOpacityTween: *mut crate::Tweening::FloatTween,
    pub _nameOpacityTween: *mut crate::Tweening::FloatTween,
    pub _namePositionTween: *mut crate::Tweening::Vector3Tween,
    pub _localGlowTween: *mut crate::Tweening::ColorTween,
    pub _titleMakingSpaceForBadgeTween: *mut crate::Tweening::Vector3Tween,
    pub _originalBadgeLocalPos: crate::UnityEngine::Vector3,
    pub _connectedPlayer: *mut crate::GlobalNamespace::IConnectedPlayer,
}
#[cfg(feature = "MultiplayerResultsPyramidViewAvatar")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerResultsPyramidViewAvatar => ""
    ."MultiplayerResultsPyramidViewAvatar"
);
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
        *mut crate::GlobalNamespace::IConnectedPlayer,
        *mut crate::GlobalNamespace::MultiplayerResultsPyramidViewAvatar,
    >,
}
#[cfg(feature = "MultiplayerResultsPyramidViewAvatar+Factory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerResultsPyramidViewAvatar_Factory => ""
    ."MultiplayerResultsPyramidViewAvatar/Factory"
);
#[cfg(feature = "MultiplayerResultsPyramidViewAvatar+Factory")]
impl std::ops::Deref
for crate::GlobalNamespace::MultiplayerResultsPyramidViewAvatar_Factory {
    type Target = crate::Zenject::PlaceholderFactory_2<
        *mut crate::GlobalNamespace::IConnectedPlayer,
        *mut crate::GlobalNamespace::MultiplayerResultsPyramidViewAvatar,
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
