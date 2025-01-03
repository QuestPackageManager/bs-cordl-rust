#[cfg(feature = "UnityEngine+AudioConfiguration")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct AudioConfiguration {
    pub speakerMode: crate::UnityEngine::AudioSpeakerMode,
    pub dspBufferSize: i32,
    pub sampleRate: i32,
    pub numRealVoices: i32,
    pub numVirtualVoices: i32,
}
#[cfg(feature = "UnityEngine+AudioConfiguration")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AudioConfiguration => "UnityEngine"
    ."AudioConfiguration"
);
#[cfg(feature = "UnityEngine+AudioConfiguration")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::AudioConfiguration {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+AudioConfiguration")]
impl crate::UnityEngine::AudioConfiguration {}
