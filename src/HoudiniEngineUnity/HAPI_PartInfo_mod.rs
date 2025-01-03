#[cfg(feature = "HoudiniEngineUnity+HAPI_PartInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct HAPI_PartInfo {
    pub id: i32,
    pub nameSH: i32,
    pub _cordl_type: crate::HoudiniEngineUnity::HAPI_PartType,
    pub faceCount: i32,
    pub vertexCount: i32,
    pub pointCount: i32,
    pub attributeCounts: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
    pub isInstanced: bool,
    pub instancedPartCount: i32,
    pub instanceCount: i32,
    pub hasChanged: bool,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_PartInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_PartInfo =>
    "HoudiniEngineUnity"."HAPI_PartInfo"
);
#[cfg(feature = "HoudiniEngineUnity+HAPI_PartInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::HAPI_PartInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_PartInfo")]
impl crate::HoudiniEngineUnity::HAPI_PartInfo {
    pub fn getElementCountByAttributeOwner(
        &mut self,
        owner: crate::HoudiniEngineUnity::HAPI_AttributeOwner,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "getElementCountByAttributeOwner",
            (owner),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn getElementCountByGroupType(
        &mut self,
        _cordl_type: crate::HoudiniEngineUnity::HAPI_GroupType,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "getElementCountByGroupType",
            (_cordl_type),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_detailAttributeCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_detailAttributeCount",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pointAttributeCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_pointAttributeCount",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_primitiveAttributeCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_primitiveAttributeCount",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_vertexAttributeCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_vertexAttributeCount",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "init",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_detailAttributeCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_detailAttributeCount",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_pointAttributeCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_pointAttributeCount",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_primitiveAttributeCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_primitiveAttributeCount",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_vertexAttributeCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_vertexAttributeCount",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
