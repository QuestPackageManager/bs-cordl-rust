#[cfg(feature = "UnityEngine+Playables+FrameData")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct FrameData {
    pub m_FrameID: u64,
    pub m_DeltaTime: f64,
    pub m_Weight: f32,
    pub m_EffectiveWeight: f32,
    pub m_EffectiveParentDelay: f64,
    pub m_EffectiveParentSpeed: f32,
    pub m_EffectiveSpeed: f32,
    pub m_Flags: crate::UnityEngine::Playables::FrameData_Flags,
    pub m_Output: crate::UnityEngine::Playables::PlayableOutput,
}
#[cfg(feature = "UnityEngine+Playables+FrameData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Playables::FrameData =>
    "UnityEngine.Playables"."FrameData"
);
#[cfg(feature = "UnityEngine+Playables+FrameData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Playables::FrameData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Playables+FrameData")]
impl crate::UnityEngine::Playables::FrameData {
    #[cfg(feature = "UnityEngine+Playables+FrameData+EvaluationType")]
    pub type EvaluationType = crate::UnityEngine::Playables::FrameData_EvaluationType;
    #[cfg(feature = "UnityEngine+Playables+FrameData+Flags")]
    pub type Flags = crate::UnityEngine::Playables::FrameData_Flags;
    pub fn HasFlags(
        &mut self,
        flag: crate::UnityEngine::Playables::FrameData_Flags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "HasFlags",
            (flag),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_deltaTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_deltaTime",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_effectivePlayState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::PlayState> {
        let __cordl_ret: crate::UnityEngine::Playables::PlayState = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_effectivePlayState",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_effectiveSpeed(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_effectiveSpeed",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_evaluationType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Playables::FrameData_EvaluationType,
    > {
        let __cordl_ret: crate::UnityEngine::Playables::FrameData_EvaluationType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_evaluationType",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_output(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::PlayableOutput> {
        let __cordl_ret: crate::UnityEngine::Playables::PlayableOutput = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_output",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_seekOccurred(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_seekOccurred",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_timeHeld(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_timeHeld",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_timeLooped(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_timeLooped",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Playables+FrameData+EvaluationType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameData_EvaluationType {
    Evaluate = 0i32,
    Playback = 1i32,
}
#[cfg(feature = "UnityEngine+Playables+FrameData+EvaluationType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Playables::FrameData_EvaluationType
    => "UnityEngine.Playables"."FrameData/EvaluationType"
);
#[cfg(feature = "UnityEngine+Playables+FrameData+Flags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameData_Flags {
    EffectivePlayStateDelayed = 16i32,
    EffectivePlayStatePlaying = 32i32,
    Evaluate = 1i32,
    Hold = 8i32,
    Loop = 4i32,
    SeekOccured = 2i32,
}
#[cfg(feature = "UnityEngine+Playables+FrameData+Flags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Playables::FrameData_Flags =>
    "UnityEngine.Playables"."FrameData/Flags"
);
