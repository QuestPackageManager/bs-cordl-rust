#[cfg(feature = "ObstacleSaberSparkleEffectManager")]
#[repr(C)]
#[derive(Debug)]
pub struct ObstacleSaberSparkleEffectManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _obstacleSaberSparkleEffectPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ObstacleSaberSparkleEffect,
    >,
    pub _rumblePreset: quest_hook::libil2cpp::Gc<
        crate::Libraries::HM::HMLib::VR::HapticPresetSO,
    >,
    pub _beatmapObjectManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapObjectManager,
    >,
    pub _saberManager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SaberManager>,
    pub _hapticFeedbackManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::HapticFeedbackManager,
    >,
    pub _colorManager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorManager>,
    pub _sabers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::GlobalNamespace::Saber>,
    >,
    pub _effects: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::ObstacleSaberSparkleEffect,
        >,
    >,
    pub sparkleEffectDidStartEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<crate::GlobalNamespace::SaberType>,
    >,
    pub sparkleEffectDidEndEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<crate::GlobalNamespace::SaberType>,
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
    pub fn FindBoxSurfaceRotation(
        bounds: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>,
        position: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindBoxSurfaceRotation", (bounds, position))?;
        Ok(__cordl_ret.into())
    }
    pub fn IntersectBounds(
        bounds: crate::UnityEngine::Bounds,
        start: crate::UnityEngine::Vector3,
        end: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IntersectBounds", (bounds, start, end))?;
        Ok(__cordl_ret.into())
    }
    pub fn IntersectBoxSurfacePose(
        bounds: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>,
        start: crate::UnityEngine::Vector3,
        end: crate::UnityEngine::Vector3,
        hit: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Pose>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IntersectBoxSurfacePose", (bounds, start, end, hit))?;
        Ok(__cordl_ret.into())
    }
    pub fn IntersectBoxSurfacePosition(
        bounds: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>,
        start: crate::UnityEngine::Vector3,
        end: crate::UnityEngine::Vector3,
        hit: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IntersectBoxSurfacePosition", (bounds, start, end, hit))?;
        Ok(__cordl_ret.into())
    }
    pub fn IntersectSaberWithObstacles(
        saber: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Saber>,
        obstacles: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ObstacleController>,
            >,
        >,
        intersectObstacleSurface: quest_hook::libil2cpp::ByRefMut<bool>,
        hit: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Pose>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "IntersectSaberWithObstacles",
                (saber, obstacles, intersectObstacleSurface, hit),
            )?;
        Ok(__cordl_ret.into())
    }
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
    pub fn add_sparkleEffectDidEndEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::GlobalNamespace::SaberType>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_sparkleEffectDidEndEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_sparkleEffectDidStartEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::GlobalNamespace::SaberType>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_sparkleEffectDidStartEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_sparkleEffectDidEndEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::GlobalNamespace::SaberType>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_sparkleEffectDidEndEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_sparkleEffectDidStartEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::GlobalNamespace::SaberType>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_sparkleEffectDidStartEvent", (value))?;
        Ok(__cordl_ret.into())
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
