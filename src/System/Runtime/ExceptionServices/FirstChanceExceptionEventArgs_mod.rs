#[cfg(feature = "cordl_class_System+Runtime+ExceptionServices+FirstChanceExceptionEventArgs")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct FirstChanceExceptionEventArgs {
    __cordl_parent: crate::System::EventArgs,
}
#[cfg(feature = "cordl_class_System+Runtime+ExceptionServices+FirstChanceExceptionEventArgs")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::System::Runtime::ExceptionServices::FirstChanceExceptionEventArgs
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.ExceptionServices";
    const CLASS_NAME: &'static str = "FirstChanceExceptionEventArgs";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Runtime+ExceptionServices+FirstChanceExceptionEventArgs")]
impl std::ops::Deref for crate::System::Runtime::ExceptionServices::FirstChanceExceptionEventArgs {
    type Target = crate::System::EventArgs;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+ExceptionServices+FirstChanceExceptionEventArgs")]
impl std::ops::DerefMut
    for crate::System::Runtime::ExceptionServices::FirstChanceExceptionEventArgs
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+ExceptionServices+FirstChanceExceptionEventArgs")]
impl crate::System::Runtime::ExceptionServices::FirstChanceExceptionEventArgs {}
#[cfg(feature = "cordl_class_System+Runtime+ExceptionServices+FirstChanceExceptionEventArgs")]
impl quest_hook::libil2cpp::ObjectType
    for crate::System::Runtime::ExceptionServices::FirstChanceExceptionEventArgs
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
