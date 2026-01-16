#[cfg(feature = "cordl_class_UnityEngine+Rendering+LODGroupCullingData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LODGroupCullingData {
    pub worldSpaceReferencePoint: crate::Unity::Mathematics::float3,
    pub lodCount: i32,
    pub sqrDistances: crate::UnityEngine::Rendering::LODGroupCullingData__sqrDistances_e__FixedBuffer,
    pub transitionDistances: crate::UnityEngine::Rendering::LODGroupCullingData__transitionDistances_e__FixedBuffer,
    pub worldSpaceSize: f32,
    pub percentageFlags: crate::UnityEngine::Rendering::LODGroupCullingData__percentageFlags_e__FixedBuffer,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+LODGroupCullingData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::LODGroupCullingData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "LODGroupCullingData";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+LODGroupCullingData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::LODGroupCullingData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+LODGroupCullingData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::LODGroupCullingData {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+LODGroupCullingData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::LODGroupCullingData {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+LODGroupCullingData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::LODGroupCullingData {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+LODGroupCullingData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::LODGroupCullingData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+LODGroupCullingData")]
impl crate::UnityEngine::Rendering::LODGroupCullingData {
    #[cfg(
        feature = "UnityEngine+Rendering+LODGroupCullingData+_percentageFlags_e__FixedBuffer"
    )]
    pub type _percentageFlags_e__FixedBuffer = crate::UnityEngine::Rendering::LODGroupCullingData__percentageFlags_e__FixedBuffer;
    #[cfg(
        feature = "UnityEngine+Rendering+LODGroupCullingData+_sqrDistances_e__FixedBuffer"
    )]
    pub type _sqrDistances_e__FixedBuffer = crate::UnityEngine::Rendering::LODGroupCullingData__sqrDistances_e__FixedBuffer;
    #[cfg(
        feature = "UnityEngine+Rendering+LODGroupCullingData+_transitionDistances_e__FixedBuffer"
    )]
    pub type _transitionDistances_e__FixedBuffer = crate::UnityEngine::Rendering::LODGroupCullingData__transitionDistances_e__FixedBuffer;
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+LODGroupCullingData+_percentageFlags_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LODGroupCullingData__percentageFlags_e__FixedBuffer {
    pub FixedElementField: bool,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+LODGroupCullingData+_percentageFlags_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::LODGroupCullingData__percentageFlags_e__FixedBuffer {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "LODGroupCullingData/<percentageFlags>e__FixedBuffer";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+LODGroupCullingData+_percentageFlags_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::LODGroupCullingData__percentageFlags_e__FixedBuffer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+LODGroupCullingData+_percentageFlags_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::LODGroupCullingData__percentageFlags_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+LODGroupCullingData+_percentageFlags_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::LODGroupCullingData__percentageFlags_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+LODGroupCullingData+_percentageFlags_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::LODGroupCullingData__percentageFlags_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+LODGroupCullingData+_percentageFlags_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::LODGroupCullingData__percentageFlags_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+LODGroupCullingData+_percentageFlags_e__FixedBuffer"
)]
impl crate::UnityEngine::Rendering::LODGroupCullingData__percentageFlags_e__FixedBuffer {}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+LODGroupCullingData+_sqrDistances_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LODGroupCullingData__sqrDistances_e__FixedBuffer {
    pub FixedElementField: f32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+LODGroupCullingData+_sqrDistances_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::LODGroupCullingData__sqrDistances_e__FixedBuffer {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "LODGroupCullingData/<sqrDistances>e__FixedBuffer";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+LODGroupCullingData+_sqrDistances_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::LODGroupCullingData__sqrDistances_e__FixedBuffer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+LODGroupCullingData+_sqrDistances_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::LODGroupCullingData__sqrDistances_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+LODGroupCullingData+_sqrDistances_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::LODGroupCullingData__sqrDistances_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+LODGroupCullingData+_sqrDistances_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::LODGroupCullingData__sqrDistances_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+LODGroupCullingData+_sqrDistances_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::LODGroupCullingData__sqrDistances_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+LODGroupCullingData+_sqrDistances_e__FixedBuffer"
)]
impl crate::UnityEngine::Rendering::LODGroupCullingData__sqrDistances_e__FixedBuffer {}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+LODGroupCullingData+_transitionDistances_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LODGroupCullingData__transitionDistances_e__FixedBuffer {
    pub FixedElementField: f32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+LODGroupCullingData+_transitionDistances_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::LODGroupCullingData__transitionDistances_e__FixedBuffer {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "LODGroupCullingData/<transitionDistances>e__FixedBuffer";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+LODGroupCullingData+_transitionDistances_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::LODGroupCullingData__transitionDistances_e__FixedBuffer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+LODGroupCullingData+_transitionDistances_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::LODGroupCullingData__transitionDistances_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+LODGroupCullingData+_transitionDistances_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::LODGroupCullingData__transitionDistances_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+LODGroupCullingData+_transitionDistances_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::LODGroupCullingData__transitionDistances_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+LODGroupCullingData+_transitionDistances_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::LODGroupCullingData__transitionDistances_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+LODGroupCullingData+_transitionDistances_e__FixedBuffer"
)]
impl crate::UnityEngine::Rendering::LODGroupCullingData__transitionDistances_e__FixedBuffer {}
