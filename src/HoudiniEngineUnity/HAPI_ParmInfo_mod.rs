#[cfg(feature = "HoudiniEngineUnity+HAPI_ParmInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct HAPI_ParmInfo {
    pub id: i32,
    pub parentId: i32,
    pub childIndex: i32,
    pub _cordl_type: crate::HoudiniEngineUnity::HAPI_ParmType,
    pub scriptType: crate::HoudiniEngineUnity::HAPI_PrmScriptType,
    pub typeInfoSH: i32,
    pub permissions: crate::HoudiniEngineUnity::HAPI_Permissions,
    pub tagCount: i32,
    pub _cordl_size: i32,
    pub choiceListType: crate::HoudiniEngineUnity::HAPI_ChoiceListType,
    pub choiceCount: i32,
    pub nameSH: i32,
    pub labelSH: i32,
    pub templateNameSH: i32,
    pub helpSH: i32,
    pub hasMin: bool,
    pub hasMax: bool,
    pub hasUIMin: bool,
    pub hasUIMax: bool,
    pub min: f32,
    pub max: f32,
    pub UIMin: f32,
    pub UIMax: f32,
    pub invisible: bool,
    pub disabled: bool,
    pub spare: bool,
    pub joinNext: bool,
    pub labelNone: bool,
    pub intValuesIndex: i32,
    pub floatValuesIndex: i32,
    pub stringValuesIndex: i32,
    pub choiceIndex: i32,
    pub inputNodeType: crate::HoudiniEngineUnity::HAPI_NodeType,
    pub inputNodeFlag: crate::HoudiniEngineUnity::HAPI_NodeFlags,
    pub isChildOfMultiParm: bool,
    pub instanceNum: i32,
    pub instanceLength: i32,
    pub instanceCount: i32,
    pub instanceStartOffset: i32,
    pub rampType: crate::HoudiniEngineUnity::HAPI_RampType,
    pub visibilityConditionSH: i32,
    pub disabledConditionSH: i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_ParmInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_ParmInfo =>
    "HoudiniEngineUnity"."HAPI_ParmInfo"
);
#[cfg(feature = "HoudiniEngineUnity+HAPI_ParmInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::HAPI_ParmInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_ParmInfo")]
impl crate::HoudiniEngineUnity::HAPI_ParmInfo {
    pub fn isFloat(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "isFloat",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn isInt(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "isInt",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn isNode(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "isNode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn isNonValue(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "isNonValue",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn isPath(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "isPath",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn isString(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "isString",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
