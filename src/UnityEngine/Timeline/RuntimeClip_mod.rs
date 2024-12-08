#[cfg(feature = "UnityEngine+Timeline+RuntimeClip")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimeClip {
    __cordl_parent: crate::UnityEngine::Timeline::RuntimeClipBase,
    pub m_Clip: *mut crate::UnityEngine::Timeline::TimelineClip,
    pub m_Playable: crate::UnityEngine::Playables::Playable,
    pub m_ParentMixer: crate::UnityEngine::Playables::Playable,
}
#[cfg(feature = "UnityEngine+Timeline+RuntimeClip")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::RuntimeClip =>
    "UnityEngine.Timeline"."RuntimeClip"
);
#[cfg(feature = "UnityEngine+Timeline+RuntimeClip")]
impl std::ops::Deref for crate::UnityEngine::Timeline::RuntimeClip {
    type Target = crate::UnityEngine::Timeline::RuntimeClipBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+RuntimeClip")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::RuntimeClip {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+RuntimeClip")]
impl crate::UnityEngine::Timeline::RuntimeClip {
    pub fn Create(
        &mut self,
        clip: *mut crate::UnityEngine::Timeline::TimelineClip,
        clipPlayable: crate::UnityEngine::Playables::Playable,
        parentMixer: crate::UnityEngine::Playables::Playable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Create", (clip, clipPlayable, parentMixer))?;
        Ok(__cordl_ret)
    }
    pub fn DisableAt(
        &mut self,
        localTime: f64,
        rootDuration: f64,
        frameData: crate::UnityEngine::Playables::FrameData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisableAt", (localTime, rootDuration, frameData))?;
        Ok(__cordl_ret)
    }
    pub fn EvaluateAt(
        &mut self,
        localTime: f64,
        frameData: crate::UnityEngine::Playables::FrameData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EvaluateAt", (localTime, frameData))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        clip: *mut crate::UnityEngine::Timeline::TimelineClip,
        clipPlayable: crate::UnityEngine::Playables::Playable,
        parentMixer: crate::UnityEngine::Playables::Playable,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (clip, clipPlayable, parentMixer))?;
        Ok(__cordl_object)
    }
    pub fn SetDuration(
        &mut self,
        duration: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDuration", (duration))?;
        Ok(__cordl_ret)
    }
    pub fn SetTime(
        &mut self,
        _cordl_time: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTime", (_cordl_time))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        clip: *mut crate::UnityEngine::Timeline::TimelineClip,
        clipPlayable: crate::UnityEngine::Playables::Playable,
        parentMixer: crate::UnityEngine::Playables::Playable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (clip, clipPlayable, parentMixer))?;
        Ok(__cordl_ret)
    }
    pub fn get_clip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Timeline::TimelineClip> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Timeline::TimelineClip = __cordl_object
            .invoke("get_clip", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_duration(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_duration", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_mixer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Playables::Playable = __cordl_object
            .invoke("get_mixer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_playable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Playables::Playable = __cordl_object
            .invoke("get_playable", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_start(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_start", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_enable(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_enable", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+Timeline+RuntimeClip")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Timeline::RuntimeClip {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
