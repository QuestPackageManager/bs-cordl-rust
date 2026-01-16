#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct Update {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::PlayerLoop::Update {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "Update";
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::PlayerLoop::Update {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::PlayerLoop::Update {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::PlayerLoop::Update {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::PlayerLoop::Update {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::PlayerLoop::Update {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+Update")]
impl crate::UnityEngine::PlayerLoop::Update {
    #[cfg(feature = "UnityEngine+PlayerLoop+Update+DirectorUpdate")]
    pub type DirectorUpdate = crate::UnityEngine::PlayerLoop::Update_DirectorUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+Update+ScriptRunBehaviourUpdate")]
    pub type ScriptRunBehaviourUpdate =
        crate::UnityEngine::PlayerLoop::Update_ScriptRunBehaviourUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+Update+ScriptRunDelayedDynamicFrameRate")]
    pub type ScriptRunDelayedDynamicFrameRate =
        crate::UnityEngine::PlayerLoop::Update_ScriptRunDelayedDynamicFrameRate;
    #[cfg(feature = "UnityEngine+PlayerLoop+Update+ScriptRunDelayedTasks")]
    pub type ScriptRunDelayedTasks = crate::UnityEngine::PlayerLoop::Update_ScriptRunDelayedTasks;
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update+DirectorUpdate")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct Update_DirectorUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update+DirectorUpdate")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::PlayerLoop::Update_DirectorUpdate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "Update/DirectorUpdate";
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update+DirectorUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::Update_DirectorUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update+DirectorUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::Update_DirectorUpdate
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update+DirectorUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::Update_DirectorUpdate
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update+DirectorUpdate")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::Update_DirectorUpdate
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update+DirectorUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::Update_DirectorUpdate
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+Update+DirectorUpdate")]
impl crate::UnityEngine::PlayerLoop::Update_DirectorUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update+ScriptRunBehaviourUpdate")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct Update_ScriptRunBehaviourUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update+ScriptRunBehaviourUpdate")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::Update_ScriptRunBehaviourUpdate
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "Update/ScriptRunBehaviourUpdate";
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update+ScriptRunBehaviourUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::Update_ScriptRunBehaviourUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update+ScriptRunBehaviourUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::Update_ScriptRunBehaviourUpdate
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update+ScriptRunBehaviourUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::Update_ScriptRunBehaviourUpdate
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update+ScriptRunBehaviourUpdate")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::Update_ScriptRunBehaviourUpdate
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update+ScriptRunBehaviourUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::Update_ScriptRunBehaviourUpdate
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+Update+ScriptRunBehaviourUpdate")]
impl crate::UnityEngine::PlayerLoop::Update_ScriptRunBehaviourUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update+ScriptRunDelayedDynamicFrameRate")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct Update_ScriptRunDelayedDynamicFrameRate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update+ScriptRunDelayedDynamicFrameRate")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::Update_ScriptRunDelayedDynamicFrameRate
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "Update/ScriptRunDelayedDynamicFrameRate";
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update+ScriptRunDelayedDynamicFrameRate")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::Update_ScriptRunDelayedDynamicFrameRate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update+ScriptRunDelayedDynamicFrameRate")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::Update_ScriptRunDelayedDynamicFrameRate
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update+ScriptRunDelayedDynamicFrameRate")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::Update_ScriptRunDelayedDynamicFrameRate
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update+ScriptRunDelayedDynamicFrameRate")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::Update_ScriptRunDelayedDynamicFrameRate
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update+ScriptRunDelayedDynamicFrameRate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::Update_ScriptRunDelayedDynamicFrameRate
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+Update+ScriptRunDelayedDynamicFrameRate")]
impl crate::UnityEngine::PlayerLoop::Update_ScriptRunDelayedDynamicFrameRate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update+ScriptRunDelayedTasks")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct Update_ScriptRunDelayedTasks {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update+ScriptRunDelayedTasks")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::Update_ScriptRunDelayedTasks
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "Update/ScriptRunDelayedTasks";
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update+ScriptRunDelayedTasks")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::Update_ScriptRunDelayedTasks
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update+ScriptRunDelayedTasks")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::Update_ScriptRunDelayedTasks
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update+ScriptRunDelayedTasks")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::Update_ScriptRunDelayedTasks
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update+ScriptRunDelayedTasks")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::Update_ScriptRunDelayedTasks
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+Update+ScriptRunDelayedTasks")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::Update_ScriptRunDelayedTasks
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+Update+ScriptRunDelayedTasks")]
impl crate::UnityEngine::PlayerLoop::Update_ScriptRunDelayedTasks {}
