#[cfg(feature = "System+Net+ServicePointScheduler+AsyncManualResetEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct ServicePointScheduler_AsyncManualResetEvent {
    __cordl_parent: crate::System::Object,
    pub m_tcs: *mut crate::System::Threading::Tasks::TaskCompletionSource_1<bool>,
}
#[cfg(feature = "System+Net+ServicePointScheduler+AsyncManualResetEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::ServicePointScheduler_AsyncManualResetEvent => "System.Net"
    ."ServicePointScheduler/AsyncManualResetEvent"
);
#[cfg(feature = "System+Net+ServicePointScheduler+AsyncManualResetEvent")]
impl std::ops::Deref
for crate::System::Net::ServicePointScheduler_AsyncManualResetEvent {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ServicePointScheduler+AsyncManualResetEvent")]
impl std::ops::DerefMut
for crate::System::Net::ServicePointScheduler_AsyncManualResetEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ServicePointScheduler+AsyncManualResetEvent")]
impl crate::System::Net::ServicePointScheduler_AsyncManualResetEvent {
    #[cfg(feature = "System+Net+ServicePointScheduler+AsyncManualResetEvent+__c")]
    pub type __c = crate::System::Net::AsyncManualResetEvent___c;
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn Set(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object.invoke("Set", ())?;
        Ok(__cordl_ret)
    }
    pub fn WaitAsync(
        &mut self,
        millisecondTimeout: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<bool>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<bool> = __cordl_object
            .invoke("WaitAsync", (millisecondTimeout))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        state: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (state))?;
        Ok(__cordl_ret)
    }
    pub fn New(state: bool) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (state))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Net+ServicePointScheduler+AsyncManualResetEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::ServicePointScheduler_AsyncManualResetEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+ServicePointScheduler+ConnectionGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct ServicePointScheduler_ConnectionGroup {
    __cordl_parent: crate::System::Object,
    pub _Scheduler_k__BackingField: *mut crate::System::Net::ServicePointScheduler,
    pub _Name_k__BackingField: *mut crate::System::String,
    pub _cordl_ID: i32,
    pub connections: *mut crate::System::Collections::Generic::LinkedList_1<
        *mut crate::System::Net::WebConnection,
    >,
    pub queue: *mut crate::System::Collections::Generic::LinkedList_1<
        *mut crate::System::Net::WebOperation,
    >,
}
#[cfg(feature = "System+Net+ServicePointScheduler+ConnectionGroup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::ServicePointScheduler_ConnectionGroup => "System.Net"
    ."ServicePointScheduler/ConnectionGroup"
);
#[cfg(feature = "System+Net+ServicePointScheduler+ConnectionGroup")]
impl std::ops::Deref for crate::System::Net::ServicePointScheduler_ConnectionGroup {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ServicePointScheduler+ConnectionGroup")]
impl std::ops::DerefMut for crate::System::Net::ServicePointScheduler_ConnectionGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ServicePointScheduler+ConnectionGroup")]
impl crate::System::Net::ServicePointScheduler_ConnectionGroup {
    pub fn FindIdleConnection(
        &mut self,
        operation: *mut crate::System::Net::WebOperation,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::WebConnection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::WebConnection = __cordl_object
            .invoke("FindIdleConnection", (operation))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveConnection(
        &mut self,
        connection: *mut crate::System::Net::WebConnection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveConnection", (connection))?;
        Ok(__cordl_ret)
    }
    pub fn get_Scheduler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::ServicePointScheduler> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::ServicePointScheduler = __cordl_object
            .invoke("get_Scheduler", ())?;
        Ok(__cordl_ret)
    }
    pub fn EnqueueOperation(
        &mut self,
        operation: *mut crate::System::Net::WebOperation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnqueueOperation", (operation))?;
        Ok(__cordl_ret)
    }
    pub fn IsEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEmpty", ())?;
        Ok(__cordl_ret)
    }
    pub fn Cleanup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Cleanup", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateOrReuseConnection(
        &mut self,
        operation: *mut crate::System::Net::WebOperation,
        force: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ValueTuple_2<*mut crate::System::Net::WebConnection, bool>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ValueTuple_2<
            *mut crate::System::Net::WebConnection,
            bool,
        > = __cordl_object.invoke("CreateOrReuseConnection", (operation, force))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        scheduler: *mut crate::System::Net::ServicePointScheduler,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (scheduler, name))?;
        Ok(__cordl_ret)
    }
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetNextOperation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::WebOperation> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::WebOperation = __cordl_object
            .invoke("GetNextOperation", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        scheduler: *mut crate::System::Net::ServicePointScheduler,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (scheduler, name))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Net+ServicePointScheduler+ConnectionGroup")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::ServicePointScheduler_ConnectionGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+ServicePointScheduler")]
#[repr(C)]
#[derive(Debug)]
pub struct ServicePointScheduler {
    __cordl_parent: crate::System::Object,
    pub _ServicePoint_k__BackingField: *mut crate::System::Net::ServicePoint,
    pub running: i32,
    pub maxIdleTime: i32,
    pub schedulerEvent: *mut crate::System::Net::ServicePointScheduler_AsyncManualResetEvent,
    pub defaultGroup: *mut crate::System::Net::ServicePointScheduler_ConnectionGroup,
    pub groups: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::System::Net::ServicePointScheduler_ConnectionGroup,
    >,
    pub operations: *mut crate::System::Collections::Generic::LinkedList_1<
        crate::System::ValueTuple_2<
            *mut crate::System::Net::ServicePointScheduler_ConnectionGroup,
            *mut crate::System::Net::WebOperation,
        >,
    >,
    pub idleConnections: *mut crate::System::Collections::Generic::LinkedList_1<
        crate::System::ValueTuple_3<
            *mut crate::System::Net::ServicePointScheduler_ConnectionGroup,
            *mut crate::System::Net::WebConnection,
            *mut crate::System::Threading::Tasks::Task,
        >,
    >,
    pub currentConnections: i32,
    pub connectionLimit: i32,
    pub idleSince: crate::System::DateTime,
    pub _cordl_ID: i32,
}
#[cfg(feature = "System+Net+ServicePointScheduler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::ServicePointScheduler =>
    "System.Net"."ServicePointScheduler"
);
#[cfg(feature = "System+Net+ServicePointScheduler")]
impl std::ops::Deref for crate::System::Net::ServicePointScheduler {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ServicePointScheduler")]
impl std::ops::DerefMut for crate::System::Net::ServicePointScheduler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ServicePointScheduler")]
impl crate::System::Net::ServicePointScheduler {
    #[cfg(feature = "System+Net+ServicePointScheduler+ConnectionGroup")]
    pub type ConnectionGroup = crate::System::Net::ServicePointScheduler_ConnectionGroup;
    #[cfg(feature = "System+Net+ServicePointScheduler+_WaitAsync_d__46")]
    pub type _WaitAsync_d__46 = crate::System::Net::ServicePointScheduler__WaitAsync_d__46;
    #[cfg(feature = "System+Net+ServicePointScheduler+_RunScheduler_d__32")]
    pub type _RunScheduler_d__32 = crate::System::Net::ServicePointScheduler__RunScheduler_d__32;
    #[cfg(feature = "System+Net+ServicePointScheduler+AsyncManualResetEvent")]
    pub type AsyncManualResetEvent = crate::System::Net::ServicePointScheduler_AsyncManualResetEvent;
    pub fn FinalCleanup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinalCleanup", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnConnectionClosed(
        &mut self,
        connection: *mut crate::System::Net::WebConnection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnConnectionClosed", (connection))?;
        Ok(__cordl_ret)
    }
    pub fn Run(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object.invoke("Run", ())?;
        Ok(__cordl_ret)
    }
    pub fn OperationCompleted(
        &mut self,
        group: *mut crate::System::Net::ServicePointScheduler_ConnectionGroup,
        operation: *mut crate::System::Net::WebOperation,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("OperationCompleted", (group, operation))?;
        Ok(__cordl_ret)
    }
    pub fn OnConnectionCreated(
        &mut self,
        connection: *mut crate::System::Net::WebConnection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnConnectionCreated", (connection))?;
        Ok(__cordl_ret)
    }
    pub fn RunScheduler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("RunScheduler", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetConnectionGroup(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Net::ServicePointScheduler_ConnectionGroup,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::ServicePointScheduler_ConnectionGroup = __cordl_object
            .invoke("GetConnectionGroup", (name))?;
        Ok(__cordl_ret)
    }
    pub fn SchedulerIteration(
        &mut self,
        group: *mut crate::System::Net::ServicePointScheduler_ConnectionGroup,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("SchedulerIteration", (group))?;
        Ok(__cordl_ret)
    }
    pub fn SendRequest(
        &mut self,
        operation: *mut crate::System::Net::WebOperation,
        groupName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendRequest", (operation, groupName))?;
        Ok(__cordl_ret)
    }
    pub fn CloseConnectionGroup(
        &mut self,
        groupName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CloseConnectionGroup", (groupName))?;
        Ok(__cordl_ret)
    }
    pub fn get_ServicePoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::ServicePoint> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::ServicePoint = __cordl_object
            .invoke("get_ServicePoint", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ServicePoint(
        &mut self,
        value: *mut crate::System::Net::ServicePoint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ServicePoint", (value))?;
        Ok(__cordl_ret)
    }
    pub fn CloseIdleConnection(
        &mut self,
        group: *mut crate::System::Net::ServicePointScheduler_ConnectionGroup,
        connection: *mut crate::System::Net::WebConnection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CloseIdleConnection", (group, connection))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveIdleConnection(
        &mut self,
        connection: *mut crate::System::Net::WebConnection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveIdleConnection", (connection))?;
        Ok(__cordl_ret)
    }
    pub fn RunSchedulerIteration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RunSchedulerIteration", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MaxIdleTime(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_MaxIdleTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn RemoveOperation(
        &mut self,
        operation: *mut crate::System::Net::WebOperation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveOperation", (operation))?;
        Ok(__cordl_ret)
    }
    pub fn _Run_b__31_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("<Run>b__31_0", ())?;
        Ok(__cordl_ret)
    }
    pub fn Cleanup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Cleanup", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        servicePoint: *mut crate::System::Net::ServicePoint,
        connectionLimit: i32,
        maxIdleTime: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (servicePoint, connectionLimit, maxIdleTime))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        servicePoint: *mut crate::System::Net::ServicePoint,
        connectionLimit: i32,
        maxIdleTime: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (servicePoint, connectionLimit, maxIdleTime))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Net+ServicePointScheduler")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::ServicePointScheduler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
