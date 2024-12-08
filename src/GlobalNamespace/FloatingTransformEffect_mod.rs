#[cfg(feature = "FloatingTransformEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct FloatingTransformEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _positionMultiplier: crate::UnityEngine::Vector3,
    pub _rotationMultiplier: crate::UnityEngine::Vector3,
    pub _rotationTransform: *mut crate::UnityEngine::Transform,
    pub _maxRotationDegrees: f32,
    pub _xAmplitude: crate::UnityEngine::Vector2,
    pub _xFrequency: crate::UnityEngine::Vector2,
    pub _yAmplitude: crate::UnityEngine::Vector2,
    pub _yFrequency: crate::UnityEngine::Vector2,
    pub _zAmplitude: crate::UnityEngine::Vector2,
    pub _zFrequency: crate::UnityEngine::Vector2,
    pub _transform: *mut crate::UnityEngine::Transform,
    pub _origin: crate::UnityEngine::Vector3,
    pub _offsetX: f32,
    pub _offsetY: f32,
    pub _offsetZ: f32,
    pub _amplitudeX: f32,
    pub _amplitudeY: f32,
    pub _amplitudeZ: f32,
    pub _frequencyX: f32,
    pub _frequencyY: f32,
    pub _frequencyZ: f32,
    pub _targetRotation: crate::UnityEngine::Quaternion,
}
#[cfg(feature = "FloatingTransformEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for FloatingTransformEffect => ""."FloatingTransformEffect"
);
#[cfg(feature = "FloatingTransformEffect")]
impl std::ops::Deref for FloatingTransformEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FloatingTransformEffect")]
impl std::ops::DerefMut for FloatingTransformEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FloatingTransformEffect")]
impl FloatingTransformEffect {
    pub fn GetPoint(
        &mut self,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("GetPoint", (_cordl_time))?;
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
    pub fn Refresh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Refresh", ())?;
        Ok(__cordl_ret)
    }
    pub fn Tick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Tick", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "FloatingTransformEffect")]
impl quest_hook::libil2cpp::ObjectType for FloatingTransformEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
