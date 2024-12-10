#[cfg(feature = "UnityEngine+Playables+ScriptPlayableOutput")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ScriptPlayableOutput {
    pub m_Handle: crate::UnityEngine::Playables::PlayableOutputHandle,
}
#[cfg(feature = "UnityEngine+Playables+ScriptPlayableOutput")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Playables::ScriptPlayableOutput =>
    "UnityEngine.Playables"."ScriptPlayableOutput"
);
#[cfg(feature = "UnityEngine+Playables+ScriptPlayableOutput")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Playables::ScriptPlayableOutput {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Playables+ScriptPlayableOutput")]
impl crate::UnityEngine::Playables::ScriptPlayableOutput {
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Playables+ScriptPlayableOutput")]
impl AsRef<crate::UnityEngine::Playables::IPlayableOutput>
for crate::UnityEngine::Playables::ScriptPlayableOutput {
    fn as_ref(&self) -> &crate::UnityEngine::Playables::IPlayableOutput {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Playables+ScriptPlayableOutput")]
impl AsMut<crate::UnityEngine::Playables::IPlayableOutput>
for crate::UnityEngine::Playables::ScriptPlayableOutput {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Playables::IPlayableOutput {
        todo!()
    }
}
