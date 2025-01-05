#[cfg(feature = "UnityEngine+Experimental+Playables+TexturePlayableOutput")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TexturePlayableOutput {
    pub m_Handle: crate::UnityEngine::Playables::PlayableOutputHandle,
}
#[cfg(feature = "UnityEngine+Experimental+Playables+TexturePlayableOutput")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Experimental::Playables::TexturePlayableOutput =>
    "UnityEngine.Experimental.Playables"."TexturePlayableOutput"
);
#[cfg(feature = "UnityEngine+Experimental+Playables+TexturePlayableOutput")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Experimental::Playables::TexturePlayableOutput {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Experimental+Playables+TexturePlayableOutput")]
impl crate::UnityEngine::Experimental::Playables::TexturePlayableOutput {
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
}
#[cfg(feature = "UnityEngine+Experimental+Playables+TexturePlayableOutput")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::UnityEngine::Playables::IPlayableOutput>>
for crate::UnityEngine::Experimental::Playables::TexturePlayableOutput {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::UnityEngine::Playables::IPlayableOutput> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Experimental+Playables+TexturePlayableOutput")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::UnityEngine::Playables::IPlayableOutput>>
for crate::UnityEngine::Experimental::Playables::TexturePlayableOutput {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::UnityEngine::Playables::IPlayableOutput> {
        todo!()
    }
}
