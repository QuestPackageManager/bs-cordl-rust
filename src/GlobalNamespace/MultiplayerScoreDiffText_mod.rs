#[cfg(feature = "MultiplayerScoreDiffText")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerScoreDiffText {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _activeTextColor: crate::UnityEngine::Color,
    pub _normalBackgroundColor: crate::UnityEngine::Color,
    pub _leadingBackgroundColor: crate::UnityEngine::Color,
    pub _useAutomaticLeadPlayerSelection: bool,
    pub _onPlatformText: *mut crate::TMPro::TextMeshPro,
    pub _backgroundSpriteRenderer: *mut crate::UnityEngine::SpriteRenderer,
    pub _tweeningManager: *mut crate::Tweening::TimeTweeningManager,
    pub _connectedPlayer: *mut crate::GlobalNamespace::IConnectedPlayer,
    pub _leadPlayerProvider: *mut crate::GlobalNamespace::MultiplayerLeadPlayerProvider,
    pub _currentBackgroundColor: crate::UnityEngine::Color,
    pub _state: crate::GlobalNamespace::MultiplayerScoreDiffText_State,
    pub _onPlatformTextAlphaTween: *mut crate::Tweening::FloatTween,
}
#[cfg(feature = "MultiplayerScoreDiffText")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerScoreDiffText => ""
    ."MultiplayerScoreDiffText"
);
#[cfg(feature = "MultiplayerScoreDiffText")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerScoreDiffText {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerScoreDiffText")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerScoreDiffText {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerScoreDiffText")]
impl crate::GlobalNamespace::MultiplayerScoreDiffText {
    #[cfg(feature = "MultiplayerScoreDiffText+HorizontalPosition")]
    pub type HorizontalPosition = crate::GlobalNamespace::MultiplayerScoreDiffText_HorizontalPosition;
    #[cfg(feature = "MultiplayerScoreDiffText+State")]
    pub type State = crate::GlobalNamespace::MultiplayerScoreDiffText_State;
    pub fn AnimateHide(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AnimateHide", ())?;
        Ok(__cordl_ret)
    }
    pub fn AnimateIsLeadPlayer(
        &mut self,
        isLeader: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AnimateIsLeadPlayer", (isLeader))?;
        Ok(__cordl_ret)
    }
    pub fn AnimateScoreDiff(
        &mut self,
        scoreDiff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AnimateScoreDiff", (scoreDiff))?;
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
    pub fn SetHorizontalPositionRelativeToLocalPlayer(
        &mut self,
        relativePosition: crate::GlobalNamespace::MultiplayerScoreDiffText_HorizontalPosition,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetHorizontalPositionRelativeToLocalPlayer", (relativePosition))?;
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
    pub fn _AnimateHide_b__18_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<AnimateHide>b__18_0", ())?;
        Ok(__cordl_ret)
    }
    pub fn _AnimateScoreDiff_b__17_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<AnimateScoreDiff>b__17_0", ())?;
        Ok(__cordl_ret)
    }
    pub fn _Start_b__14_0(
        &mut self,
        val: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<Start>b__14_0", (val))?;
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
#[cfg(feature = "MultiplayerScoreDiffText")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerScoreDiffText {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerScoreDiffText+HorizontalPosition")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiplayerScoreDiffText_HorizontalPosition {
    Left = 0i32,
    Right = 1i32,
}
#[cfg(feature = "MultiplayerScoreDiffText+HorizontalPosition")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerScoreDiffText_HorizontalPosition => ""
    ."MultiplayerScoreDiffText/HorizontalPosition"
);
#[cfg(feature = "MultiplayerScoreDiffText+State")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiplayerScoreDiffText_State {
    AnimatingDisplay = 2i32,
    AnimatingHide = 3i32,
    Displayed = 1i32,
    Hidden = 0i32,
}
#[cfg(feature = "MultiplayerScoreDiffText+State")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerScoreDiffText_State
    => ""."MultiplayerScoreDiffText/State"
);
