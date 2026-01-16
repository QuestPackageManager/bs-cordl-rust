#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct FixedUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::PlayerLoop::FixedUpdate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "FixedUpdate";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::PlayerLoop::FixedUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::PlayerLoop::FixedUpdate {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::PlayerLoop::FixedUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::PlayerLoop::FixedUpdate {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::PlayerLoop::FixedUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate")]
impl crate::UnityEngine::PlayerLoop::FixedUpdate {
    #[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+AudioFixedUpdate")]
    pub type AudioFixedUpdate = crate::UnityEngine::PlayerLoop::FixedUpdate_AudioFixedUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+ClearLines")]
    pub type ClearLines = crate::UnityEngine::PlayerLoop::FixedUpdate_ClearLines;
    #[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedSampleTime")]
    pub type DirectorFixedSampleTime =
        crate::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedSampleTime;
    #[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedUpdate")]
    pub type DirectorFixedUpdate = crate::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedUpdatePostPhysics")]
    pub type DirectorFixedUpdatePostPhysics =
        crate::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedUpdatePostPhysics;
    #[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+LegacyFixedAnimationUpdate")]
    pub type LegacyFixedAnimationUpdate =
        crate::UnityEngine::PlayerLoop::FixedUpdate_LegacyFixedAnimationUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+NewInputFixedUpdate")]
    pub type NewInputFixedUpdate = crate::UnityEngine::PlayerLoop::FixedUpdate_NewInputFixedUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+Physics2DFixedUpdate")]
    pub type Physics2DFixedUpdate =
        crate::UnityEngine::PlayerLoop::FixedUpdate_Physics2DFixedUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+PhysicsClothFixedUpdate")]
    pub type PhysicsClothFixedUpdate =
        crate::UnityEngine::PlayerLoop::FixedUpdate_PhysicsClothFixedUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+PhysicsFixedUpdate")]
    pub type PhysicsFixedUpdate = crate::UnityEngine::PlayerLoop::FixedUpdate_PhysicsFixedUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+ScriptRunBehaviourFixedUpdate")]
    pub type ScriptRunBehaviourFixedUpdate =
        crate::UnityEngine::PlayerLoop::FixedUpdate_ScriptRunBehaviourFixedUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+ScriptRunDelayedFixedFrameRate")]
    pub type ScriptRunDelayedFixedFrameRate =
        crate::UnityEngine::PlayerLoop::FixedUpdate_ScriptRunDelayedFixedFrameRate;
    #[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+XRFixedUpdate")]
    pub type XRFixedUpdate = crate::UnityEngine::PlayerLoop::FixedUpdate_XRFixedUpdate;
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+AudioFixedUpdate")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct FixedUpdate_AudioFixedUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+AudioFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::FixedUpdate_AudioFixedUpdate
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "FixedUpdate/AudioFixedUpdate";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+AudioFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::FixedUpdate_AudioFixedUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+AudioFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::FixedUpdate_AudioFixedUpdate
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+AudioFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::FixedUpdate_AudioFixedUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+AudioFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::FixedUpdate_AudioFixedUpdate
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+AudioFixedUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::FixedUpdate_AudioFixedUpdate
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+AudioFixedUpdate")]
impl crate::UnityEngine::PlayerLoop::FixedUpdate_AudioFixedUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+ClearLines")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct FixedUpdate_ClearLines {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+ClearLines")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::PlayerLoop::FixedUpdate_ClearLines {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "FixedUpdate/ClearLines";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+ClearLines")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::FixedUpdate_ClearLines
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+ClearLines")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::FixedUpdate_ClearLines
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+ClearLines")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::FixedUpdate_ClearLines
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+ClearLines")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::FixedUpdate_ClearLines
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+ClearLines")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::FixedUpdate_ClearLines
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+ClearLines")]
impl crate::UnityEngine::PlayerLoop::FixedUpdate_ClearLines {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedSampleTime")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct FixedUpdate_DirectorFixedSampleTime {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedSampleTime")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedSampleTime
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "FixedUpdate/DirectorFixedSampleTime";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedSampleTime")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedSampleTime
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedSampleTime")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedSampleTime
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedSampleTime")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedSampleTime
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedSampleTime")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedSampleTime
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedSampleTime")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedSampleTime
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedSampleTime")]
impl crate::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedSampleTime {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedUpdate")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct FixedUpdate_DirectorFixedUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedUpdate
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "FixedUpdate/DirectorFixedUpdate";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedUpdate
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedUpdate
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedUpdate
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedUpdate")]
impl crate::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedUpdatePostPhysics")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct FixedUpdate_DirectorFixedUpdatePostPhysics {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedUpdatePostPhysics")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedUpdatePostPhysics
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "FixedUpdate/DirectorFixedUpdatePostPhysics";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedUpdatePostPhysics")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedUpdatePostPhysics
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedUpdatePostPhysics")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedUpdatePostPhysics
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedUpdatePostPhysics")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedUpdatePostPhysics
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedUpdatePostPhysics")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedUpdatePostPhysics
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedUpdatePostPhysics")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedUpdatePostPhysics
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedUpdatePostPhysics")]
impl crate::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedUpdatePostPhysics {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+LegacyFixedAnimationUpdate")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct FixedUpdate_LegacyFixedAnimationUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+LegacyFixedAnimationUpdate")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::FixedUpdate_LegacyFixedAnimationUpdate
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "FixedUpdate/LegacyFixedAnimationUpdate";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+LegacyFixedAnimationUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::FixedUpdate_LegacyFixedAnimationUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+LegacyFixedAnimationUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::FixedUpdate_LegacyFixedAnimationUpdate
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+LegacyFixedAnimationUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::FixedUpdate_LegacyFixedAnimationUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+LegacyFixedAnimationUpdate")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::FixedUpdate_LegacyFixedAnimationUpdate
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+LegacyFixedAnimationUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::FixedUpdate_LegacyFixedAnimationUpdate
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+LegacyFixedAnimationUpdate")]
impl crate::UnityEngine::PlayerLoop::FixedUpdate_LegacyFixedAnimationUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+NewInputFixedUpdate")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct FixedUpdate_NewInputFixedUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+NewInputFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::FixedUpdate_NewInputFixedUpdate
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "FixedUpdate/NewInputFixedUpdate";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+NewInputFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::FixedUpdate_NewInputFixedUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+NewInputFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::FixedUpdate_NewInputFixedUpdate
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+NewInputFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::FixedUpdate_NewInputFixedUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+NewInputFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::FixedUpdate_NewInputFixedUpdate
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+NewInputFixedUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::FixedUpdate_NewInputFixedUpdate
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+NewInputFixedUpdate")]
impl crate::UnityEngine::PlayerLoop::FixedUpdate_NewInputFixedUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+Physics2DFixedUpdate")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct FixedUpdate_Physics2DFixedUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+Physics2DFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::FixedUpdate_Physics2DFixedUpdate
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "FixedUpdate/Physics2DFixedUpdate";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+Physics2DFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::FixedUpdate_Physics2DFixedUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+Physics2DFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::FixedUpdate_Physics2DFixedUpdate
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+Physics2DFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::FixedUpdate_Physics2DFixedUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+Physics2DFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::FixedUpdate_Physics2DFixedUpdate
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+Physics2DFixedUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::FixedUpdate_Physics2DFixedUpdate
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+Physics2DFixedUpdate")]
impl crate::UnityEngine::PlayerLoop::FixedUpdate_Physics2DFixedUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+PhysicsClothFixedUpdate")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct FixedUpdate_PhysicsClothFixedUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+PhysicsClothFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::FixedUpdate_PhysicsClothFixedUpdate
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "FixedUpdate/PhysicsClothFixedUpdate";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+PhysicsClothFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::FixedUpdate_PhysicsClothFixedUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+PhysicsClothFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::FixedUpdate_PhysicsClothFixedUpdate
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+PhysicsClothFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::FixedUpdate_PhysicsClothFixedUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+PhysicsClothFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::FixedUpdate_PhysicsClothFixedUpdate
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+PhysicsClothFixedUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::FixedUpdate_PhysicsClothFixedUpdate
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+PhysicsClothFixedUpdate")]
impl crate::UnityEngine::PlayerLoop::FixedUpdate_PhysicsClothFixedUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+PhysicsFixedUpdate")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct FixedUpdate_PhysicsFixedUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+PhysicsFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::FixedUpdate_PhysicsFixedUpdate
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "FixedUpdate/PhysicsFixedUpdate";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+PhysicsFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::FixedUpdate_PhysicsFixedUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+PhysicsFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::FixedUpdate_PhysicsFixedUpdate
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+PhysicsFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::FixedUpdate_PhysicsFixedUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+PhysicsFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::FixedUpdate_PhysicsFixedUpdate
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+PhysicsFixedUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::FixedUpdate_PhysicsFixedUpdate
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+PhysicsFixedUpdate")]
impl crate::UnityEngine::PlayerLoop::FixedUpdate_PhysicsFixedUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+ScriptRunBehaviourFixedUpdate")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct FixedUpdate_ScriptRunBehaviourFixedUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+ScriptRunBehaviourFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::FixedUpdate_ScriptRunBehaviourFixedUpdate
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "FixedUpdate/ScriptRunBehaviourFixedUpdate";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+ScriptRunBehaviourFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::FixedUpdate_ScriptRunBehaviourFixedUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+ScriptRunBehaviourFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::FixedUpdate_ScriptRunBehaviourFixedUpdate
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+ScriptRunBehaviourFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::FixedUpdate_ScriptRunBehaviourFixedUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+ScriptRunBehaviourFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::FixedUpdate_ScriptRunBehaviourFixedUpdate
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+ScriptRunBehaviourFixedUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::FixedUpdate_ScriptRunBehaviourFixedUpdate
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+ScriptRunBehaviourFixedUpdate")]
impl crate::UnityEngine::PlayerLoop::FixedUpdate_ScriptRunBehaviourFixedUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+ScriptRunDelayedFixedFrameRate")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct FixedUpdate_ScriptRunDelayedFixedFrameRate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+ScriptRunDelayedFixedFrameRate")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::FixedUpdate_ScriptRunDelayedFixedFrameRate
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "FixedUpdate/ScriptRunDelayedFixedFrameRate";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+ScriptRunDelayedFixedFrameRate")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::FixedUpdate_ScriptRunDelayedFixedFrameRate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+ScriptRunDelayedFixedFrameRate")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::FixedUpdate_ScriptRunDelayedFixedFrameRate
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+ScriptRunDelayedFixedFrameRate")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::FixedUpdate_ScriptRunDelayedFixedFrameRate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+ScriptRunDelayedFixedFrameRate")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::FixedUpdate_ScriptRunDelayedFixedFrameRate
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+ScriptRunDelayedFixedFrameRate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::FixedUpdate_ScriptRunDelayedFixedFrameRate
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+ScriptRunDelayedFixedFrameRate")]
impl crate::UnityEngine::PlayerLoop::FixedUpdate_ScriptRunDelayedFixedFrameRate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+XRFixedUpdate")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct FixedUpdate_XRFixedUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+XRFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::FixedUpdate_XRFixedUpdate
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "FixedUpdate/XRFixedUpdate";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+XRFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::FixedUpdate_XRFixedUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+XRFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::FixedUpdate_XRFixedUpdate
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+XRFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::FixedUpdate_XRFixedUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+XRFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::FixedUpdate_XRFixedUpdate
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+FixedUpdate+XRFixedUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::FixedUpdate_XRFixedUpdate
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+XRFixedUpdate")]
impl crate::UnityEngine::PlayerLoop::FixedUpdate_XRFixedUpdate {}
