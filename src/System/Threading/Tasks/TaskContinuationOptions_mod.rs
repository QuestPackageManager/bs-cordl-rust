#[cfg(feature = "cordl_class_System+Threading+Tasks+TaskContinuationOptions")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum TaskContinuationOptions {
    #[default]
    AttachedToParent = 4i32,
    DenyChildAttach = 8i32,
    ExecuteSynchronously = 524288i32,
    HideScheduler = 16i32,
    LazyCancellation = 32i32,
    LongRunning = 2i32,
    None = 0i32,
    NotOnCanceled = 262144i32,
    NotOnFaulted = 131072i32,
    NotOnRanToCompletion = 65536i32,
    OnlyOnCanceled = 196608i32,
    OnlyOnFaulted = 327680i32,
    OnlyOnRanToCompletion = 393216i32,
    PreferFairness = 1i32,
    RunContinuationsAsynchronously = 64i32,
}
#[cfg(feature = "cordl_class_System+Threading+Tasks+TaskContinuationOptions")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::System::Threading::Tasks::TaskContinuationOptions
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Threading.Tasks";
    const CLASS_NAME: &'static str = "TaskContinuationOptions";
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
#[cfg(feature = "cordl_class_System+Threading+Tasks+TaskContinuationOptions")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::System::Threading::Tasks::TaskContinuationOptions
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_System+Threading+Tasks+TaskContinuationOptions")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::System::Threading::Tasks::TaskContinuationOptions
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
#[cfg(feature = "cordl_class_System+Threading+Tasks+TaskContinuationOptions")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::System::Threading::Tasks::TaskContinuationOptions
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
#[cfg(feature = "cordl_class_System+Threading+Tasks+TaskContinuationOptions")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::System::Threading::Tasks::TaskContinuationOptions
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
