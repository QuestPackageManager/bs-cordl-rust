#[cfg(feature = "HoudiniEngineUnity+HAPI_PartType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HAPI_PartType {
    #[default]
    HAPI_PARTTYPE_BOX = 4i32,
    HAPI_PARTTYPE_CURVE = 1i32,
    HAPI_PARTTYPE_INSTANCER = 3i32,
    HAPI_PARTTYPE_INVALID = -1i32,
    HAPI_PARTTYPE_MAX = 6i32,
    HAPI_PARTTYPE_MESH = 0i32,
    HAPI_PARTTYPE_SPHERE = 5i32,
    HAPI_PARTTYPE_VOLUME = 2i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_PartType")]
unsafe impl quest_hook::libil2cpp::Type for crate::HoudiniEngineUnity::HAPI_PartType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HAPI_PartType";
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
#[cfg(feature = "HoudiniEngineUnity+HAPI_PartType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::HoudiniEngineUnity::HAPI_PartType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_PartType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::HoudiniEngineUnity::HAPI_PartType {
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
#[cfg(feature = "HoudiniEngineUnity+HAPI_PartType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::HoudiniEngineUnity::HAPI_PartType {
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
#[cfg(feature = "HoudiniEngineUnity+HAPI_PartType")]
unsafe impl quest_hook::libil2cpp::Return for crate::HoudiniEngineUnity::HAPI_PartType {
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
