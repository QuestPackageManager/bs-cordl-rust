#[cfg(feature = "OVRHaptics+OVRHapticsOutput+ClipPlaybackTracker")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRHapticsOutput_ClipPlaybackTracker {
    __cordl_parent: crate::System::Object,
    pub _ReadCount_k__BackingField: i32,
    pub _Clip_k__BackingField: *mut OVRHapticsClip,
}
#[cfg(feature = "OVRHaptics+OVRHapticsOutput+ClipPlaybackTracker")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRHapticsOutput_ClipPlaybackTracker => ""
    ."OVRHaptics/OVRHapticsOutput/ClipPlaybackTracker"
);
#[cfg(feature = "OVRHaptics+OVRHapticsOutput+ClipPlaybackTracker")]
impl std::ops::Deref for crate::GlobalNamespace::OVRHapticsOutput_ClipPlaybackTracker {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRHaptics+OVRHapticsOutput+ClipPlaybackTracker")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRHapticsOutput_ClipPlaybackTracker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRHaptics+OVRHapticsOutput+ClipPlaybackTracker")]
impl crate::GlobalNamespace::OVRHapticsOutput_ClipPlaybackTracker {
    pub fn New(clip: *mut OVRHapticsClip) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (clip))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        clip: *mut OVRHapticsClip,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (clip))?;
        Ok(__cordl_ret)
    }
    pub fn get_Clip(&mut self) -> quest_hook::libil2cpp::Result<*mut OVRHapticsClip> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut OVRHapticsClip = __cordl_object.invoke("get_Clip", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ReadCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ReadCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Clip(
        &mut self,
        value: *mut OVRHapticsClip,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Clip", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ReadCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ReadCount", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRHaptics+OVRHapticsOutput+ClipPlaybackTracker")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRHapticsOutput_ClipPlaybackTracker {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRHaptics+Config")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRHaptics_Config {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRHaptics+Config")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRHaptics_Config => ""
    ."OVRHaptics/Config"
);
#[cfg(feature = "OVRHaptics+Config")]
impl std::ops::Deref for crate::GlobalNamespace::OVRHaptics_Config {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRHaptics+Config")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRHaptics_Config {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRHaptics+Config")]
impl crate::GlobalNamespace::OVRHaptics_Config {}
#[cfg(feature = "OVRHaptics+Config")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRHaptics_Config {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRHaptics")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRHaptics {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRHaptics")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OVRHaptics => ""."OVRHaptics"
);
#[cfg(feature = "OVRHaptics")]
impl std::ops::Deref for OVRHaptics {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRHaptics")]
impl std::ops::DerefMut for OVRHaptics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRHaptics")]
impl OVRHaptics {
    #[cfg(feature = "OVRHaptics+Config")]
    pub type Config = crate::GlobalNamespace::OVRHaptics_Config;
    #[cfg(feature = "OVRHaptics+OVRHapticsOutput")]
    pub type OVRHapticsOutput = crate::GlobalNamespace::OVRHaptics_OVRHapticsOutput;
    #[cfg(feature = "OVRHaptics+OVRHapticsChannel")]
    pub type OVRHapticsChannel = crate::GlobalNamespace::OVRHaptics_OVRHapticsChannel;
}
#[cfg(feature = "OVRHaptics")]
impl quest_hook::libil2cpp::ObjectType for OVRHaptics {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRHaptics+OVRHapticsChannel")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRHaptics_OVRHapticsChannel {
    __cordl_parent: crate::System::Object,
    pub m_output: *mut crate::GlobalNamespace::OVRHaptics_OVRHapticsOutput,
}
#[cfg(feature = "OVRHaptics+OVRHapticsChannel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRHaptics_OVRHapticsChannel =>
    ""."OVRHaptics/OVRHapticsChannel"
);
#[cfg(feature = "OVRHaptics+OVRHapticsChannel")]
impl std::ops::Deref for crate::GlobalNamespace::OVRHaptics_OVRHapticsChannel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRHaptics+OVRHapticsChannel")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRHaptics_OVRHapticsChannel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRHaptics+OVRHapticsChannel")]
impl crate::GlobalNamespace::OVRHaptics_OVRHapticsChannel {
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret)
    }
    pub fn Mix(
        &mut self,
        clip: *mut OVRHapticsClip,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Mix", (clip))?;
        Ok(__cordl_ret)
    }
    pub fn New(outputIndex: u32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (outputIndex))?;
        Ok(__cordl_object)
    }
    pub fn Preempt(
        &mut self,
        clip: *mut OVRHapticsClip,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Preempt", (clip))?;
        Ok(__cordl_ret)
    }
    pub fn Queue(
        &mut self,
        clip: *mut OVRHapticsClip,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Queue", (clip))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        outputIndex: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (outputIndex))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRHaptics+OVRHapticsChannel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRHaptics_OVRHapticsChannel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRHaptics+OVRHapticsOutput")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRHaptics_OVRHapticsOutput {
    __cordl_parent: crate::System::Object,
    pub m_lowLatencyMode: bool,
    pub m_prevSamplesQueued: i32,
    pub m_prevSamplesQueuedTime: f32,
    pub m_numPredictionHits: i32,
    pub m_numPredictionMisses: i32,
    pub m_numUnderruns: i32,
    pub m_pendingClips: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::OVRHapticsOutput_ClipPlaybackTracker,
    >,
    pub m_controller: u32,
    pub m_nativeBuffer: *mut OVRNativeBuffer,
    pub PrevSampleRateHz: i32,
}
#[cfg(feature = "OVRHaptics+OVRHapticsOutput")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRHaptics_OVRHapticsOutput =>
    ""."OVRHaptics/OVRHapticsOutput"
);
#[cfg(feature = "OVRHaptics+OVRHapticsOutput")]
impl std::ops::Deref for crate::GlobalNamespace::OVRHaptics_OVRHapticsOutput {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRHaptics+OVRHapticsOutput")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRHaptics_OVRHapticsOutput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRHaptics+OVRHapticsOutput")]
impl crate::GlobalNamespace::OVRHaptics_OVRHapticsOutput {
    #[cfg(feature = "OVRHaptics+OVRHapticsOutput+ClipPlaybackTracker")]
    pub type ClipPlaybackTracker = crate::GlobalNamespace::OVRHapticsOutput_ClipPlaybackTracker;
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret)
    }
    pub fn Mix(
        &mut self,
        clip: *mut OVRHapticsClip,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Mix", (clip))?;
        Ok(__cordl_ret)
    }
    pub fn New(controller: u32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (controller))?;
        Ok(__cordl_object)
    }
    pub fn Preempt(
        &mut self,
        clip: *mut OVRHapticsClip,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Preempt", (clip))?;
        Ok(__cordl_ret)
    }
    pub fn Process(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Process", ())?;
        Ok(__cordl_ret)
    }
    pub fn Queue(
        &mut self,
        clip: *mut OVRHapticsClip,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Queue", (clip))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        controller: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (controller))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRHaptics+OVRHapticsOutput")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRHaptics_OVRHapticsOutput {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
