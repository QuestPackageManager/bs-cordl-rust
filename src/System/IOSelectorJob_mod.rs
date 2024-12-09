#[cfg(feature = "System+IOSelectorJob")]
#[repr(C)]
#[derive(Debug)]
pub struct IOSelectorJob {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub operation: crate::System::IOOperation,
    pub callback: *mut crate::System::IOAsyncCallback,
    pub state: *mut crate::System::IOAsyncResult,
}
#[cfg(feature = "System+IOSelectorJob")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IOSelectorJob => "System"
    ."IOSelectorJob"
);
#[cfg(feature = "System+IOSelectorJob")]
impl std::ops::Deref for crate::System::IOSelectorJob {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IOSelectorJob")]
impl std::ops::DerefMut for crate::System::IOSelectorJob {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IOSelectorJob")]
impl crate::System::IOSelectorJob {
    pub fn MarkDisposed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkDisposed", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        operation: crate::System::IOOperation,
        callback: *mut crate::System::IOAsyncCallback,
        state: *mut crate::System::IOAsyncResult,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (operation, callback, state))?;
        Ok(__cordl_object)
    }
    pub fn System_Threading_IThreadPoolWorkItem_ExecuteWorkItem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Threading.IThreadPoolWorkItem.ExecuteWorkItem", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Threading_IThreadPoolWorkItem_MarkAborted(
        &mut self,
        tae: *mut crate::System::Threading::ThreadAbortException,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Threading.IThreadPoolWorkItem.MarkAborted", (tae))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        operation: crate::System::IOOperation,
        callback: *mut crate::System::IOAsyncCallback,
        state: *mut crate::System::IOAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (operation, callback, state))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+IOSelectorJob")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IOSelectorJob {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
