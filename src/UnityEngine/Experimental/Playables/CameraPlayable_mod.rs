#[cfg(feature = "UnityEngine+Experimental+Playables+CameraPlayable")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct CameraPlayable {
    pub m_Handle: crate::UnityEngine::Playables::PlayableHandle,
}
#[cfg(feature = "UnityEngine+Experimental+Playables+CameraPlayable")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Experimental::Playables::CameraPlayable =>
    "UnityEngine.Experimental.Playables"."CameraPlayable"
);
#[cfg(feature = "UnityEngine+Experimental+Playables+CameraPlayable")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Experimental::Playables::CameraPlayable {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Experimental+Playables+CameraPlayable")]
impl crate::UnityEngine::Experimental::Playables::CameraPlayable {
    pub fn Equals(
        &mut self,
        other: crate::UnityEngine::Experimental::Playables::CameraPlayable,
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
#[cfg(feature = "UnityEngine+Experimental+Playables+CameraPlayable")]
impl AsRef<
    quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Experimental::Playables::CameraPlayable,
    >,
> for crate::UnityEngine::Experimental::Playables::CameraPlayable {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Experimental::Playables::CameraPlayable,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Experimental+Playables+CameraPlayable")]
impl AsMut<
    quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Experimental::Playables::CameraPlayable,
    >,
> for crate::UnityEngine::Experimental::Playables::CameraPlayable {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Experimental::Playables::CameraPlayable,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Experimental+Playables+CameraPlayable")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::UnityEngine::Playables::IPlayable>>
for crate::UnityEngine::Experimental::Playables::CameraPlayable {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::UnityEngine::Playables::IPlayable> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Experimental+Playables+CameraPlayable")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::UnityEngine::Playables::IPlayable>>
for crate::UnityEngine::Experimental::Playables::CameraPlayable {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::UnityEngine::Playables::IPlayable> {
        todo!()
    }
}
