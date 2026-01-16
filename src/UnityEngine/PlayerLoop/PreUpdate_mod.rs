#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PreUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::PlayerLoop::PreUpdate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PreUpdate";
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PreUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PreUpdate {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PreUpdate {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::PlayerLoop::PreUpdate {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PreUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate")]
impl crate::UnityEngine::PlayerLoop::PreUpdate {
    #[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+AIUpdate")]
    pub type AIUpdate = crate::UnityEngine::PlayerLoop::PreUpdate_AIUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+CheckTexFieldInput")]
    pub type CheckTexFieldInput = crate::UnityEngine::PlayerLoop::PreUpdate_CheckTexFieldInput;
    #[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+IMGUISendQueuedEvents")]
    pub type IMGUISendQueuedEvents = crate::UnityEngine::PlayerLoop::PreUpdate_IMGUISendQueuedEvents;
    #[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+InputForUIUpdate")]
    pub type InputForUIUpdate = crate::UnityEngine::PlayerLoop::PreUpdate_InputForUIUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+NewInputUpdate")]
    pub type NewInputUpdate = crate::UnityEngine::PlayerLoop::PreUpdate_NewInputUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+Physics2DUpdate")]
    pub type Physics2DUpdate = crate::UnityEngine::PlayerLoop::PreUpdate_Physics2DUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+PhysicsClothUpdate")]
    pub type PhysicsClothUpdate = crate::UnityEngine::PlayerLoop::PreUpdate_PhysicsClothUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+PhysicsUpdate")]
    pub type PhysicsUpdate = crate::UnityEngine::PlayerLoop::PreUpdate_PhysicsUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+SendMouseEvents")]
    pub type SendMouseEvents = crate::UnityEngine::PlayerLoop::PreUpdate_SendMouseEvents;
    #[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+UpdateVideo")]
    pub type UpdateVideo = crate::UnityEngine::PlayerLoop::PreUpdate_UpdateVideo;
    #[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+WindUpdate")]
    pub type WindUpdate = crate::UnityEngine::PlayerLoop::PreUpdate_WindUpdate;
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+AIUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PreUpdate_AIUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+AIUpdate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PreUpdate_AIUpdate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PreUpdate/AIUpdate";
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+AIUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PreUpdate_AIUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+AIUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PreUpdate_AIUpdate {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+AIUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PreUpdate_AIUpdate {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+AIUpdate")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PreUpdate_AIUpdate {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+AIUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PreUpdate_AIUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+AIUpdate")]
impl crate::UnityEngine::PlayerLoop::PreUpdate_AIUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+CheckTexFieldInput")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PreUpdate_CheckTexFieldInput {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+CheckTexFieldInput")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PreUpdate_CheckTexFieldInput {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PreUpdate/CheckTexFieldInput";
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+CheckTexFieldInput")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PreUpdate_CheckTexFieldInput {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+CheckTexFieldInput")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PreUpdate_CheckTexFieldInput {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+CheckTexFieldInput")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PreUpdate_CheckTexFieldInput {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+CheckTexFieldInput")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PreUpdate_CheckTexFieldInput {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+CheckTexFieldInput")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PreUpdate_CheckTexFieldInput {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+CheckTexFieldInput")]
impl crate::UnityEngine::PlayerLoop::PreUpdate_CheckTexFieldInput {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+IMGUISendQueuedEvents")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PreUpdate_IMGUISendQueuedEvents {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+IMGUISendQueuedEvents")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PreUpdate_IMGUISendQueuedEvents {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PreUpdate/IMGUISendQueuedEvents";
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+IMGUISendQueuedEvents")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PreUpdate_IMGUISendQueuedEvents {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+IMGUISendQueuedEvents")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PreUpdate_IMGUISendQueuedEvents {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+IMGUISendQueuedEvents")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PreUpdate_IMGUISendQueuedEvents {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+IMGUISendQueuedEvents")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PreUpdate_IMGUISendQueuedEvents {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+IMGUISendQueuedEvents")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PreUpdate_IMGUISendQueuedEvents {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+IMGUISendQueuedEvents")]
impl crate::UnityEngine::PlayerLoop::PreUpdate_IMGUISendQueuedEvents {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+InputForUIUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PreUpdate_InputForUIUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+InputForUIUpdate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PreUpdate_InputForUIUpdate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PreUpdate/InputForUIUpdate";
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+InputForUIUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PreUpdate_InputForUIUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+InputForUIUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PreUpdate_InputForUIUpdate {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+InputForUIUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PreUpdate_InputForUIUpdate {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+InputForUIUpdate")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PreUpdate_InputForUIUpdate {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+InputForUIUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PreUpdate_InputForUIUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+InputForUIUpdate")]
impl crate::UnityEngine::PlayerLoop::PreUpdate_InputForUIUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+NewInputUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PreUpdate_NewInputUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+NewInputUpdate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PreUpdate_NewInputUpdate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PreUpdate/NewInputUpdate";
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+NewInputUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PreUpdate_NewInputUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+NewInputUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PreUpdate_NewInputUpdate {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+NewInputUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PreUpdate_NewInputUpdate {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+NewInputUpdate")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PreUpdate_NewInputUpdate {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+NewInputUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PreUpdate_NewInputUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+NewInputUpdate")]
impl crate::UnityEngine::PlayerLoop::PreUpdate_NewInputUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+Physics2DUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PreUpdate_Physics2DUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+Physics2DUpdate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PreUpdate_Physics2DUpdate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PreUpdate/Physics2DUpdate";
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+Physics2DUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PreUpdate_Physics2DUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+Physics2DUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PreUpdate_Physics2DUpdate {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+Physics2DUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PreUpdate_Physics2DUpdate {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+Physics2DUpdate")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PreUpdate_Physics2DUpdate {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+Physics2DUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PreUpdate_Physics2DUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+Physics2DUpdate")]
impl crate::UnityEngine::PlayerLoop::PreUpdate_Physics2DUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+PhysicsClothUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PreUpdate_PhysicsClothUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+PhysicsClothUpdate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PreUpdate_PhysicsClothUpdate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PreUpdate/PhysicsClothUpdate";
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+PhysicsClothUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PreUpdate_PhysicsClothUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+PhysicsClothUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PreUpdate_PhysicsClothUpdate {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+PhysicsClothUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PreUpdate_PhysicsClothUpdate {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+PhysicsClothUpdate")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PreUpdate_PhysicsClothUpdate {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+PhysicsClothUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PreUpdate_PhysicsClothUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+PhysicsClothUpdate")]
impl crate::UnityEngine::PlayerLoop::PreUpdate_PhysicsClothUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+PhysicsUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PreUpdate_PhysicsUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+PhysicsUpdate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PreUpdate_PhysicsUpdate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PreUpdate/PhysicsUpdate";
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+PhysicsUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PreUpdate_PhysicsUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+PhysicsUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PreUpdate_PhysicsUpdate {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+PhysicsUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PreUpdate_PhysicsUpdate {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+PhysicsUpdate")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PreUpdate_PhysicsUpdate {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+PhysicsUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PreUpdate_PhysicsUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+PhysicsUpdate")]
impl crate::UnityEngine::PlayerLoop::PreUpdate_PhysicsUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+SendMouseEvents")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PreUpdate_SendMouseEvents {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+SendMouseEvents")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PreUpdate_SendMouseEvents {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PreUpdate/SendMouseEvents";
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+SendMouseEvents")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PreUpdate_SendMouseEvents {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+SendMouseEvents")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PreUpdate_SendMouseEvents {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+SendMouseEvents")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PreUpdate_SendMouseEvents {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+SendMouseEvents")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PreUpdate_SendMouseEvents {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+SendMouseEvents")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PreUpdate_SendMouseEvents {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+SendMouseEvents")]
impl crate::UnityEngine::PlayerLoop::PreUpdate_SendMouseEvents {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+UpdateVideo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PreUpdate_UpdateVideo {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+UpdateVideo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PreUpdate_UpdateVideo {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PreUpdate/UpdateVideo";
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+UpdateVideo")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PreUpdate_UpdateVideo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+UpdateVideo")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PreUpdate_UpdateVideo {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+UpdateVideo")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PreUpdate_UpdateVideo {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+UpdateVideo")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PreUpdate_UpdateVideo {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+UpdateVideo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PreUpdate_UpdateVideo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+UpdateVideo")]
impl crate::UnityEngine::PlayerLoop::PreUpdate_UpdateVideo {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+WindUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PreUpdate_WindUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+WindUpdate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PreUpdate_WindUpdate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PreUpdate/WindUpdate";
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+WindUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PreUpdate_WindUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+WindUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PreUpdate_WindUpdate {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+WindUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PreUpdate_WindUpdate {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+WindUpdate")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PreUpdate_WindUpdate {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PreUpdate+WindUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PreUpdate_WindUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+WindUpdate")]
impl crate::UnityEngine::PlayerLoop::PreUpdate_WindUpdate {}
