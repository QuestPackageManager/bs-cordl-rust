#[cfg(feature = "UnityEngine+Audio+AudioPlayableOutput")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AudioPlayableOutput {
    pub m_Handle: crate::UnityEngine::Playables::PlayableOutputHandle,
}
#[cfg(feature = "UnityEngine+Audio+AudioPlayableOutput")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Audio::AudioPlayableOutput =>
    "UnityEngine.Audio"."AudioPlayableOutput"
);
#[cfg(feature = "UnityEngine+Audio+AudioPlayableOutput")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Audio::AudioPlayableOutput {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Audio+AudioPlayableOutput")]
impl crate::UnityEngine::Audio::AudioPlayableOutput {
    pub fn GetTarget(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::AudioSource> {
        let __cordl_ret: *mut crate::UnityEngine::AudioSource = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetTarget",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetTarget(
        &mut self,
        value: *mut crate::UnityEngine::AudioSource,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetTarget",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetEvaluateOnSeek(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetEvaluateOnSeek",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHandle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Playables::PlayableOutputHandle,
    > {
        let __cordl_ret: crate::UnityEngine::Playables::PlayableOutputHandle = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHandle",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        handle: crate::UnityEngine::Playables::PlayableOutputHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (handle),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetEvaluateOnSeek(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetEvaluateOnSeek",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
