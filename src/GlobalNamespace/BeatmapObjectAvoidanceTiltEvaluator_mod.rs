#[cfg(feature = "BeatmapObjectAvoidanceTiltEvaluator")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapObjectAvoidanceTiltEvaluator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _audioTimeSource: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IAudioTimeSource,
    >,
    pub _gravity: crate::UnityEngine::Vector2,
    pub _normalizedGravity: crate::UnityEngine::Vector2,
    pub _bezierSplineEvaluator: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BezierSplineEvaluator,
    >,
    pub _currentAcceleration: f32,
}
#[cfg(feature = "BeatmapObjectAvoidanceTiltEvaluator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapObjectAvoidanceTiltEvaluator => ""
    ."BeatmapObjectAvoidanceTiltEvaluator"
);
#[cfg(feature = "BeatmapObjectAvoidanceTiltEvaluator")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapObjectAvoidanceTiltEvaluator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectAvoidanceTiltEvaluator")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapObjectAvoidanceTiltEvaluator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectAvoidanceTiltEvaluator")]
impl crate::GlobalNamespace::BeatmapObjectAvoidanceTiltEvaluator {
    pub const kLookAheadTime: f32 = 0.2f32;
    pub fn GetTiltAngle(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetTiltAngle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        audioTimeSource: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IAudioTimeSource,
        >,
        bezierSplineEvaluator: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BezierSplineEvaluator,
        >,
        gravity: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (audioTimeSource, bezierSplineEvaluator, gravity))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        audioTimeSource: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IAudioTimeSource,
        >,
        bezierSplineEvaluator: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BezierSplineEvaluator,
        >,
        gravity: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (audioTimeSource, bezierSplineEvaluator, gravity))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapObjectAvoidanceTiltEvaluator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapObjectAvoidanceTiltEvaluator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
