#[cfg(feature = "ObstacleSaberSparkleEffectManager")]
#[repr(C)]
#[derive(Debug)]
pub struct ObstacleSaberSparkleEffectManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _obstacleSaberSparkleEffectPrefab: *mut crate::GlobalNamespace::ObstacleSaberSparkleEffect,
    pub _rumblePreset: *mut crate::Libraries::HM::HMLib::VR::HapticPresetSO,
    pub _beatmapObjectManager: *mut crate::GlobalNamespace::BeatmapObjectManager,
    pub _saberManager: *mut crate::GlobalNamespace::SaberManager,
    pub _hapticFeedbackManager: *mut crate::GlobalNamespace::HapticFeedbackManager,
    pub _colorManager: *mut crate::GlobalNamespace::ColorManager,
    pub _sabers: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::Saber,
    >,
    pub _effects: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::ObstacleSaberSparkleEffect,
    >,
    pub sparkleEffectDidStartEvent: *mut crate::System::Action_1<
        crate::GlobalNamespace::SaberType,
    >,
    pub sparkleEffectDidEndEvent: *mut crate::System::Action_1<
        crate::GlobalNamespace::SaberType,
    >,
}
#[cfg(feature = "ObstacleSaberSparkleEffectManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ObstacleSaberSparkleEffectManager => ""
    ."ObstacleSaberSparkleEffectManager"
);
#[cfg(feature = "ObstacleSaberSparkleEffectManager")]
impl std::ops::Deref for crate::GlobalNamespace::ObstacleSaberSparkleEffectManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ObstacleSaberSparkleEffectManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::ObstacleSaberSparkleEffectManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ObstacleSaberSparkleEffectManager")]
impl crate::GlobalNamespace::ObstacleSaberSparkleEffectManager {
    #[cfg(feature = "ObstacleSaberSparkleEffectManager+BoxSideRotations")]
    pub type BoxSideRotations = crate::GlobalNamespace::ObstacleSaberSparkleEffectManager_BoxSideRotations;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn add_sparkleEffectDidEndEvent(
        &mut self,
        value: *mut crate::System::Action_1<crate::GlobalNamespace::SaberType>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_sparkleEffectDidEndEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_sparkleEffectDidStartEvent(
        &mut self,
        value: *mut crate::System::Action_1<crate::GlobalNamespace::SaberType>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_sparkleEffectDidStartEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_sparkleEffectDidEndEvent(
        &mut self,
        value: *mut crate::System::Action_1<crate::GlobalNamespace::SaberType>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_sparkleEffectDidEndEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_sparkleEffectDidStartEvent(
        &mut self,
        value: *mut crate::System::Action_1<crate::GlobalNamespace::SaberType>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_sparkleEffectDidStartEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "ObstacleSaberSparkleEffectManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ObstacleSaberSparkleEffectManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ObstacleSaberSparkleEffectManager+BoxSideRotations")]
#[repr(C)]
#[derive(Debug)]
pub struct ObstacleSaberSparkleEffectManager_BoxSideRotations {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "ObstacleSaberSparkleEffectManager+BoxSideRotations")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ObstacleSaberSparkleEffectManager_BoxSideRotations => ""
    ."ObstacleSaberSparkleEffectManager/BoxSideRotations"
);
#[cfg(feature = "ObstacleSaberSparkleEffectManager+BoxSideRotations")]
impl std::ops::Deref
for crate::GlobalNamespace::ObstacleSaberSparkleEffectManager_BoxSideRotations {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ObstacleSaberSparkleEffectManager+BoxSideRotations")]
impl std::ops::DerefMut
for crate::GlobalNamespace::ObstacleSaberSparkleEffectManager_BoxSideRotations {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ObstacleSaberSparkleEffectManager+BoxSideRotations")]
impl crate::GlobalNamespace::ObstacleSaberSparkleEffectManager_BoxSideRotations {}
#[cfg(feature = "ObstacleSaberSparkleEffectManager+BoxSideRotations")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ObstacleSaberSparkleEffectManager_BoxSideRotations {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
