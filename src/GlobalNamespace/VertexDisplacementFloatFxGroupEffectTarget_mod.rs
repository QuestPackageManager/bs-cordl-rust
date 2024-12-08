#[cfg(feature = "VertexDisplacementFloatFxGroupEffectTarget")]
#[repr(C)]
#[derive(Debug)]
pub struct VertexDisplacementFloatFxGroupEffectTarget {
    __cordl_parent: crate::GlobalNamespace::FloatFxGroupEffectTarget,
    pub _displacementRanges: crate::UnityEngine::Vector3,
    pub _xAnimationCurve: *mut crate::UnityEngine::AnimationCurve,
    pub _yAnimationCurve: *mut crate::UnityEngine::AnimationCurve,
    pub _zAnimationCurve: *mut crate::UnityEngine::AnimationCurve,
    pub _displacementController: *mut crate::GlobalNamespace::MaterialPropertyBlockController,
    pub _renderer: *mut crate::UnityEngine::Renderer,
    pub _useTestValue: bool,
    pub _testFloatValue: f32,
    pub _bounds: crate::UnityEngine::Bounds,
}
#[cfg(feature = "VertexDisplacementFloatFxGroupEffectTarget")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::VertexDisplacementFloatFxGroupEffectTarget => ""
    ."VertexDisplacementFloatFxGroupEffectTarget"
);
#[cfg(feature = "VertexDisplacementFloatFxGroupEffectTarget")]
impl std::ops::Deref
for crate::GlobalNamespace::VertexDisplacementFloatFxGroupEffectTarget {
    type Target = crate::GlobalNamespace::FloatFxGroupEffectTarget;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VertexDisplacementFloatFxGroupEffectTarget")]
impl std::ops::DerefMut
for crate::GlobalNamespace::VertexDisplacementFloatFxGroupEffectTarget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VertexDisplacementFloatFxGroupEffectTarget")]
impl crate::GlobalNamespace::VertexDisplacementFloatFxGroupEffectTarget {
    pub fn CalculateDisplacementVector(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector4 = __cordl_object
            .invoke("CalculateDisplacementVector", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnValidate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnValidate", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetValue_f32_1(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValue", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SetValue_i32_i32_f32_0(
        &mut self,
        groupId: i32,
        elementId: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValue", (groupId, elementId, value))?;
        Ok(__cordl_ret)
    }
    pub fn TriggerValue(
        &mut self,
        groupId: i32,
        elementId: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TriggerValue", (groupId, elementId, value))?;
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
#[cfg(feature = "VertexDisplacementFloatFxGroupEffectTarget")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::VertexDisplacementFloatFxGroupEffectTarget {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
