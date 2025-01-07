#[cfg(feature = "UnityEngine+Timeline+ScheduleRuntimeClip")]
#[repr(C)]
#[derive(Debug)]
pub struct ScheduleRuntimeClip {
    __cordl_parent: crate::UnityEngine::Timeline::RuntimeClipBase,
    pub m_Clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
    pub m_Playable: crate::UnityEngine::Playables::Playable,
    pub m_ParentMixer: crate::UnityEngine::Playables::Playable,
    pub m_StartDelay: f64,
    pub m_FinishTail: f64,
    pub m_Started: bool,
}
#[cfg(feature = "UnityEngine+Timeline+ScheduleRuntimeClip")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Timeline::ScheduleRuntimeClip {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Timeline";
    const CLASS_NAME: &'static str = "ScheduleRuntimeClip";
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
#[cfg(feature = "UnityEngine+Timeline+ScheduleRuntimeClip")]
impl std::ops::Deref for crate::UnityEngine::Timeline::ScheduleRuntimeClip {
    type Target = crate::UnityEngine::Timeline::RuntimeClipBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+ScheduleRuntimeClip")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::ScheduleRuntimeClip {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+ScheduleRuntimeClip")]
impl crate::UnityEngine::Timeline::ScheduleRuntimeClip {
    pub fn Create(
        &mut self,
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
        clipPlayable: crate::UnityEngine::Playables::Playable,
        parentMixer: crate::UnityEngine::Playables::Playable,
        startDelay: f64,
        finishTail: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Create",
                (clip, clipPlayable, parentMixer, startDelay, finishTail),
            )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn New(
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
        clipPlayable: crate::UnityEngine::Playables::Playable,
        parentMixer: crate::UnityEngine::Playables::Playable,
        startDelay: f64,
        finishTail: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (clip, clipPlayable, parentMixer, startDelay, finishTail),
            )?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
        clipPlayable: crate::UnityEngine::Playables::Playable,
        parentMixer: crate::UnityEngine::Playables::Playable,
        startDelay: f64,
        finishTail: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (clip, clipPlayable, parentMixer, startDelay, finishTail))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_clip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::TimelineClip,
        > = __cordl_object.invoke("get_clip", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_duration(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_duration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_mixer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Playables::Playable = __cordl_object
            .invoke("get_mixer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_playable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Playables::Playable = __cordl_object
            .invoke("get_playable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_start(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_start", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+ScheduleRuntimeClip")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::ScheduleRuntimeClip {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
