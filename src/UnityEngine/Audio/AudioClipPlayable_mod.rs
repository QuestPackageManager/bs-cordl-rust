#[cfg(feature = "UnityEngine+Audio+AudioClipPlayable")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AudioClipPlayable {
    pub m_Handle: crate::UnityEngine::Playables::PlayableHandle,
}
#[cfg(feature = "UnityEngine+Audio+AudioClipPlayable")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Audio::AudioClipPlayable {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Audio";
    const CLASS_NAME: &'static str = "AudioClipPlayable";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+Audio+AudioClipPlayable")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Audio::AudioClipPlayable {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+Audio+AudioClipPlayable")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Audio::AudioClipPlayable {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+Audio+AudioClipPlayable")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Audio::AudioClipPlayable {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+Audio+AudioClipPlayable")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Audio::AudioClipPlayable {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
    pub fn Create(
        graph: crate::UnityEngine::Playables::PlayableGraph,
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        looping: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Audio::AudioClipPlayable> {
        let __cordl_ret: crate::UnityEngine::Audio::AudioClipPlayable = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (graph, clip, looping))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateHandle(
        graph: crate::UnityEngine::Playables::PlayableGraph,
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        looping: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::PlayableHandle> {
        let __cordl_ret: crate::UnityEngine::Playables::PlayableHandle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateHandle", (graph, clip, looping))?;
        Ok(__cordl_ret.into())
    }
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
    pub fn GetClipInternal(
        hdl: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetClipInternal", (hdl))?;
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
    pub fn GetIsChannelPlayingInternal(
        hdl: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetIsChannelPlayingInternal", (hdl))?;
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
    pub fn GetLoopedInternal(
        hdl: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLoopedInternal", (hdl))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPauseDelayInternal(
        hdl: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPauseDelayInternal", (hdl))?;
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
    pub fn GetSpatialBlendInternal(
        hdl: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSpatialBlendInternal", (hdl))?;
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
    pub fn GetStartDelayInternal(
        hdl: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStartDelayInternal", (hdl))?;
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
    pub fn GetStereoPanInternal(
        hdl: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStereoPanInternal", (hdl))?;
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
    pub fn GetVolumeInternal(
        hdl: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetVolumeInternal", (hdl))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalCreateAudioClipPlayable(
        graph: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableGraph,
        >,
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        looping: bool,
        handle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalCreateAudioClipPlayable", (graph, clip, looping, handle))?;
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
    pub fn SetClipInternal(
        hdl: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetClipInternal", (hdl, clip))?;
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
    pub fn SetLoopedInternal(
        hdl: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
        looped: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetLoopedInternal", (hdl, looped))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPauseDelayInternal(
        hdl: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
        delay: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetPauseDelayInternal", (hdl, delay))?;
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
    pub fn SetSpatialBlendInternal(
        hdl: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
        spatialBlend: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetSpatialBlendInternal", (hdl, spatialBlend))?;
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
    pub fn SetStartDelayInternal(
        hdl: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
        delay: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetStartDelayInternal", (hdl, delay))?;
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
    pub fn SetStereoPanInternal(
        hdl: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
        stereoPan: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetStereoPanInternal", (hdl, stereoPan))?;
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
    pub fn SetVolumeInternal(
        hdl: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
        volume: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetVolumeInternal", (hdl, volume))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateType(
        hdl: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateType", (hdl))?;
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
    pub fn op_Explicit(
        playable: crate::UnityEngine::Playables::Playable,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Audio::AudioClipPlayable> {
        let __cordl_ret: crate::UnityEngine::Audio::AudioClipPlayable = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (playable))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        playable: crate::UnityEngine::Audio::AudioClipPlayable,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_ret: crate::UnityEngine::Playables::Playable = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (playable))?;
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
