#[cfg(feature = "System+Threading+AutoResetEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct AutoResetEvent {
    __cordl_parent: crate::System::Threading::EventWaitHandle,
}
#[cfg(feature = "System+Threading+AutoResetEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::AutoResetEvent =>
    "System.Threading"."AutoResetEvent"
);
#[cfg(feature = "System+Threading+AutoResetEvent")]
impl std::ops::Deref for crate::System::Threading::AutoResetEvent {
    type Target = crate::System::Threading::EventWaitHandle;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+AutoResetEvent")]
impl std::ops::DerefMut for crate::System::Threading::AutoResetEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+AutoResetEvent")]
impl crate::System::Threading::AutoResetEvent {
    pub fn _ctor(
        &mut self,
        initialState: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initialState))?;
        Ok(__cordl_ret)
    }
    pub fn New(initialState: bool) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initialState))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Threading+AutoResetEvent")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::AutoResetEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
