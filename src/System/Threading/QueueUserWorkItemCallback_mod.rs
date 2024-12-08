#[cfg(feature = "System+Threading+QueueUserWorkItemCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct QueueUserWorkItemCallback {
    __cordl_parent: crate::System::Object,
    pub callback: *mut crate::System::Threading::WaitCallback,
    pub context: *mut crate::System::Threading::ExecutionContext,
    pub state: *mut crate::System::Object,
}
#[cfg(feature = "System+Threading+QueueUserWorkItemCallback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::QueueUserWorkItemCallback =>
    "System.Threading"."QueueUserWorkItemCallback"
);
#[cfg(feature = "System+Threading+QueueUserWorkItemCallback")]
impl std::ops::Deref for crate::System::Threading::QueueUserWorkItemCallback {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+QueueUserWorkItemCallback")]
impl std::ops::DerefMut for crate::System::Threading::QueueUserWorkItemCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+QueueUserWorkItemCallback")]
impl crate::System::Threading::QueueUserWorkItemCallback {
    pub fn New(
        waitCallback: *mut crate::System::Threading::WaitCallback,
        stateObj: *mut crate::System::Object,
        compressStack: bool,
        stackMark: quest_hook::libil2cpp::ByRefMut<
            crate::System::Threading::StackCrawlMark,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (waitCallback, stateObj, compressStack, stackMark))?;
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
        waitCallback: *mut crate::System::Threading::WaitCallback,
        stateObj: *mut crate::System::Object,
        compressStack: bool,
        stackMark: quest_hook::libil2cpp::ByRefMut<
            crate::System::Threading::StackCrawlMark,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (waitCallback, stateObj, compressStack, stackMark))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Threading+QueueUserWorkItemCallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::QueueUserWorkItemCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
