#[cfg(feature = "HoudiniEngineUnity+HAPI_GeoInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HAPI_GeoInfo {
    pub _cordl_type: crate::HoudiniEngineUnity::HAPI_GeoType,
    pub nameSH: i32,
    pub nodeId: i32,
    pub isEditable: bool,
    pub isTemplated: bool,
    pub isDisplayGeo: bool,
    pub hasGeoChanged: bool,
    pub hasMaterialChanged: bool,
    pub pointGroupCount: i32,
    pub primitiveGroupCount: i32,
    pub edgeGroupCount: i32,
    pub partCount: i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_GeoInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_GeoInfo =>
    "HoudiniEngineUnity"."HAPI_GeoInfo"
);
#[cfg(feature = "HoudiniEngineUnity+HAPI_GeoInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::HAPI_GeoInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_GeoInfo")]
impl crate::HoudiniEngineUnity::HAPI_GeoInfo {
    pub fn getGroupCountByType(
        &mut self,
        _cordl_type: crate::HoudiniEngineUnity::HAPI_GroupType,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "getGroupCountByType",
            (_cordl_type),
        )?;
        Ok(__cordl_ret.into())
    }
}
