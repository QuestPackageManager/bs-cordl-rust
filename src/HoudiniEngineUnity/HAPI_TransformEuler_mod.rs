#[cfg(feature = "HoudiniEngineUnity+HAPI_TransformEuler")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HAPI_TransformEuler {
    pub position: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub rotationEuler: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub scale: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub shear: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub rotationOrder: crate::HoudiniEngineUnity::HAPI_XYZOrder,
    pub rstOrder: crate::HoudiniEngineUnity::HAPI_RSTOrder,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_TransformEuler")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_TransformEuler =>
    "HoudiniEngineUnity"."HAPI_TransformEuler"
);
#[cfg(feature = "HoudiniEngineUnity+HAPI_TransformEuler")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::HAPI_TransformEuler {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_TransformEuler")]
impl crate::HoudiniEngineUnity::HAPI_TransformEuler {
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Init",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        initializeFields: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (initializeFields),
        )?;
        Ok(__cordl_ret)
    }
}
