#[cfg(feature = "System+Threading+Tasks+UnobservedTaskExceptionEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct UnobservedTaskExceptionEventArgs {
    __cordl_parent: crate::System::EventArgs,
    pub m_exception: *mut crate::System::AggregateException,
    pub m_observed: bool,
}
#[cfg(feature = "System+Threading+Tasks+UnobservedTaskExceptionEventArgs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::Tasks::UnobservedTaskExceptionEventArgs =>
    "System.Threading.Tasks"."UnobservedTaskExceptionEventArgs"
);
#[cfg(feature = "System+Threading+Tasks+UnobservedTaskExceptionEventArgs")]
impl std::ops::Deref
for crate::System::Threading::Tasks::UnobservedTaskExceptionEventArgs {
    type Target = crate::System::EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+UnobservedTaskExceptionEventArgs")]
impl std::ops::DerefMut
for crate::System::Threading::Tasks::UnobservedTaskExceptionEventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+UnobservedTaskExceptionEventArgs")]
impl crate::System::Threading::Tasks::UnobservedTaskExceptionEventArgs {
    pub fn _ctor(
        &mut self,
        exception: *mut crate::System::AggregateException,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (exception))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        exception: *mut crate::System::AggregateException,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (exception))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Threading+Tasks+UnobservedTaskExceptionEventArgs")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::UnobservedTaskExceptionEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
