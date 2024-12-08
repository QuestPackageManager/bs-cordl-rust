#[cfg(feature = "OVRDeserialize+DisplayRefreshRateChangedData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRDeserialize_DisplayRefreshRateChangedData {
    pub FromRefreshRate: f32,
    pub ToRefreshRate: f32,
}
#[cfg(feature = "OVRDeserialize+DisplayRefreshRateChangedData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRDeserialize_DisplayRefreshRateChangedData => ""
    ."OVRDeserialize/DisplayRefreshRateChangedData"
);
#[cfg(feature = "OVRDeserialize+DisplayRefreshRateChangedData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_DisplayRefreshRateChangedData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+DisplayRefreshRateChangedData")]
impl crate::GlobalNamespace::OVRDeserialize_DisplayRefreshRateChangedData {}
#[cfg(feature = "OVRDeserialize")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRDeserialize {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRDeserialize")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OVRDeserialize => ""."OVRDeserialize"
);
#[cfg(feature = "OVRDeserialize")]
impl std::ops::Deref for OVRDeserialize {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRDeserialize")]
impl std::ops::DerefMut for OVRDeserialize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRDeserialize")]
impl OVRDeserialize {
    #[cfg(feature = "OVRDeserialize+SpatialAnchorCreateCompleteData")]
    pub type SpatialAnchorCreateCompleteData = crate::GlobalNamespace::OVRDeserialize_SpatialAnchorCreateCompleteData;
    #[cfg(feature = "OVRDeserialize+SpaceQueryResultsData")]
    pub type SpaceQueryResultsData = crate::GlobalNamespace::OVRDeserialize_SpaceQueryResultsData;
    #[cfg(feature = "OVRDeserialize+SpaceQueryCompleteData")]
    pub type SpaceQueryCompleteData = crate::GlobalNamespace::OVRDeserialize_SpaceQueryCompleteData;
    #[cfg(feature = "OVRDeserialize+SpaceSaveCompleteData")]
    pub type SpaceSaveCompleteData = crate::GlobalNamespace::OVRDeserialize_SpaceSaveCompleteData;
    #[cfg(feature = "OVRDeserialize+SpaceShareResultData")]
    pub type SpaceShareResultData = crate::GlobalNamespace::OVRDeserialize_SpaceShareResultData;
    #[cfg(feature = "OVRDeserialize+SpaceSetComponentStatusCompleteData")]
    pub type SpaceSetComponentStatusCompleteData = crate::GlobalNamespace::OVRDeserialize_SpaceSetComponentStatusCompleteData;
    #[cfg(feature = "OVRDeserialize+DisplayRefreshRateChangedData")]
    pub type DisplayRefreshRateChangedData = crate::GlobalNamespace::OVRDeserialize_DisplayRefreshRateChangedData;
    #[cfg(feature = "OVRDeserialize+SceneCaptureCompleteData")]
    pub type SceneCaptureCompleteData = crate::GlobalNamespace::OVRDeserialize_SceneCaptureCompleteData;
    #[cfg(feature = "OVRDeserialize+SpaceListSaveResultData")]
    pub type SpaceListSaveResultData = crate::GlobalNamespace::OVRDeserialize_SpaceListSaveResultData;
    #[cfg(feature = "OVRDeserialize+SpaceEraseCompleteData")]
    pub type SpaceEraseCompleteData = crate::GlobalNamespace::OVRDeserialize_SpaceEraseCompleteData;
}
#[cfg(feature = "OVRDeserialize")]
impl quest_hook::libil2cpp::ObjectType for OVRDeserialize {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRDeserialize+SceneCaptureCompleteData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRDeserialize_SceneCaptureCompleteData {
    pub RequestId: u64,
    pub Result: i32,
}
#[cfg(feature = "OVRDeserialize+SceneCaptureCompleteData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRDeserialize_SceneCaptureCompleteData => ""
    ."OVRDeserialize/SceneCaptureCompleteData"
);
#[cfg(feature = "OVRDeserialize+SceneCaptureCompleteData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_SceneCaptureCompleteData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+SceneCaptureCompleteData")]
impl crate::GlobalNamespace::OVRDeserialize_SceneCaptureCompleteData {}
#[cfg(feature = "OVRDeserialize+SpaceEraseCompleteData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRDeserialize_SpaceEraseCompleteData {
    pub RequestId: u64,
    pub Result: i32,
    pub Uuid: crate::System::Guid,
    pub Location: crate::GlobalNamespace::OVRPlugin_SpaceStorageLocation,
}
#[cfg(feature = "OVRDeserialize+SpaceEraseCompleteData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRDeserialize_SpaceEraseCompleteData => ""
    ."OVRDeserialize/SpaceEraseCompleteData"
);
#[cfg(feature = "OVRDeserialize+SpaceEraseCompleteData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_SpaceEraseCompleteData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+SpaceEraseCompleteData")]
impl crate::GlobalNamespace::OVRDeserialize_SpaceEraseCompleteData {}
#[cfg(feature = "OVRDeserialize+SpaceListSaveResultData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRDeserialize_SpaceListSaveResultData {
    pub RequestId: u64,
    pub Result: i32,
}
#[cfg(feature = "OVRDeserialize+SpaceListSaveResultData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRDeserialize_SpaceListSaveResultData => ""
    ."OVRDeserialize/SpaceListSaveResultData"
);
#[cfg(feature = "OVRDeserialize+SpaceListSaveResultData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_SpaceListSaveResultData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+SpaceListSaveResultData")]
impl crate::GlobalNamespace::OVRDeserialize_SpaceListSaveResultData {}
#[cfg(feature = "OVRDeserialize+SpaceQueryCompleteData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRDeserialize_SpaceQueryCompleteData {
    pub RequestId: u64,
    pub Result: i32,
}
#[cfg(feature = "OVRDeserialize+SpaceQueryCompleteData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRDeserialize_SpaceQueryCompleteData => ""
    ."OVRDeserialize/SpaceQueryCompleteData"
);
#[cfg(feature = "OVRDeserialize+SpaceQueryCompleteData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_SpaceQueryCompleteData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+SpaceQueryCompleteData")]
impl crate::GlobalNamespace::OVRDeserialize_SpaceQueryCompleteData {}
#[cfg(feature = "OVRDeserialize+SpaceQueryResultsData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRDeserialize_SpaceQueryResultsData {
    pub RequestId: u64,
}
#[cfg(feature = "OVRDeserialize+SpaceQueryResultsData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRDeserialize_SpaceQueryResultsData => ""
    ."OVRDeserialize/SpaceQueryResultsData"
);
#[cfg(feature = "OVRDeserialize+SpaceQueryResultsData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_SpaceQueryResultsData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+SpaceQueryResultsData")]
impl crate::GlobalNamespace::OVRDeserialize_SpaceQueryResultsData {}
#[cfg(feature = "OVRDeserialize+SpaceSaveCompleteData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRDeserialize_SpaceSaveCompleteData {
    pub RequestId: u64,
    pub Space: u64,
    pub Result: i32,
    pub Uuid: crate::System::Guid,
}
#[cfg(feature = "OVRDeserialize+SpaceSaveCompleteData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRDeserialize_SpaceSaveCompleteData => ""
    ."OVRDeserialize/SpaceSaveCompleteData"
);
#[cfg(feature = "OVRDeserialize+SpaceSaveCompleteData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_SpaceSaveCompleteData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+SpaceSaveCompleteData")]
impl crate::GlobalNamespace::OVRDeserialize_SpaceSaveCompleteData {}
#[cfg(feature = "OVRDeserialize+SpaceSetComponentStatusCompleteData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRDeserialize_SpaceSetComponentStatusCompleteData {
    pub RequestId: u64,
    pub Result: i32,
    pub Space: u64,
    pub Uuid: crate::System::Guid,
    pub ComponentType: crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
    pub Enabled: i32,
}
#[cfg(feature = "OVRDeserialize+SpaceSetComponentStatusCompleteData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRDeserialize_SpaceSetComponentStatusCompleteData => ""
    ."OVRDeserialize/SpaceSetComponentStatusCompleteData"
);
#[cfg(feature = "OVRDeserialize+SpaceSetComponentStatusCompleteData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_SpaceSetComponentStatusCompleteData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+SpaceSetComponentStatusCompleteData")]
impl crate::GlobalNamespace::OVRDeserialize_SpaceSetComponentStatusCompleteData {}
#[cfg(feature = "OVRDeserialize+SpaceShareResultData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRDeserialize_SpaceShareResultData {
    pub RequestId: u64,
    pub Result: i32,
}
#[cfg(feature = "OVRDeserialize+SpaceShareResultData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRDeserialize_SpaceShareResultData => ""
    ."OVRDeserialize/SpaceShareResultData"
);
#[cfg(feature = "OVRDeserialize+SpaceShareResultData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_SpaceShareResultData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+SpaceShareResultData")]
impl crate::GlobalNamespace::OVRDeserialize_SpaceShareResultData {}
#[cfg(feature = "OVRDeserialize+SpatialAnchorCreateCompleteData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRDeserialize_SpatialAnchorCreateCompleteData {
    pub RequestId: u64,
    pub Result: i32,
    pub Space: u64,
    pub Uuid: crate::System::Guid,
}
#[cfg(feature = "OVRDeserialize+SpatialAnchorCreateCompleteData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRDeserialize_SpatialAnchorCreateCompleteData => ""
    ."OVRDeserialize/SpatialAnchorCreateCompleteData"
);
#[cfg(feature = "OVRDeserialize+SpatialAnchorCreateCompleteData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_SpatialAnchorCreateCompleteData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+SpatialAnchorCreateCompleteData")]
impl crate::GlobalNamespace::OVRDeserialize_SpatialAnchorCreateCompleteData {}
