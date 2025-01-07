#[cfg(feature = "HoudiniEngineUnity+HAPI_TransformComponent")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HAPI_TransformComponent {
    #[default]
    HAPI_TRANSFORM_QW = 9i32,
    HAPI_TRANSFORM_QX = 6i32,
    HAPI_TRANSFORM_QY = 7i32,
    HAPI_TRANSFORM_QZ = 8i32,
    HAPI_TRANSFORM_RX = 3i32,
    HAPI_TRANSFORM_RY = 4i32,
    HAPI_TRANSFORM_RZ = 5i32,
    HAPI_TRANSFORM_SX = 10i32,
    HAPI_TRANSFORM_SY = 11i32,
    HAPI_TRANSFORM_SZ = 12i32,
    HAPI_TRANSFORM_TX = 0i32,
    HAPI_TRANSFORM_TY = 1i32,
    HAPI_TRANSFORM_TZ = 2i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_TransformComponent")]
unsafe impl quest_hook::libil2cpp::Type
for crate::HoudiniEngineUnity::HAPI_TransformComponent {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HAPI_TransformComponent";
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
for crate::HoudiniEngineUnity::HAPI_TransformComponent {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::HoudiniEngineUnity::HAPI_TransformComponent {
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
for crate::HoudiniEngineUnity::HAPI_TransformComponent {
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
unsafe impl quest_hook::libil2cpp::Return
for crate::HoudiniEngineUnity::HAPI_TransformComponent {
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
