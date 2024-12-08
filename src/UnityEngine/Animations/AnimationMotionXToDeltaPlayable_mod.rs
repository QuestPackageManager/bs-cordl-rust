#[cfg(feature = "UnityEngine+Animations+AnimationMotionXToDeltaPlayable")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AnimationMotionXToDeltaPlayable {
    pub m_Handle: crate::UnityEngine::Playables::PlayableHandle,
}
#[cfg(feature = "UnityEngine+Animations+AnimationMotionXToDeltaPlayable")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Animations::AnimationMotionXToDeltaPlayable =>
    "UnityEngine.Animations"."AnimationMotionXToDeltaPlayable"
);
#[cfg(feature = "UnityEngine+Animations+AnimationMotionXToDeltaPlayable")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Animations::AnimationMotionXToDeltaPlayable {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Animations+AnimationMotionXToDeltaPlayable")]
impl crate::UnityEngine::Animations::AnimationMotionXToDeltaPlayable {
    pub fn Equals(
        &mut self,
        other: crate::UnityEngine::Animations::AnimationMotionXToDeltaPlayable,
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
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::PlayableHandle> {
        let __cordl_ret: crate::UnityEngine::Playables::PlayableHandle = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHandle",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetAbsoluteMotion(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetAbsoluteMotion",
            (value),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
}