#[cfg(feature = "cordl_class_HoudiniEngineUnity+HAPI_ParmType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HAPI_ParmType {
    #[default]
    HAPI_PARMTYPE_BUTTON = 3i32,
    HAPI_PARMTYPE_COLOR = 5i32,
    HAPI_PARMTYPE_CONTAINER_END = 12i32,
    HAPI_PARMTYPE_CONTAINER_START = 11i32,
    HAPI_PARMTYPE_FLOAT = 4i32,
    HAPI_PARMTYPE_FOLDER = 13i32,
    HAPI_PARMTYPE_INT = 0i32,
    HAPI_PARMTYPE_LABEL = 14i32,
    HAPI_PARMTYPE_MAX = 17i32,
    HAPI_PARMTYPE_MULTIPARMLIST = 1i32,
    HAPI_PARMTYPE_NODE = 10i32,
    HAPI_PARMTYPE_NONVALUE_END = 15i32,
    HAPI_PARMTYPE_PATH_END = 9i32,
    HAPI_PARMTYPE_PATH_FILE = 7i32,
    HAPI_PARMTYPE_PATH_FILE_DIR = 16i32,
    HAPI_PARMTYPE_PATH_FILE_GEO = 8i32,
    HAPI_PARMTYPE_STRING = 6i32,
    HAPI_PARMTYPE_TOGGLE = 2i32,
}
#[cfg(feature = "cordl_class_HoudiniEngineUnity+HAPI_ParmType")]
unsafe impl quest_hook::libil2cpp::Type for crate::HoudiniEngineUnity::HAPI_ParmType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HAPI_ParmType";
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
#[cfg(feature = "cordl_class_HoudiniEngineUnity+HAPI_ParmType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::HoudiniEngineUnity::HAPI_ParmType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_HoudiniEngineUnity+HAPI_ParmType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::HoudiniEngineUnity::HAPI_ParmType {
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
#[cfg(feature = "cordl_class_HoudiniEngineUnity+HAPI_ParmType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::HoudiniEngineUnity::HAPI_ParmType {
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
#[cfg(feature = "cordl_class_HoudiniEngineUnity+HAPI_ParmType")]
unsafe impl quest_hook::libil2cpp::Return for crate::HoudiniEngineUnity::HAPI_ParmType {
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
