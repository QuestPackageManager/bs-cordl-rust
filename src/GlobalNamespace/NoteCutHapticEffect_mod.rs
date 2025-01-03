#[cfg(feature = "NoteCutHapticEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteCutHapticEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _normalPreset: quest_hook::libil2cpp::Gc<
        crate::Libraries::HM::HMLib::VR::HapticPresetSO,
    >,
    pub _chainHeadPreset: quest_hook::libil2cpp::Gc<
        crate::Libraries::HM::HMLib::VR::HapticPresetSO,
    >,
    pub _chainLinkPreset: quest_hook::libil2cpp::Gc<
        crate::Libraries::HM::HMLib::VR::HapticPresetSO,
    >,
    pub _bombPreset: quest_hook::libil2cpp::Gc<
        crate::Libraries::HM::HMLib::VR::HapticPresetSO,
    >,
    pub _badCutPreset: quest_hook::libil2cpp::Gc<
        crate::Libraries::HM::HMLib::VR::HapticPresetSO,
    >,
    pub _arcHeadPreset: quest_hook::libil2cpp::Gc<
        crate::Libraries::HM::HMLib::VR::HapticPresetSO,
    >,
    pub _arcTailPreset: quest_hook::libil2cpp::Gc<
        crate::Libraries::HM::HMLib::VR::HapticPresetSO,
    >,
    pub _arcHeadAndTailPreset: quest_hook::libil2cpp::Gc<
        crate::Libraries::HM::HMLib::VR::HapticPresetSO,
    >,
    pub _hapticFeedbackManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::HapticFeedbackManager,
    >,
}
#[cfg(feature = "NoteCutHapticEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NoteCutHapticEffect => ""
    ."NoteCutHapticEffect"
);
#[cfg(feature = "NoteCutHapticEffect")]
impl std::ops::Deref for crate::GlobalNamespace::NoteCutHapticEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutHapticEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteCutHapticEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutHapticEffect")]
impl crate::GlobalNamespace::NoteCutHapticEffect {
    #[cfg(feature = "NoteCutHapticEffect+Type")]
    pub type Type = crate::GlobalNamespace::NoteCutHapticEffect_Type;
    pub fn HitNote(
        &mut self,
        saberType: crate::GlobalNamespace::SaberType,
        _cordl_type: crate::GlobalNamespace::NoteCutHapticEffect_Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HitNote", (saberType, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
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
#[cfg(feature = "NoteCutHapticEffect")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::NoteCutHapticEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "NoteCutHapticEffect+Type")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoteCutHapticEffect_Type {
    ArcHead = 5i32,
    ArcHeadAndTail = 7i32,
    ArcTail = 6i32,
    BadCut = 4i32,
    Bomb = 3i32,
    ChainHead = 1i32,
    ChainLink = 2i32,
    Normal = 0i32,
}
#[cfg(feature = "NoteCutHapticEffect+Type")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NoteCutHapticEffect_Type => ""
    ."NoteCutHapticEffect/Type"
);
