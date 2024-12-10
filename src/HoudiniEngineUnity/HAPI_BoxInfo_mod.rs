#[cfg(feature = "HoudiniEngineUnity+HAPI_BoxInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HAPI_BoxInfo {
    pub center: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub _cordl_size: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub rotation: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_BoxInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_BoxInfo =>
    "HoudiniEngineUnity"."HAPI_BoxInfo"
);
#[cfg(feature = "HoudiniEngineUnity+HAPI_BoxInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::HAPI_BoxInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_BoxInfo")]
impl crate::HoudiniEngineUnity::HAPI_BoxInfo {
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
