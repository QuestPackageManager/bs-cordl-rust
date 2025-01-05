#[cfg(feature = "UnityEngine+Animations+AnimationClipPlayable")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct AnimationClipPlayable {
    pub m_Handle: crate::UnityEngine::Playables::PlayableHandle,
}
#[cfg(feature = "UnityEngine+Animations+AnimationClipPlayable")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Animations::AnimationClipPlayable
    => "UnityEngine.Animations"."AnimationClipPlayable"
);
#[cfg(feature = "UnityEngine+Animations+AnimationClipPlayable")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Animations::AnimationClipPlayable {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Animations+AnimationClipPlayable")]
impl crate::UnityEngine::Animations::AnimationClipPlayable {
    pub fn Create(
        graph: crate::UnityEngine::Playables::PlayableGraph,
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Animations::AnimationClipPlayable,
    > {
        let __cordl_ret: crate::UnityEngine::Animations::AnimationClipPlayable = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (graph, clip))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateHandle(
        graph: crate::UnityEngine::Playables::PlayableGraph,
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::PlayableHandle> {
        let __cordl_ret: crate::UnityEngine::Playables::PlayableHandle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateHandle", (graph, clip))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateHandleInternal(
        graph: crate::UnityEngine::Playables::PlayableGraph,
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
        handle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateHandleInternal", (graph, clip, handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateHandleInternal_Injected(
        graph: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableGraph,
        >,
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
        handle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateHandleInternal_Injected", (graph, clip, handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        other: crate::UnityEngine::Animations::AnimationClipPlayable,
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
    pub fn SetApplyFootIK(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetApplyFootIK",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetApplyFootIKInternal(
        handle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetApplyFootIKInternal", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLoopTime(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetLoopTime",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLoopTimeInternal(
        handle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetLoopTimeInternal", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetOverrideLoopTime(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetOverrideLoopTime",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetOverrideLoopTimeInternal(
        handle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetOverrideLoopTimeInternal", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetRemoveStartOffset(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetRemoveStartOffset",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetRemoveStartOffsetInternal(
        handle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetRemoveStartOffsetInternal", (handle, value))?;
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
        playable: crate::UnityEngine::Animations::AnimationClipPlayable,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_ret: crate::UnityEngine::Playables::Playable = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (playable))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Animations+AnimationClipPlayable")]
impl AsRef<
    crate::System::IEquatable_1<crate::UnityEngine::Animations::AnimationClipPlayable>,
> for crate::UnityEngine::Animations::AnimationClipPlayable {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::UnityEngine::Animations::AnimationClipPlayable,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Animations+AnimationClipPlayable")]
impl AsMut<
    crate::System::IEquatable_1<crate::UnityEngine::Animations::AnimationClipPlayable>,
> for crate::UnityEngine::Animations::AnimationClipPlayable {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::Animations::AnimationClipPlayable,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Animations+AnimationClipPlayable")]
impl AsRef<crate::UnityEngine::Playables::IPlayable>
for crate::UnityEngine::Animations::AnimationClipPlayable {
    fn as_ref(&self) -> &crate::UnityEngine::Playables::IPlayable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Animations+AnimationClipPlayable")]
impl AsMut<crate::UnityEngine::Playables::IPlayable>
for crate::UnityEngine::Animations::AnimationClipPlayable {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Playables::IPlayable {
        todo!()
    }
}
