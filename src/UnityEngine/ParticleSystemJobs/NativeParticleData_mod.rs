#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NativeParticleData {
    pub count: i32,
    pub positions: crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array3,
    pub velocities: crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array3,
    pub axisOfRotations: crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array3,
    pub rotations: crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array3,
    pub rotationalSpeeds: crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array3,
    pub sizes: crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array3,
    pub startColors: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub aliveTimePercent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub inverseStartLifetimes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppObject,
    >,
    pub randomSeeds: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub customData1: crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array4,
    pub customData2: crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array4,
    pub meshIndices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ParticleSystemJobs::NativeParticleData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.ParticleSystemJobs";
    const CLASS_NAME: &'static str = "NativeParticleData";
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
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::ParticleSystemJobs::NativeParticleData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::ParticleSystemJobs::NativeParticleData {
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
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::ParticleSystemJobs::NativeParticleData {
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
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::ParticleSystemJobs::NativeParticleData {
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
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystemJobs::NativeParticleData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData")]
impl crate::UnityEngine::ParticleSystemJobs::NativeParticleData {
    #[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData+Array3")]
    pub type Array3 = crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array3;
    #[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData+Array4")]
    pub type Array4 = crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array4;
}
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData+Array3")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NativeParticleData_Array3 {
    pub x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData+Array3")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array3 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.ParticleSystemJobs";
    const CLASS_NAME: &'static str = "NativeParticleData/Array3";
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
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData+Array3")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array3 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData+Array3")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array3 {
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
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData+Array3")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array3 {
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
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData+Array3")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array3 {
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
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData+Array3")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array3 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData+Array3")]
impl crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array3 {}
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData+Array4")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NativeParticleData_Array4 {
    pub x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub w: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData+Array4")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array4 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.ParticleSystemJobs";
    const CLASS_NAME: &'static str = "NativeParticleData/Array4";
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
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData+Array4")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array4 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData+Array4")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array4 {
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
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData+Array4")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array4 {
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
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData+Array4")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array4 {
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
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData+Array4")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array4 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData+Array4")]
impl crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array4 {}
