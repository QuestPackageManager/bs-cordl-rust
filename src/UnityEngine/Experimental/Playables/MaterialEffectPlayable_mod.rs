#[cfg(feature = "UnityEngine+Experimental+Playables+MaterialEffectPlayable")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct MaterialEffectPlayable {
    pub m_Handle: crate::UnityEngine::Playables::PlayableHandle,
}
#[cfg(feature = "UnityEngine+Experimental+Playables+MaterialEffectPlayable")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Experimental::Playables::MaterialEffectPlayable =>
    "UnityEngine.Experimental.Playables"."MaterialEffectPlayable"
);
#[cfg(feature = "UnityEngine+Experimental+Playables+MaterialEffectPlayable")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Experimental::Playables::MaterialEffectPlayable {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Experimental+Playables+MaterialEffectPlayable")]
impl crate::UnityEngine::Experimental::Playables::MaterialEffectPlayable {
    pub fn Equals(
        &mut self,
        other: crate::UnityEngine::Experimental::Playables::MaterialEffectPlayable,
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
}
#[cfg(feature = "UnityEngine+Experimental+Playables+MaterialEffectPlayable")]
impl AsRef<
    crate::System::IEquatable_1<
        crate::UnityEngine::Experimental::Playables::MaterialEffectPlayable,
    >,
> for crate::UnityEngine::Experimental::Playables::MaterialEffectPlayable {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::UnityEngine::Experimental::Playables::MaterialEffectPlayable,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Experimental+Playables+MaterialEffectPlayable")]
impl AsMut<
    crate::System::IEquatable_1<
        crate::UnityEngine::Experimental::Playables::MaterialEffectPlayable,
    >,
> for crate::UnityEngine::Experimental::Playables::MaterialEffectPlayable {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::Experimental::Playables::MaterialEffectPlayable,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Experimental+Playables+MaterialEffectPlayable")]
impl AsRef<crate::UnityEngine::Playables::IPlayable>
for crate::UnityEngine::Experimental::Playables::MaterialEffectPlayable {
    fn as_ref(&self) -> &crate::UnityEngine::Playables::IPlayable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Experimental+Playables+MaterialEffectPlayable")]
impl AsMut<crate::UnityEngine::Playables::IPlayable>
for crate::UnityEngine::Experimental::Playables::MaterialEffectPlayable {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Playables::IPlayable {
        todo!()
    }
}
