#[cfg(feature = "UnityEngine+Animations+AnimationLayerMixerPlayable")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AnimationLayerMixerPlayable {
    pub m_Handle: crate::UnityEngine::Playables::PlayableHandle,
}
#[cfg(feature = "UnityEngine+Animations+AnimationLayerMixerPlayable")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Animations::AnimationLayerMixerPlayable => "UnityEngine.Animations"
    ."AnimationLayerMixerPlayable"
);
#[cfg(feature = "UnityEngine+Animations+AnimationLayerMixerPlayable")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Animations::AnimationLayerMixerPlayable {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Animations+AnimationLayerMixerPlayable")]
impl crate::UnityEngine::Animations::AnimationLayerMixerPlayable {
    pub fn Equals(
        &mut self,
        other: crate::UnityEngine::Animations::AnimationLayerMixerPlayable,
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
    pub fn SetLayerMaskFromAvatarMask(
        &mut self,
        layerIndex: u32,
        mask: quest_hook::libil2cpp::Gc<crate::UnityEngine::AvatarMask>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetLayerMaskFromAvatarMask",
            (layerIndex, mask),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        handle: crate::UnityEngine::Playables::PlayableHandle,
        singleLayerOptimization: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (handle, singleLayerOptimization),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Animations+AnimationLayerMixerPlayable")]
impl AsRef<
    crate::System::IEquatable_1<
        crate::UnityEngine::Animations::AnimationLayerMixerPlayable,
    >,
> for crate::UnityEngine::Animations::AnimationLayerMixerPlayable {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::UnityEngine::Animations::AnimationLayerMixerPlayable,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Animations+AnimationLayerMixerPlayable")]
impl AsMut<
    crate::System::IEquatable_1<
        crate::UnityEngine::Animations::AnimationLayerMixerPlayable,
    >,
> for crate::UnityEngine::Animations::AnimationLayerMixerPlayable {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::Animations::AnimationLayerMixerPlayable,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Animations+AnimationLayerMixerPlayable")]
impl AsRef<crate::UnityEngine::Playables::IPlayable>
for crate::UnityEngine::Animations::AnimationLayerMixerPlayable {
    fn as_ref(&self) -> &crate::UnityEngine::Playables::IPlayable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Animations+AnimationLayerMixerPlayable")]
impl AsMut<crate::UnityEngine::Playables::IPlayable>
for crate::UnityEngine::Animations::AnimationLayerMixerPlayable {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Playables::IPlayable {
        todo!()
    }
}
