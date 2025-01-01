#[cfg(feature = "BeatmapEditorStartTestLevelData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BeatmapEditorStartTestLevelData {
    pub fpfc: crate::GlobalNamespace::BeatmapEditorStartTestLevelData_FpfcData,
    pub overdrawData: crate::GlobalNamespace::BeatmapEditorStartTestLevelData_OverdrawData,
}
#[cfg(feature = "BeatmapEditorStartTestLevelData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapEditorStartTestLevelData
    => ""."BeatmapEditorStartTestLevelData"
);
#[cfg(feature = "BeatmapEditorStartTestLevelData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::BeatmapEditorStartTestLevelData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapEditorStartTestLevelData")]
impl crate::GlobalNamespace::BeatmapEditorStartTestLevelData {
    #[cfg(feature = "BeatmapEditorStartTestLevelData+FpfcData")]
    pub type FpfcData = crate::GlobalNamespace::BeatmapEditorStartTestLevelData_FpfcData;
    #[cfg(feature = "BeatmapEditorStartTestLevelData+OverdrawData")]
    pub type OverdrawData = crate::GlobalNamespace::BeatmapEditorStartTestLevelData_OverdrawData;
}
#[cfg(feature = "BeatmapEditorStartTestLevelData+FpfcData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BeatmapEditorStartTestLevelData_FpfcData {
    pub enabled: bool,
}
#[cfg(feature = "BeatmapEditorStartTestLevelData+FpfcData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapEditorStartTestLevelData_FpfcData => ""
    ."BeatmapEditorStartTestLevelData/FpfcData"
);
#[cfg(feature = "BeatmapEditorStartTestLevelData+FpfcData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::BeatmapEditorStartTestLevelData_FpfcData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapEditorStartTestLevelData+FpfcData")]
impl crate::GlobalNamespace::BeatmapEditorStartTestLevelData_FpfcData {}
#[cfg(feature = "BeatmapEditorStartTestLevelData+OverdrawData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BeatmapEditorStartTestLevelData_OverdrawData {
    pub enabled: bool,
    pub computeBuffer: *mut crate::UnityEngine::ComputeBuffer,
    pub audioClipFrequency: i32,
    pub samplesPerOverdrawBucket: f32,
}
#[cfg(feature = "BeatmapEditorStartTestLevelData+OverdrawData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapEditorStartTestLevelData_OverdrawData => ""
    ."BeatmapEditorStartTestLevelData/OverdrawData"
);
#[cfg(feature = "BeatmapEditorStartTestLevelData+OverdrawData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::BeatmapEditorStartTestLevelData_OverdrawData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapEditorStartTestLevelData+OverdrawData")]
impl crate::GlobalNamespace::BeatmapEditorStartTestLevelData_OverdrawData {}
