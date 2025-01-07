#[cfg(feature = "HoudiniEngineUnity+HAPI_Result")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HAPI_Result {
    #[default]
    HAPI_RESULT_ALREADY_INITIALIZED = 2i32,
    HAPI_RESULT_ASSET_DEF_ALREADY_LOADED = 10i32,
    HAPI_RESULT_ASSET_INVALID = 200i32,
    HAPI_RESULT_CANT_GENERATE_PRESET = 8i32,
    HAPI_RESULT_CANT_LOADFILE = 4i32,
    HAPI_RESULT_CANT_LOAD_GEO = 7i32,
    HAPI_RESULT_CANT_LOAD_PRESET = 9i32,
    HAPI_RESULT_DISALLOWED_HENGINEINDIE_W_3PARTY_PLUGIN = 160i32,
    HAPI_RESULT_DISALLOWED_LC_ASSET_WITH_C_LICENSE = 150i32,
    HAPI_RESULT_DISALLOWED_NC_ASSET_WITH_C_LICENSE = 130i32,
    HAPI_RESULT_DISALLOWED_NC_ASSET_WITH_LC_LICENSE = 140i32,
    HAPI_RESULT_DISALLOWED_NC_LICENSE_FOUND = 120i32,
    HAPI_RESULT_FAILURE = 1i32,
    HAPI_RESULT_INVALID_ARGUMENT = 6i32,
    HAPI_RESULT_INVALID_SESSION = 400i32,
    HAPI_RESULT_NODE_INVALID = 210i32,
    HAPI_RESULT_NOT_INITIALIZED = 3i32,
    HAPI_RESULT_NO_LICENSE_FOUND = 110i32,
    HAPI_RESULT_PARM_SET_FAILED = 5i32,
    HAPI_RESULT_SUCCESS = 0i32,
    HAPI_RESULT_USER_INTERRUPTED = 300i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_Result")]
unsafe impl quest_hook::libil2cpp::Type for crate::HoudiniEngineUnity::HAPI_Result {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HAPI_Result";
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
#[cfg(feature = "HoudiniEngineUnity+HAPI_Result")]
unsafe impl quest_hook::libil2cpp::Argument for crate::HoudiniEngineUnity::HAPI_Result {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_Result")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::HoudiniEngineUnity::HAPI_Result {
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
#[cfg(feature = "HoudiniEngineUnity+HAPI_Result")]
unsafe impl quest_hook::libil2cpp::Returned for crate::HoudiniEngineUnity::HAPI_Result {
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
#[cfg(feature = "HoudiniEngineUnity+HAPI_Result")]
unsafe impl quest_hook::libil2cpp::Return for crate::HoudiniEngineUnity::HAPI_Result {
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
