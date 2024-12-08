#[cfg(feature = "BeatmapObjectAvoidanceTiltEvaluator")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapObjectAvoidanceTiltEvaluator {
    __cordl_parent: crate::System::Object,
    pub _audioTimeSource: *mut crate::GlobalNamespace::IAudioTimeSource,
    pub _gravity: crate::UnityEngine::Vector2,
    pub _normalizedGravity: crate::UnityEngine::Vector2,
    pub _bezierSplineEvaluator: *mut crate::GlobalNamespace::BezierSplineEvaluator,
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
    type Target = crate::System::Object;
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
        Ok(__cordl_ret)
    }
    pub fn New(
        audioTimeSource: *mut crate::GlobalNamespace::IAudioTimeSource,
        bezierSplineEvaluator: *mut crate::GlobalNamespace::BezierSplineEvaluator,
        gravity: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (audioTimeSource, bezierSplineEvaluator, gravity))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        audioTimeSource: *mut crate::GlobalNamespace::IAudioTimeSource,
        bezierSplineEvaluator: *mut crate::GlobalNamespace::BezierSplineEvaluator,
        gravity: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (audioTimeSource, bezierSplineEvaluator, gravity))?;
        Ok(__cordl_ret)
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
