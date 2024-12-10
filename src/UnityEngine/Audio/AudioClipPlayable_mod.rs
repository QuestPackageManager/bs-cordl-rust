#[cfg(feature = "UnityEngine+Audio+AudioClipPlayable")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AudioClipPlayable {
    pub m_Handle: crate::UnityEngine::Playables::PlayableHandle,
}
#[cfg(feature = "UnityEngine+Audio+AudioClipPlayable")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Audio::AudioClipPlayable =>
    "UnityEngine.Audio"."AudioClipPlayable"
);
#[cfg(feature = "UnityEngine+Audio+AudioClipPlayable")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Audio::AudioClipPlayable {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Audio+AudioClipPlayable")]
impl crate::UnityEngine::Audio::AudioClipPlayable {
    pub fn Equals(
        &mut self,
        other: crate::UnityEngine::Audio::AudioClipPlayable,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetClip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetClip",
            (),
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
    pub fn GetLooped(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetLooped",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPauseDelay_0(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetPauseDelay",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPauseDelay_f64_1(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetPauseDelay",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpatialBlend(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetSpatialBlend",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStartDelay(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetStartDelay",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStereoPan(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetStereoPan",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVolume(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetVolume",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsChannelPlaying(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsChannelPlaying",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPlaying(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsPlaying",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Seek_f64_1(
        &mut self,
        startTime: f64,
        startDelay: f64,
        duration: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Seek",
            (startTime, startDelay, duration),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Seek_f64_f64_0(
        &mut self,
        startTime: f64,
        startDelay: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Seek",
            (startTime, startDelay),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetClip(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetClip",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLooped(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetLooped",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSpatialBlend(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetSpatialBlend",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStartDelay(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetStartDelay",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStereoPan(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetStereoPan",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetVolume(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetVolume",
            (value),
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
}
#[cfg(feature = "UnityEngine+Audio+AudioClipPlayable")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::Audio::AudioClipPlayable>>
for crate::UnityEngine::Audio::AudioClipPlayable {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::Audio::AudioClipPlayable> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Audio+AudioClipPlayable")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::Audio::AudioClipPlayable>>
for crate::UnityEngine::Audio::AudioClipPlayable {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::Audio::AudioClipPlayable> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Audio+AudioClipPlayable")]
impl AsRef<crate::UnityEngine::Playables::IPlayable>
for crate::UnityEngine::Audio::AudioClipPlayable {
    fn as_ref(&self) -> &crate::UnityEngine::Playables::IPlayable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Audio+AudioClipPlayable")]
impl AsMut<crate::UnityEngine::Playables::IPlayable>
for crate::UnityEngine::Audio::AudioClipPlayable {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Playables::IPlayable {
        todo!()
    }
}
