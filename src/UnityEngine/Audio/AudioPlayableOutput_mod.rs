#[cfg(feature = "UnityEngine+Audio+AudioPlayableOutput")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
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
    pub fn Create(
        graph: crate::UnityEngine::Playables::PlayableGraph,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioSource>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Audio::AudioPlayableOutput> {
        let __cordl_ret: crate::UnityEngine::Audio::AudioPlayableOutput = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (graph, name, target))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEvaluateOnSeek(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetEvaluateOnSeek",
            (),
        )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn GetTarget(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioSource>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioSource> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetTarget",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalGetEvaluateOnSeek(
        output: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableOutputHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalGetEvaluateOnSeek", (output))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalGetTarget(
        output: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableOutputHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioSource>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioSource> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalGetTarget", (output))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalSetEvaluateOnSeek(
        output: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableOutputHandle,
        >,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalSetEvaluateOnSeek", (output, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalSetTarget(
        output: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableOutputHandle,
        >,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalSetTarget", (output, target))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn SetTarget(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetTarget",
            (value),
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
    pub fn get_Null() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Audio::AudioPlayableOutput,
    > {
        let __cordl_ret: crate::UnityEngine::Audio::AudioPlayableOutput = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Null", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit(
        output: crate::UnityEngine::Playables::PlayableOutput,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Audio::AudioPlayableOutput> {
        let __cordl_ret: crate::UnityEngine::Audio::AudioPlayableOutput = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (output))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        output: crate::UnityEngine::Audio::AudioPlayableOutput,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::PlayableOutput> {
        let __cordl_ret: crate::UnityEngine::Playables::PlayableOutput = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (output))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Audio+AudioPlayableOutput")]
impl AsRef<crate::UnityEngine::Playables::IPlayableOutput>
for crate::UnityEngine::Audio::AudioPlayableOutput {
    fn as_ref(&self) -> &crate::UnityEngine::Playables::IPlayableOutput {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Audio+AudioPlayableOutput")]
impl AsMut<crate::UnityEngine::Playables::IPlayableOutput>
for crate::UnityEngine::Audio::AudioPlayableOutput {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Playables::IPlayableOutput {
        todo!()
    }
}
