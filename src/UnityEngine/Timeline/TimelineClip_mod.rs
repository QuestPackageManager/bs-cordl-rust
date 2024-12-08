#[cfg(feature = "UnityEngine+Timeline+TimelineClip+BlendCurveMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimelineClip_BlendCurveMode {
    Auto = 0i32,
    Manual = 1i32,
}
#[cfg(feature = "UnityEngine+Timeline+TimelineClip+BlendCurveMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Timeline::TimelineClip_BlendCurveMode => "UnityEngine.Timeline"
    ."TimelineClip/BlendCurveMode"
);
#[cfg(feature = "UnityEngine+Timeline+TimelineClip+ClipExtrapolation")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimelineClip_ClipExtrapolation {
    Continue = 4i32,
    Hold = 1i32,
    Loop = 2i32,
    None = 0i32,
    PingPong = 3i32,
}
#[cfg(feature = "UnityEngine+Timeline+TimelineClip+ClipExtrapolation")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Timeline::TimelineClip_ClipExtrapolation => "UnityEngine.Timeline"
    ."TimelineClip/ClipExtrapolation"
);
#[cfg(feature = "UnityEngine+Timeline+TimelineClip")]
#[repr(C)]
#[derive(Debug)]
pub struct TimelineClip {
    __cordl_parent: crate::System::Object,
    pub m_Version: i32,
    pub m_Start: f64,
    pub m_ClipIn: f64,
    pub m_Asset: *mut crate::UnityEngine::Object,
    pub m_Duration: f64,
    pub m_TimeScale: f64,
    pub m_ParentTrack: *mut crate::UnityEngine::Timeline::TrackAsset,
    pub m_EaseInDuration: f64,
    pub m_EaseOutDuration: f64,
    pub m_BlendInDuration: f64,
    pub m_BlendOutDuration: f64,
    pub m_MixInCurve: *mut crate::UnityEngine::AnimationCurve,
    pub m_MixOutCurve: *mut crate::UnityEngine::AnimationCurve,
    pub m_BlendInCurveMode: crate::UnityEngine::Timeline::TimelineClip_BlendCurveMode,
    pub m_BlendOutCurveMode: crate::UnityEngine::Timeline::TimelineClip_BlendCurveMode,
    pub m_ExposedParameterNames: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::String,
    >,
    pub m_AnimationCurves: *mut crate::UnityEngine::AnimationClip,
    pub m_Recordable: bool,
    pub m_PostExtrapolationMode: crate::UnityEngine::Timeline::TimelineClip_ClipExtrapolation,
    pub m_PreExtrapolationMode: crate::UnityEngine::Timeline::TimelineClip_ClipExtrapolation,
    pub m_PostExtrapolationTime: f64,
    pub m_PreExtrapolationTime: f64,
    pub m_DisplayName: *mut crate::System::String,
}
#[cfg(feature = "UnityEngine+Timeline+TimelineClip")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::TimelineClip =>
    "UnityEngine.Timeline"."TimelineClip"
);
#[cfg(feature = "UnityEngine+Timeline+TimelineClip")]
impl std::ops::Deref for crate::UnityEngine::Timeline::TimelineClip {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineClip")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::TimelineClip {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineClip")]
impl crate::UnityEngine::Timeline::TimelineClip {
    pub const k_LatestVersion: i32 = 1i32;
    #[cfg(feature = "UnityEngine+Timeline+TimelineClip+Versions")]
    pub type Versions = crate::UnityEngine::Timeline::TimelineClip_Versions;
    #[cfg(feature = "UnityEngine+Timeline+TimelineClip+TimelineClipUpgrade")]
    pub type TimelineClipUpgrade = crate::UnityEngine::Timeline::TimelineClip_TimelineClipUpgrade;
    #[cfg(feature = "UnityEngine+Timeline+TimelineClip+ClipExtrapolation")]
    pub type ClipExtrapolation = crate::UnityEngine::Timeline::TimelineClip_ClipExtrapolation;
    #[cfg(feature = "UnityEngine+Timeline+TimelineClip+BlendCurveMode")]
    pub type BlendCurveMode = crate::UnityEngine::Timeline::TimelineClip_BlendCurveMode;
    pub fn get_hasPostExtrapolation(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasPostExtrapolation", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_mixOutDuration(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_mixOutDuration", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsExtrapolatedTime(
        &mut self,
        sequenceTime: f64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsExtrapolatedTime", (sequenceTime))?;
        Ok(__cordl_ret)
    }
    pub fn IsPostExtrapolatedTime(
        &mut self,
        sequenceTime: f64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsPostExtrapolatedTime", (sequenceTime))?;
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_Timeline_ICurvesOwner_get_targetTrack(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Timeline::TrackAsset> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Timeline::TrackAsset = __cordl_object
            .invoke("UnityEngine.Timeline.ICurvesOwner.get_targetTrack", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_recordable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_recordable", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_extrapolatedStart(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_extrapolatedStart", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_ISerializationCallbackReceiver_OnAfterDeserialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.ISerializationCallbackReceiver.OnAfterDeserialize",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn EvaluateMixIn(
        &mut self,
        _cordl_time: f64,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("EvaluateMixIn", (_cordl_time))?;
        Ok(__cordl_ret)
    }
    pub fn get_start(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_start", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_mixInPercentage(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_mixInPercentage", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_mixInDuration(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_mixInDuration", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_blendInCurveMode(
        &mut self,
        value: crate::UnityEngine::Timeline::TimelineClip_BlendCurveMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_blendInCurveMode", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_hasCurves(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasCurves", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_timeScale(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_timeScale", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_displayName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_displayName", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_mixInCurve(
        &mut self,
        value: *mut crate::UnityEngine::AnimationCurve,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_mixInCurve", (value))?;
        Ok(__cordl_ret)
    }
    pub fn FromLocalTimeUnbound(
        &mut self,
        _cordl_time: f64,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object
            .invoke("FromLocalTimeUnbound", (_cordl_time))?;
        Ok(__cordl_ret)
    }
    pub fn set_blendInDuration(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_blendInDuration", (value))?;
        Ok(__cordl_ret)
    }
    pub fn EvaluateMixOut(
        &mut self,
        _cordl_time: f64,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("EvaluateMixOut", (_cordl_time))?;
        Ok(__cordl_ret)
    }
    pub fn set_parentTrack(
        &mut self,
        value: *mut crate::UnityEngine::Timeline::TrackAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_parentTrack", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_clipIn(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_clipIn", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateDirty(
        &mut self,
        oldValue: f64,
        newValue: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateDirty", (oldValue, newValue))?;
        Ok(__cordl_ret)
    }
    pub fn get_hasBlendIn(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasBlendIn", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_postExtrapolationMode(
        &mut self,
        value: crate::UnityEngine::Timeline::TimelineClip_ClipExtrapolation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_postExtrapolationMode", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_hasPreExtrapolation(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasPreExtrapolation", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_duration(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_duration", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_curves(
        &mut self,
        value: *mut crate::UnityEngine::AnimationClip,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_curves", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_asset(
        &mut self,
        value: *mut crate::UnityEngine::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_asset", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_mixOutPercentage(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_mixOutPercentage", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_preExtrapolationMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Timeline::TimelineClip_ClipExtrapolation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Timeline::TimelineClip_ClipExtrapolation = __cordl_object
            .invoke("get_preExtrapolationMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateCurves(
        &mut self,
        curvesClipName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateCurves", (curvesClipName))?;
        Ok(__cordl_ret)
    }
    pub fn get_easeOutTime(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_easeOutTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_blendOutDuration(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_blendOutDuration", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_mixOutTime(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_mixOutTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_exposedParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_exposedParameters", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_displayName(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_displayName", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_curves(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::AnimationClip> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AnimationClip = __cordl_object
            .invoke("get_curves", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsPreExtrapolatedTime(
        &mut self,
        sequenceTime: f64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsPreExtrapolatedTime", (sequenceTime))?;
        Ok(__cordl_ret)
    }
    pub fn get_mixOutCurve(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::AnimationCurve> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AnimationCurve = __cordl_object
            .invoke("get_mixOutCurve", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_underlyingAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Object = __cordl_object
            .invoke("get_underlyingAsset", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_easeInDuration(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_easeInDuration", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_easeOutDuration(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_easeOutDuration", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_mixOutCurve(
        &mut self,
        value: *mut crate::UnityEngine::AnimationCurve,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_mixOutCurve", (value))?;
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_ISerializationCallbackReceiver_OnBeforeSerialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.ISerializationCallbackReceiver.OnBeforeSerialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_hasBlendOut(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasBlendOut", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_timeScale(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_timeScale", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_blendInDuration(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_blendInDuration", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_mixInCurve(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::AnimationCurve> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AnimationCurve = __cordl_object
            .invoke("get_mixInCurve", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_eastOutTime(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_eastOutTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_end(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_end", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_Timeline_ICurvesOwner_get_defaultCurvesName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("UnityEngine.Timeline.ICurvesOwner.get_defaultCurvesName", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_Timeline_ICurvesOwner_get_assetOwner(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Object = __cordl_object
            .invoke("UnityEngine.Timeline.ICurvesOwner.get_assetOwner", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_start(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_start", (value))?;
        Ok(__cordl_ret)
    }
    pub fn UpgradeToLatestVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpgradeToLatestVersion", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_clipAssetDuration(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_clipAssetDuration", ())?;
        Ok(__cordl_ret)
    }
    pub fn ToLocalTime(
        &mut self,
        _cordl_time: f64,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("ToLocalTime", (_cordl_time))?;
        Ok(__cordl_ret)
    }
    pub fn SetPostExtrapolationTime(
        &mut self,
        _cordl_time: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPostExtrapolationTime", (_cordl_time))?;
        Ok(__cordl_ret)
    }
    pub fn get_extrapolatedDuration(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_extrapolatedDuration", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_clipIn(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_clipIn", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_blendOutCurveMode(
        &mut self,
        value: crate::UnityEngine::Timeline::TimelineClip_BlendCurveMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_blendOutCurveMode", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_blendOutCurveMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Timeline::TimelineClip_BlendCurveMode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Timeline::TimelineClip_BlendCurveMode = __cordl_object
            .invoke("get_blendOutCurveMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn ToLocalTimeUnbound(
        &mut self,
        _cordl_time: f64,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object
            .invoke("ToLocalTimeUnbound", (_cordl_time))?;
        Ok(__cordl_ret)
    }
    pub fn get_postExtrapolationMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Timeline::TimelineClip_ClipExtrapolation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Timeline::TimelineClip_ClipExtrapolation = __cordl_object
            .invoke("get_postExtrapolationMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_clipCaps(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Timeline::ClipCaps> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Timeline::ClipCaps = __cordl_object
            .invoke("get_clipCaps", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_blendInCurveMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Timeline::TimelineClip_BlendCurveMode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Timeline::TimelineClip_BlendCurveMode = __cordl_object
            .invoke("get_blendInCurveMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetParentTrack_Internal(
        &mut self,
        newParentTrack: *mut crate::UnityEngine::Timeline::TrackAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetParentTrack_Internal", (newParentTrack))?;
        Ok(__cordl_ret)
    }
    pub fn ConformEaseValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConformEaseValues", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_asset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Object = __cordl_object
            .invoke("get_asset", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_duration(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_duration", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_blendOutDuration(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_blendOutDuration", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetPreExtrapolationTime(
        &mut self,
        _cordl_time: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPreExtrapolationTime", (_cordl_time))?;
        Ok(__cordl_ret)
    }
    pub fn GetParentTrack(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Timeline::TrackAsset> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Timeline::TrackAsset = __cordl_object
            .invoke("GetParentTrack", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_parentTrack(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Timeline::TrackAsset> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Timeline::TrackAsset = __cordl_object
            .invoke("get_parentTrack", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_easeInDuration(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_easeInDuration", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        parent: *mut crate::UnityEngine::Timeline::TrackAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (parent))?;
        Ok(__cordl_ret)
    }
    pub fn get_animationClip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::AnimationClip> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AnimationClip = __cordl_object
            .invoke("get_animationClip", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_easeOutDuration(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_easeOutDuration", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_recordable(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_recordable", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_underlyingAsset(
        &mut self,
        value: *mut crate::UnityEngine::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_underlyingAsset", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Hash(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Hash", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_preExtrapolationMode(
        &mut self,
        value: crate::UnityEngine::Timeline::TimelineClip_ClipExtrapolation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_preExtrapolationMode", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        parent: *mut crate::UnityEngine::Timeline::TrackAsset,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parent))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineClip")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Timeline::TimelineClip {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineClip+TimelineClipUpgrade")]
#[repr(C)]
#[derive(Debug)]
pub struct TimelineClip_TimelineClipUpgrade {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+Timeline+TimelineClip+TimelineClipUpgrade")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Timeline::TimelineClip_TimelineClipUpgrade => "UnityEngine.Timeline"
    ."TimelineClip/TimelineClipUpgrade"
);
#[cfg(feature = "UnityEngine+Timeline+TimelineClip+TimelineClipUpgrade")]
impl std::ops::Deref for crate::UnityEngine::Timeline::TimelineClip_TimelineClipUpgrade {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineClip+TimelineClipUpgrade")]
impl std::ops::DerefMut
for crate::UnityEngine::Timeline::TimelineClip_TimelineClipUpgrade {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineClip+TimelineClipUpgrade")]
impl crate::UnityEngine::Timeline::TimelineClip_TimelineClipUpgrade {}
#[cfg(feature = "UnityEngine+Timeline+TimelineClip+TimelineClipUpgrade")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::TimelineClip_TimelineClipUpgrade {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineClip+Versions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimelineClip_Versions {
    ClipInFromGlobalToLocal = 1i32,
    Initial = 0i32,
}
#[cfg(feature = "UnityEngine+Timeline+TimelineClip+Versions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::TimelineClip_Versions =>
    "UnityEngine.Timeline"."TimelineClip/Versions"
);
