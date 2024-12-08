#[cfg(feature = "FlyingObjectEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct FlyingObjectEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _moveAnimationCurve: *mut crate::UnityEngine::AnimationCurve,
    pub _shakeFrequency: f32,
    pub _shakeStrength: f32,
    pub _shakeStrengthAnimationCurve: *mut crate::UnityEngine::AnimationCurve,
    pub _initialized: bool,
    pub _shakeRotation: crate::UnityEngine::Quaternion,
    pub _rotation: crate::UnityEngine::Quaternion,
    pub _elapsedTime: f32,
    pub _startPos: crate::UnityEngine::Vector3,
    pub _targetPos: crate::UnityEngine::Vector3,
    pub _duration: f32,
    pub _shake: bool,
    pub _didFinishEvent: *mut LazyCopyHashSet_1<*mut IFlyingObjectEffectDidFinishEvent>,
}
#[cfg(feature = "FlyingObjectEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for FlyingObjectEffect => ""."FlyingObjectEffect"
);
#[cfg(feature = "FlyingObjectEffect")]
impl std::ops::Deref for FlyingObjectEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FlyingObjectEffect")]
impl std::ops::DerefMut for FlyingObjectEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FlyingObjectEffect")]
impl FlyingObjectEffect {
    pub fn InitAndPresent(
        &mut self,
        duration: f32,
        targetPos: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        shake: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitAndPresent", (duration, targetPos, rotation, shake))?;
        Ok(__cordl_ret)
    }
    pub fn ManualUpdate(
        &mut self,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ManualUpdate", (t))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn get_didFinishEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut ILazyCopyHashSet_1<*mut IFlyingObjectEffectDidFinishEvent>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut ILazyCopyHashSet_1<
            *mut IFlyingObjectEffectDidFinishEvent,
        > = __cordl_object.invoke("get_didFinishEvent", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "FlyingObjectEffect")]
impl quest_hook::libil2cpp::ObjectType for FlyingObjectEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}