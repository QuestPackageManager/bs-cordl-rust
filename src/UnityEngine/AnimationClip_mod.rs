#[cfg(feature = "UnityEngine+AnimationClip")]
#[repr(C)]
#[derive(Debug)]
pub struct AnimationClip {
    __cordl_parent: crate::UnityEngine::Motion,
}
#[cfg(feature = "UnityEngine+AnimationClip")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AnimationClip => "UnityEngine"
    ."AnimationClip"
);
#[cfg(feature = "UnityEngine+AnimationClip")]
impl std::ops::Deref for crate::UnityEngine::AnimationClip {
    type Target = crate::UnityEngine::Motion;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AnimationClip")]
impl std::ops::DerefMut for crate::UnityEngine::AnimationClip {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AnimationClip")]
impl crate::UnityEngine::AnimationClip {
    pub fn AddEvent(
        &mut self,
        evt: *mut crate::UnityEngine::AnimationEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddEvent", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn AddEventInternal(
        &mut self,
        evt: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddEventInternal", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn ClearCurves(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearCurves", ())?;
        Ok(__cordl_ret)
    }
    pub fn EnsureQuaternionContinuity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnsureQuaternionContinuity", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetEventsInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Array> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Array = __cordl_object
            .invoke("GetEventsInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SampleAnimation(
        &mut self,
        go: *mut crate::UnityEngine::GameObject,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SampleAnimation", (go, _cordl_time))?;
        Ok(__cordl_ret)
    }
    pub fn SetCurve(
        &mut self,
        relativePath: *mut crate::System::String,
        _cordl_type: *mut crate::System::Type,
        propertyName: *mut crate::System::String,
        curve: *mut crate::UnityEngine::AnimationCurve,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCurve", (relativePath, _cordl_type, propertyName, curve))?;
        Ok(__cordl_ret)
    }
    pub fn SetEventsInternal(
        &mut self,
        value: *mut crate::System::Array,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetEventsInternal", (value))?;
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
    pub fn get_empty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_empty", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_events(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::AnimationEvent>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::AnimationEvent,
        > = __cordl_object.invoke("get_events", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_frameRate(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_frameRate", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_hasGenericRootTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_hasGenericRootTransform", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_hasMotionCurves(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasMotionCurves", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_hasMotionFloatCurves(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasMotionFloatCurves", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_hasRootCurves(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasRootCurves", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_hasRootMotion(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasRootMotion", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_humanMotion(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_humanMotion", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_legacy(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_legacy", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_length(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_length", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_localBounds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Bounds> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Bounds = __cordl_object
            .invoke("get_localBounds", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_localBounds_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_localBounds_Injected", (ret))?;
        Ok(__cordl_ret)
    }
    pub fn get_startTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_startTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_stopTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_stopTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_wrapMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::WrapMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::WrapMode = __cordl_object
            .invoke("get_wrapMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_events(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::AnimationEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_events", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_frameRate(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_frameRate", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_legacy(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_legacy", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_localBounds(
        &mut self,
        value: crate::UnityEngine::Bounds,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_localBounds", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_localBounds_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_localBounds_Injected", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_wrapMode(
        &mut self,
        value: crate::UnityEngine::WrapMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_wrapMode", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+AnimationClip")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::AnimationClip {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}