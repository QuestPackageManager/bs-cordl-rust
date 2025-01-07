#[cfg(feature = "HoudiniEngineUnity+HAPI_PartInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
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
unsafe impl quest_hook::libil2cpp::Type for crate::HoudiniEngineUnity::HAPI_PartInfo {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HAPI_PartInfo";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_PartInfo")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::HoudiniEngineUnity::HAPI_PartInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_PartInfo")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::HoudiniEngineUnity::HAPI_PartInfo {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_PartInfo")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::HoudiniEngineUnity::HAPI_PartInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_PartInfo")]
unsafe impl quest_hook::libil2cpp::Return for crate::HoudiniEngineUnity::HAPI_PartInfo {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
