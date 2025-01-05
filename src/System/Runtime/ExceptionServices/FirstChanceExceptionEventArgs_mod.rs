#[cfg(feature = "System+Runtime+ExceptionServices+FirstChanceExceptionEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct FirstChanceExceptionEventArgs {
    __cordl_parent: crate::System::EventArgs,
}
#[cfg(feature = "System+Runtime+ExceptionServices+FirstChanceExceptionEventArgs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::ExceptionServices::FirstChanceExceptionEventArgs =>
    "System.Runtime.ExceptionServices"."FirstChanceExceptionEventArgs"
);
#[cfg(feature = "System+Runtime+ExceptionServices+FirstChanceExceptionEventArgs")]
impl std::ops::Deref
for crate::System::Runtime::ExceptionServices::FirstChanceExceptionEventArgs {
    type Target = crate::System::EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+ExceptionServices+FirstChanceExceptionEventArgs")]
impl std::ops::DerefMut
for crate::System::Runtime::ExceptionServices::FirstChanceExceptionEventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+ExceptionServices+FirstChanceExceptionEventArgs")]
impl crate::System::Runtime::ExceptionServices::FirstChanceExceptionEventArgs {}
#[cfg(feature = "System+Runtime+ExceptionServices+FirstChanceExceptionEventArgs")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::ExceptionServices::FirstChanceExceptionEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
