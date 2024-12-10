#[cfg(feature = "HoudiniEngineUnity+HAPI_SphereInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HAPI_SphereInfo {
    pub center: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub radius: f32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_SphereInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_SphereInfo =>
    "HoudiniEngineUnity"."HAPI_SphereInfo"
);
#[cfg(feature = "HoudiniEngineUnity+HAPI_SphereInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::HAPI_SphereInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_SphereInfo")]
impl crate::HoudiniEngineUnity::HAPI_SphereInfo {
    pub fn _ctor(
        &mut self,
        initialize_fields: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (initialize_fields),
        )?;
        Ok(__cordl_ret.into())
    }
}
