#[cfg(feature = "HoudiniEngineUnity+HAPI_Transform")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct HAPI_Transform {
    pub position: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    pub rotationQuaternion: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<f32>,
    >,
    pub scale: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    pub shear: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    pub rstOrder: crate::HoudiniEngineUnity::HAPI_RSTOrder,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_Transform")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_Transform =>
    "HoudiniEngineUnity"."HAPI_Transform"
);
#[cfg(feature = "HoudiniEngineUnity+HAPI_Transform")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::HAPI_Transform {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_Transform")]
impl crate::HoudiniEngineUnity::HAPI_Transform {
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Init",
            (),
        )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
}
