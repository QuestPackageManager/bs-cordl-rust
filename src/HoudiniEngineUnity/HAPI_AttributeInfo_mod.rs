#[cfg(feature = "HoudiniEngineUnity+HAPI_AttributeInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HAPI_AttributeInfo {
    pub exists: bool,
    pub owner: crate::HoudiniEngineUnity::HAPI_AttributeOwner,
    pub storage: crate::HoudiniEngineUnity::HAPI_StorageType,
    pub originalOwner: crate::HoudiniEngineUnity::HAPI_AttributeOwner,
    pub count: i32,
    pub tupleSize: i32,
    pub totalArrayElements: i64,
    pub typeInfo: crate::HoudiniEngineUnity::HAPI_AttributeTypeInfo,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_AttributeInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_AttributeInfo =>
    "HoudiniEngineUnity"."HAPI_AttributeInfo"
);
#[cfg(feature = "HoudiniEngineUnity+HAPI_AttributeInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::HAPI_AttributeInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_AttributeInfo")]
impl crate::HoudiniEngineUnity::HAPI_AttributeInfo {
    pub fn _ctor(
        &mut self,
        ignored: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (ignored),
        )?;
        Ok(__cordl_ret)
    }
}
