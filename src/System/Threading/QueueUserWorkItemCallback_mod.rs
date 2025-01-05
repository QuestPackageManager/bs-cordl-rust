#[cfg(feature = "System+Threading+QueueUserWorkItemCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct QueueUserWorkItemCallback {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub callback: quest_hook::libil2cpp::Gc<crate::System::Threading::WaitCallback>,
    pub context: quest_hook::libil2cpp::Gc<crate::System::Threading::ExecutionContext>,
    pub state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Threading+QueueUserWorkItemCallback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::QueueUserWorkItemCallback =>
    "System.Threading"."QueueUserWorkItemCallback"
);
#[cfg(feature = "System+Threading+QueueUserWorkItemCallback")]
impl std::ops::Deref for crate::System::Threading::QueueUserWorkItemCallback {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        waitCallback: quest_hook::libil2cpp::Gc<crate::System::Threading::WaitCallback>,
        stateObj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        compressStack: bool,
        stackMark: quest_hook::libil2cpp::ByRefMut<
            crate::System::Threading::StackCrawlMark,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (waitCallback, stateObj, compressStack, stackMark))?;
        Ok(__cordl_object.into())
    }
    pub fn System_Threading_IThreadPoolWorkItem_ExecuteWorkItem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Threading.IThreadPoolWorkItem.ExecuteWorkItem", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Threading_IThreadPoolWorkItem_MarkAborted(
        &mut self,
        tae: quest_hook::libil2cpp::Gc<crate::System::Threading::ThreadAbortException>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Threading.IThreadPoolWorkItem.MarkAborted", (tae))?;
        Ok(__cordl_ret.into())
    }
    pub fn WaitCallback_Context(
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WaitCallback_Context", (state))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        waitCallback: quest_hook::libil2cpp::Gc<crate::System::Threading::WaitCallback>,
        stateObj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
        Ok(__cordl_ret.into())
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
#[cfg(feature = "System+Threading+QueueUserWorkItemCallback")]
impl AsRef<crate::System::Threading::IThreadPoolWorkItem>
for crate::System::Threading::QueueUserWorkItemCallback {
    fn as_ref(&self) -> &crate::System::Threading::IThreadPoolWorkItem {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Threading+QueueUserWorkItemCallback")]
impl AsMut<crate::System::Threading::IThreadPoolWorkItem>
for crate::System::Threading::QueueUserWorkItemCallback {
    fn as_mut(&mut self) -> &mut crate::System::Threading::IThreadPoolWorkItem {
        unsafe { std::mem::transmute(self) }
    }
}
