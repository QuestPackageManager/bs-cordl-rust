#[cfg(feature = "HoudiniEngineUnity+HAPI_NodeFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HAPI_NodeFlags {
    #[default]
    HAPI_NODEFLAGS_ANY = -1i32,
    HAPI_NODEFLAGS_BYPASS = 32i32,
    HAPI_NODEFLAGS_DISPLAY = 1i32,
    HAPI_NODEFLAGS_EDITABLE = 16i32,
    HAPI_NODEFLAGS_LOCKED = 8i32,
    HAPI_NODEFLAGS_NETWORK = 64i32,
    HAPI_NODEFLAGS_NONE = 0i32,
    HAPI_NODEFLAGS_NON_BYPASS = 16384i32,
    HAPI_NODEFLAGS_OBJ_CAMERA = 256i32,
    HAPI_NODEFLAGS_OBJ_GEOMETRY = 128i32,
    HAPI_NODEFLAGS_OBJ_LIGHT = 512i32,
    HAPI_NODEFLAGS_OBJ_SUBNET = 1024i32,
    HAPI_NODEFLAGS_RENDER = 2i32,
    HAPI_NODEFLAGS_SOP_CURVE = 2048i32,
    HAPI_NODEFLAGS_SOP_GUIDE = 4096i32,
    HAPI_NODEFLAGS_TEMPLATED = 4i32,
    HAPI_NODEFLAGS_TOP_NONSCHEDULER = 8192i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_NodeFlags")]
unsafe impl quest_hook::libil2cpp::Type for crate::HoudiniEngineUnity::HAPI_NodeFlags {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HAPI_NodeFlags";
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
unsafe impl quest_hook::libil2cpp::Argument
for crate::HoudiniEngineUnity::HAPI_NodeFlags {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::HoudiniEngineUnity::HAPI_NodeFlags {
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
unsafe impl quest_hook::libil2cpp::Returned
for crate::HoudiniEngineUnity::HAPI_NodeFlags {
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
unsafe impl quest_hook::libil2cpp::Return for crate::HoudiniEngineUnity::HAPI_NodeFlags {
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
