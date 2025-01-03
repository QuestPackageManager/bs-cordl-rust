#[cfg(feature = "System+Threading+SemaphoreSlim")]
#[repr(C)]
#[derive(Debug)]
pub struct SemaphoreSlim {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_currentCount: i32,
    pub m_maxCount: i32,
    pub m_waitCount: i32,
    pub m_lockObj: *mut quest_hook::libil2cpp::Il2CppObject,
    pub m_waitHandle: *mut crate::System::Threading::ManualResetEvent,
    pub m_asyncHead: *mut crate::System::Threading::SemaphoreSlim_TaskNode,
    pub m_asyncTail: *mut crate::System::Threading::SemaphoreSlim_TaskNode,
}
#[cfg(feature = "System+Threading+SemaphoreSlim")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::SemaphoreSlim =>
    "System.Threading"."SemaphoreSlim"
);
#[cfg(feature = "System+Threading+SemaphoreSlim")]
impl std::ops::Deref for crate::System::Threading::SemaphoreSlim {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+SemaphoreSlim")]
impl std::ops::DerefMut for crate::System::Threading::SemaphoreSlim {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+SemaphoreSlim")]
impl crate::System::Threading::SemaphoreSlim {
    pub const NO_MAXIMUM: i32 = 2147483647i32;
    #[cfg(feature = "System+Threading+SemaphoreSlim+TaskNode")]
    pub type TaskNode = crate::System::Threading::SemaphoreSlim_TaskNode;
    #[cfg(
        feature = "System+Threading+SemaphoreSlim+_WaitUntilCountOrTimeoutAsync_d__32"
    )]
    pub type _WaitUntilCountOrTimeoutAsync_d__32 = crate::System::Threading::SemaphoreSlim__WaitUntilCountOrTimeoutAsync_d__32;
    pub fn CancellationTokenCanceledEventHandler(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CancellationTokenCanceledEventHandler", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckDispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckDispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateAndAddAsyncWaiter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::SemaphoreSlim_TaskNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::SemaphoreSlim_TaskNode,
        > = __cordl_object.invoke("CreateAndAddAsyncWaiter", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose__cordl_bool1(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetResourceString(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetResourceString", (str))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_i32_0(
        initialCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initialCount))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_1(
        initialCount: i32,
        maxCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initialCount, maxCount))?;
        Ok(__cordl_object.into())
    }
    pub fn QueueWaiterTask(
        waiterTask: quest_hook::libil2cpp::Gc<
            crate::System::Threading::SemaphoreSlim_TaskNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("QueueWaiterTask", (waiterTask))?;
        Ok(__cordl_ret.into())
    }
    pub fn Release_0(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Release", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Release_i32_1(
        &mut self,
        releaseCount: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Release", (releaseCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveAsyncWaiter(
        &mut self,
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::SemaphoreSlim_TaskNode>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("RemoveAsyncWaiter", (task))?;
        Ok(__cordl_ret.into())
    }
    pub fn WaitAsync_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WaitAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WaitAsync_CancellationToken1(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WaitAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WaitAsync_i32_CancellationToken2(
        &mut self,
        millisecondsTimeout: i32,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = __cordl_object
            .invoke("WaitAsync", (millisecondsTimeout, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WaitUntilCountOrTimeout(
        &mut self,
        millisecondsTimeout: i32,
        startTime: u32,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "WaitUntilCountOrTimeout",
                (millisecondsTimeout, startTime, cancellationToken),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WaitUntilCountOrTimeoutAsync(
        &mut self,
        asyncWaiter: quest_hook::libil2cpp::Gc<
            crate::System::Threading::SemaphoreSlim_TaskNode,
        >,
        millisecondsTimeout: i32,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = __cordl_object
            .invoke(
                "WaitUntilCountOrTimeoutAsync",
                (asyncWaiter, millisecondsTimeout, cancellationToken),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Wait_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Wait", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Wait_i32_1(
        &mut self,
        millisecondsTimeout: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Wait", (millisecondsTimeout))?;
        Ok(__cordl_ret.into())
    }
    pub fn Wait_i32_CancellationToken2(
        &mut self,
        millisecondsTimeout: i32,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Wait", (millisecondsTimeout, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_0(
        &mut self,
        initialCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initialCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_1(
        &mut self,
        initialCount: i32,
        maxCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initialCount, maxCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CurrentCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_CurrentCount", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+SemaphoreSlim")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::SemaphoreSlim {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Threading+SemaphoreSlim")]
impl AsRef<crate::System::IDisposable> for crate::System::Threading::SemaphoreSlim {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Threading+SemaphoreSlim")]
impl AsMut<crate::System::IDisposable> for crate::System::Threading::SemaphoreSlim {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Threading+SemaphoreSlim+TaskNode")]
#[repr(C)]
#[derive(Debug)]
pub struct SemaphoreSlim_TaskNode {
    __cordl_parent: crate::System::Threading::Tasks::Task_1<bool>,
    pub Prev: *mut crate::System::Threading::SemaphoreSlim_TaskNode,
    pub Next: *mut crate::System::Threading::SemaphoreSlim_TaskNode,
}
#[cfg(feature = "System+Threading+SemaphoreSlim+TaskNode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::SemaphoreSlim_TaskNode =>
    "System.Threading"."SemaphoreSlim/TaskNode"
);
#[cfg(feature = "System+Threading+SemaphoreSlim+TaskNode")]
impl std::ops::Deref for crate::System::Threading::SemaphoreSlim_TaskNode {
    type Target = crate::System::Threading::Tasks::Task_1<bool>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+SemaphoreSlim+TaskNode")]
impl std::ops::DerefMut for crate::System::Threading::SemaphoreSlim_TaskNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+SemaphoreSlim+TaskNode")]
impl crate::System::Threading::SemaphoreSlim_TaskNode {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
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
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+SemaphoreSlim+TaskNode")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::SemaphoreSlim_TaskNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Threading+SemaphoreSlim+TaskNode")]
impl AsRef<crate::System::Threading::IThreadPoolWorkItem>
for crate::System::Threading::SemaphoreSlim_TaskNode {
    fn as_ref(&self) -> &crate::System::Threading::IThreadPoolWorkItem {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Threading+SemaphoreSlim+TaskNode")]
impl AsMut<crate::System::Threading::IThreadPoolWorkItem>
for crate::System::Threading::SemaphoreSlim_TaskNode {
    fn as_mut(&mut self) -> &mut crate::System::Threading::IThreadPoolWorkItem {
        unsafe { std::mem::transmute(self) }
    }
}
