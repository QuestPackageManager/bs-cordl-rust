#[cfg(feature = "GhostEffectBehaviour")]
#[repr(C)]
#[derive(Debug)]
pub struct GhostEffectBehaviour {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Playables::PlayableBehaviour,
    >,
    pub alphaCurve: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationCurve>,
    pub sizeCurve: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationCurve>,
    pub distanceCurve: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationCurve>,
    pub _distanceMultiplier: crate::UnityEngine::Vector3,
    pub _useStartTransform: bool,
    pub _useEndTransform: bool,
    pub _startLocalPosition: crate::UnityEngine::Vector3,
    pub _startTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _endLocalPosition: crate::UnityEngine::Vector3,
    pub _endTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _positionEasing: crate::GlobalNamespace::EaseType,
    pub _endBehavior: crate::GlobalNamespace::GhostEffectBehaviour_EndBehavior,
    pub progress: f32,
    pub textMeshPros: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshPro>,
        >,
    >,
    pub _canvasGroups: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::CanvasGroup>,
        >,
    >,
    pub _ghostEffectType: crate::GlobalNamespace::GhostEffectBehaviour_GhostEffectType,
    pub _ghostEffectTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _direction: crate::UnityEngine::Vector3,
    pub _finished: bool,
}
#[cfg(feature = "GhostEffectBehaviour")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GhostEffectBehaviour => ""
    ."GhostEffectBehaviour"
);
#[cfg(feature = "GhostEffectBehaviour")]
impl std::ops::Deref for crate::GlobalNamespace::GhostEffectBehaviour {
    type Target = quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Playables::PlayableBehaviour,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GhostEffectBehaviour")]
impl std::ops::DerefMut for crate::GlobalNamespace::GhostEffectBehaviour {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GhostEffectBehaviour")]
impl crate::GlobalNamespace::GhostEffectBehaviour {
    #[cfg(feature = "GhostEffectBehaviour+EndBehavior")]
    pub type EndBehavior = crate::GlobalNamespace::GhostEffectBehaviour_EndBehavior;
    #[cfg(feature = "GhostEffectBehaviour+GhostEffectType")]
    pub type GhostEffectType = crate::GlobalNamespace::GhostEffectBehaviour_GhostEffectType;
    pub fn EnableObjects(
        &mut self,
        on: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnableObjects", (on))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnBehaviourPlay(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
        info: crate::UnityEngine::Playables::FrameData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnBehaviourPlay", (playable, info))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessFrame(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
        info: crate::UnityEngine::Playables::FrameData,
        playerData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessFrame", (playable, info, playerData))?;
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
#[cfg(feature = "GhostEffectBehaviour")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::GhostEffectBehaviour {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GhostEffectBehaviour+EndBehavior")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GhostEffectBehaviour_EndBehavior {
    #[default]
    DisableAll = 0i32,
    DisableCopies = 1i32,
    Nothing = 2i32,
}
#[cfg(feature = "GhostEffectBehaviour+EndBehavior")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GhostEffectBehaviour_EndBehavior => ""
    ."GhostEffectBehaviour/EndBehavior"
);
#[cfg(feature = "GhostEffectBehaviour+GhostEffectType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GhostEffectBehaviour_GhostEffectType {
    #[default]
    Canvas = 1i32,
    TextMeshPro = 0i32,
}
#[cfg(feature = "GhostEffectBehaviour+GhostEffectType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GhostEffectBehaviour_GhostEffectType => ""
    ."GhostEffectBehaviour/GhostEffectType"
);
