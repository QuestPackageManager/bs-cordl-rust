#[cfg(feature = "UnityEngine+Playables+PlayableOutput")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PlayableOutput {
    pub m_Handle: crate::UnityEngine::Playables::PlayableOutputHandle,
}
#[cfg(feature = "UnityEngine+Playables+PlayableOutput")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Playables::PlayableOutput =>
    "UnityEngine.Playables"."PlayableOutput"
);
#[cfg(feature = "UnityEngine+Playables+PlayableOutput")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Playables::PlayableOutput {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableOutput")]
impl crate::UnityEngine::Playables::PlayableOutput {
    pub fn Equals(
        &mut self,
        other: crate::UnityEngine::Playables::PlayableOutput,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
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
    pub fn IsPlayableOutputOfType<T>(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsPlayableOutputOfType",
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
}