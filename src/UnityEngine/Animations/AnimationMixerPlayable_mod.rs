#[cfg(feature = "UnityEngine+Animations+AnimationMixerPlayable")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct AnimationMixerPlayable {
    pub m_Handle: crate::UnityEngine::Playables::PlayableHandle,
}
#[cfg(feature = "UnityEngine+Animations+AnimationMixerPlayable")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Animations::AnimationMixerPlayable
    => "UnityEngine.Animations"."AnimationMixerPlayable"
);
#[cfg(feature = "UnityEngine+Animations+AnimationMixerPlayable")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Animations::AnimationMixerPlayable {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Animations+AnimationMixerPlayable")]
impl crate::UnityEngine::Animations::AnimationMixerPlayable {
    pub fn Create(
        graph: crate::UnityEngine::Playables::PlayableGraph,
        inputCount: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Animations::AnimationMixerPlayable,
    > {
        let __cordl_ret: crate::UnityEngine::Animations::AnimationMixerPlayable = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (graph, inputCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateHandle(
        graph: crate::UnityEngine::Playables::PlayableGraph,
        inputCount: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::PlayableHandle> {
        let __cordl_ret: crate::UnityEngine::Playables::PlayableHandle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateHandle", (graph, inputCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateHandleInternal(
        graph: crate::UnityEngine::Playables::PlayableGraph,
        handle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateHandleInternal", (graph, handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateHandleInternal_Injected(
        graph: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableGraph,
        >,
        handle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateHandleInternal_Injected", (graph, handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        other: crate::UnityEngine::Animations::AnimationMixerPlayable,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHandle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::PlayableHandle> {
        let __cordl_ret: crate::UnityEngine::Playables::PlayableHandle = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHandle",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        handle: crate::UnityEngine::Playables::PlayableHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (handle),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        playable: crate::UnityEngine::Animations::AnimationMixerPlayable,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_ret: crate::UnityEngine::Playables::Playable = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (playable))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Animations+AnimationMixerPlayable")]
impl AsRef<
    crate::System::IEquatable_1<crate::UnityEngine::Animations::AnimationMixerPlayable>,
> for crate::UnityEngine::Animations::AnimationMixerPlayable {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::UnityEngine::Animations::AnimationMixerPlayable,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Animations+AnimationMixerPlayable")]
impl AsMut<
    crate::System::IEquatable_1<crate::UnityEngine::Animations::AnimationMixerPlayable>,
> for crate::UnityEngine::Animations::AnimationMixerPlayable {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::Animations::AnimationMixerPlayable,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Animations+AnimationMixerPlayable")]
impl AsRef<crate::UnityEngine::Playables::IPlayable>
for crate::UnityEngine::Animations::AnimationMixerPlayable {
    fn as_ref(&self) -> &crate::UnityEngine::Playables::IPlayable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Animations+AnimationMixerPlayable")]
impl AsMut<crate::UnityEngine::Playables::IPlayable>
for crate::UnityEngine::Animations::AnimationMixerPlayable {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Playables::IPlayable {
        todo!()
    }
}
