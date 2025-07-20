#[cfg(feature = "HoudiniEngineUnity+HAPI_GeoInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
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
unsafe impl quest_hook::libil2cpp::Type for crate::HoudiniEngineUnity::HAPI_GeoInfo {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HAPI_GeoInfo";
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
#[cfg(feature = "HoudiniEngineUnity+HAPI_GeoInfo")]
unsafe impl quest_hook::libil2cpp::Argument for crate::HoudiniEngineUnity::HAPI_GeoInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_GeoInfo")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::HoudiniEngineUnity::HAPI_GeoInfo {
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
#[cfg(feature = "HoudiniEngineUnity+HAPI_GeoInfo")]
unsafe impl quest_hook::libil2cpp::Returned for crate::HoudiniEngineUnity::HAPI_GeoInfo {
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
#[cfg(feature = "HoudiniEngineUnity+HAPI_GeoInfo")]
unsafe impl quest_hook::libil2cpp::Return for crate::HoudiniEngineUnity::HAPI_GeoInfo {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::HoudiniEngineUnity::HAPI_GroupType),
                        i32,
                        1usize,
                    >("getGroupCountByType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "getGroupCountByType", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
}
