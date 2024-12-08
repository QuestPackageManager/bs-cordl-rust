#[cfg(feature = "HoudiniEngineUnity+HAPI_VolumeInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HAPI_VolumeInfo {
    pub nameSH: i32,
    pub _cordl_type: crate::HoudiniEngineUnity::HAPI_VolumeType,
    pub xLength: i32,
    pub yLength: i32,
    pub zLength: i32,
    pub minX: i32,
    pub minY: i32,
    pub minZ: i32,
    pub tupleSize: i32,
    pub storage: crate::HoudiniEngineUnity::HAPI_StorageType,
    pub tileSize: i32,
    pub transform: crate::HoudiniEngineUnity::HAPI_Transform,
    pub hasTaper: bool,
    pub xTaper: f32,
    pub yTaper: f32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_VolumeInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_VolumeInfo =>
    "HoudiniEngineUnity"."HAPI_VolumeInfo"
);
#[cfg(feature = "HoudiniEngineUnity+HAPI_VolumeInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::HAPI_VolumeInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_VolumeInfo")]
impl crate::HoudiniEngineUnity::HAPI_VolumeInfo {}
