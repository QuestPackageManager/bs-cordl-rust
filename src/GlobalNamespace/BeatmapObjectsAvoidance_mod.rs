#[cfg(feature = "BeatmapObjectsAvoidance")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapObjectsAvoidance {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _zOffset: f32,
    pub _yOffset: f32,
    pub _gravity: crate::UnityEngine::Vector2,
    pub _towardsPlayerWrapperTransform: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Transform,
    >,
    pub _audioTimeSource: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IAudioTimeSource,
    >,
    pub _beatmapData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IReadonlyBeatmapData,
    >,
    pub _beatmapObjectSpawnController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapObjectSpawnController,
    >,
    pub _variableMovementDataProvider: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IVariableMovementDataProvider,
    >,
    pub _jumpOffsetYProvider: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IJumpOffsetYProvider,
    >,
    pub _playerTransforms: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerTransforms,
    >,
    pub _avoidanceYOffsetEvaluatorProvider: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapObjectAvoidanceYOffsetEvaluator,
    >,
    pub _pathEvaluator: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapObjectAvoidancePathEvaluator,
    >,
    pub _tiltEvaluator: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapObjectAvoidanceTiltEvaluator,
    >,
    pub _pathBezierSplineEvaluator: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BezierSplineEvaluator,
    >,
    pub _accelerationBezierSplineEvaluator: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BezierSplineEvaluator,
    >,
    pub _transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
}
#[cfg(feature = "BeatmapObjectsAvoidance")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapObjectsAvoidance {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapObjectsAvoidance";
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
#[cfg(feature = "BeatmapObjectsAvoidance")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapObjectsAvoidance {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectsAvoidance")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapObjectsAvoidance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectsAvoidance")]
impl crate::GlobalNamespace::BeatmapObjectsAvoidance {
    pub fn AdjustPositionWithOffsetDirection(
        &mut self,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        lineIndex: i32,
        offsetDirection: crate::GlobalNamespace::OffsetDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AdjustPositionWithOffsetDirection",
                (position, lineIndex, offsetDirection),
            )?;
        Ok(__cordl_ret.into())
    }
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
    pub fn BuildAnimationCurvePath(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("BuildAnimationCurvePath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleBeatmapObjectSpawnControllerDidInit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleBeatmapObjectSpawnControllerDidInit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetupAndRun(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupAndRun", ())?;
        Ok(__cordl_ret.into())
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
}
#[cfg(feature = "BeatmapObjectsAvoidance")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapObjectsAvoidance {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
