#[cfg(feature = "cordl_class_HoudiniEngineUnity+HAPI_PrmScriptType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HAPI_PrmScriptType {
    #[default]
    HAPI_PRM_SCRIPT_TYPE_ANGLE = 2i32,
    HAPI_PRM_SCRIPT_TYPE_BUTTON = 9i32,
    HAPI_PRM_SCRIPT_TYPE_BUTTONSTRIP = 40i32,
    HAPI_PRM_SCRIPT_TYPE_COLOR = 19i32,
    HAPI_PRM_SCRIPT_TYPE_COLOR4 = 20i32,
    HAPI_PRM_SCRIPT_TYPE_DATA = 36i32,
    HAPI_PRM_SCRIPT_TYPE_DIR = 18i32,
    HAPI_PRM_SCRIPT_TYPE_DIRECTORY = 5i32,
    HAPI_PRM_SCRIPT_TYPE_FILE = 4i32,
    HAPI_PRM_SCRIPT_TYPE_FLOAT = 1i32,
    HAPI_PRM_SCRIPT_TYPE_FLOAT_LOG = 34i32,
    HAPI_PRM_SCRIPT_TYPE_FLOAT_MINMAX = 37i32,
    HAPI_PRM_SCRIPT_TYPE_GEOMETRY = 7i32,
    HAPI_PRM_SCRIPT_TYPE_GEOMETRY_DATA = 27i32,
    HAPI_PRM_SCRIPT_TYPE_GROUP = 1003i32,
    HAPI_PRM_SCRIPT_TYPE_GROUPCOLLAPSIBLE = 1001i32,
    HAPI_PRM_SCRIPT_TYPE_GROUPRADIO = 1000i32,
    HAPI_PRM_SCRIPT_TYPE_GROUPSIMPLE = 1002i32,
    HAPI_PRM_SCRIPT_TYPE_ICONSTRIP = 41i32,
    HAPI_PRM_SCRIPT_TYPE_IMAGE = 6i32,
    HAPI_PRM_SCRIPT_TYPE_INT = 0i32,
    HAPI_PRM_SCRIPT_TYPE_INTVECTOR2 = 13i32,
    HAPI_PRM_SCRIPT_TYPE_INTVECTOR3 = 14i32,
    HAPI_PRM_SCRIPT_TYPE_INTVECTOR4 = 15i32,
    HAPI_PRM_SCRIPT_TYPE_INT_LOG = 35i32,
    HAPI_PRM_SCRIPT_TYPE_INT_MINMAX = 38i32,
    HAPI_PRM_SCRIPT_TYPE_INT_STARTEND = 39i32,
    HAPI_PRM_SCRIPT_TYPE_KEY_VALUE_DICT = 28i32,
    HAPI_PRM_SCRIPT_TYPE_LABEL = 29i32,
    HAPI_PRM_SCRIPT_TYPE_OBJECT = 23i32,
    HAPI_PRM_SCRIPT_TYPE_OBJECTLIST = 24i32,
    HAPI_PRM_SCRIPT_TYPE_OPLIST = 22i32,
    HAPI_PRM_SCRIPT_TYPE_OPPATH = 21i32,
    HAPI_PRM_SCRIPT_TYPE_ORDINAL = 31i32,
    HAPI_PRM_SCRIPT_TYPE_RAMP_FLT = 32i32,
    HAPI_PRM_SCRIPT_TYPE_RAMP_RGB = 33i32,
    HAPI_PRM_SCRIPT_TYPE_RENDER = 25i32,
    HAPI_PRM_SCRIPT_TYPE_RGBAMASK = 30i32,
    HAPI_PRM_SCRIPT_TYPE_SEPARATOR = 26i32,
    HAPI_PRM_SCRIPT_TYPE_STRING = 3i32,
    HAPI_PRM_SCRIPT_TYPE_TOGGLE = 8i32,
    HAPI_PRM_SCRIPT_TYPE_UV = 16i32,
    HAPI_PRM_SCRIPT_TYPE_UVW = 17i32,
    HAPI_PRM_SCRIPT_TYPE_VECTOR2 = 10i32,
    HAPI_PRM_SCRIPT_TYPE_VECTOR3 = 11i32,
    HAPI_PRM_SCRIPT_TYPE_VECTOR4 = 12i32,
}
#[cfg(feature = "cordl_class_HoudiniEngineUnity+HAPI_PrmScriptType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::HoudiniEngineUnity::HAPI_PrmScriptType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HAPI_PrmScriptType";
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
#[cfg(feature = "cordl_class_HoudiniEngineUnity+HAPI_PrmScriptType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::HoudiniEngineUnity::HAPI_PrmScriptType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_HoudiniEngineUnity+HAPI_PrmScriptType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::HoudiniEngineUnity::HAPI_PrmScriptType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_HoudiniEngineUnity+HAPI_PrmScriptType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::HoudiniEngineUnity::HAPI_PrmScriptType {
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
#[cfg(feature = "cordl_class_HoudiniEngineUnity+HAPI_PrmScriptType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::HoudiniEngineUnity::HAPI_PrmScriptType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
