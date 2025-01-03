#[cfg(feature = "System+Net+ServicePointScheduler")]
#[repr(C)]
#[derive(Debug)]
pub struct ServicePointScheduler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _ServicePoint_k__BackingField: *mut crate::System::Net::ServicePoint,
    pub running: i32,
    pub maxIdleTime: i32,
    pub schedulerEvent: *mut crate::System::Net::ServicePointScheduler_AsyncManualResetEvent,
    pub defaultGroup: *mut crate::System::Net::ServicePointScheduler_ConnectionGroup,
    pub groups: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    #[cfg(feature = "System+Net+ServicePointScheduler+AsyncManualResetEvent")]
    pub type AsyncManualResetEvent = crate::System::Net::ServicePointScheduler_AsyncManualResetEvent;
    #[cfg(feature = "System+Net+ServicePointScheduler+ConnectionGroup")]
    pub type ConnectionGroup = crate::System::Net::ServicePointScheduler_ConnectionGroup;
    pub fn Cleanup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Cleanup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CloseConnectionGroup(
        &mut self,
        groupName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CloseConnectionGroup", (groupName))?;
        Ok(__cordl_ret.into())
    }
    pub fn CloseIdleConnection(
        &mut self,
        group: quest_hook::libil2cpp::Gc<
            crate::System::Net::ServicePointScheduler_ConnectionGroup,
        >,
        connection: quest_hook::libil2cpp::Gc<crate::System::Net::WebConnection>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CloseIdleConnection", (group, connection))?;
        Ok(__cordl_ret.into())
    }
    pub fn FinalCleanup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinalCleanup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetConnectionGroup(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Net::ServicePointScheduler_ConnectionGroup,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::ServicePointScheduler_ConnectionGroup,
        > = __cordl_object.invoke("GetConnectionGroup", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        servicePoint: quest_hook::libil2cpp::Gc<crate::System::Net::ServicePoint>,
        connectionLimit: i32,
        maxIdleTime: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (servicePoint, connectionLimit, maxIdleTime))?;
        Ok(__cordl_object.into())
    }
    pub fn OnConnectionClosed(
        &mut self,
        connection: quest_hook::libil2cpp::Gc<crate::System::Net::WebConnection>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnConnectionClosed", (connection))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnConnectionCreated(
        &mut self,
        connection: quest_hook::libil2cpp::Gc<crate::System::Net::WebConnection>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnConnectionCreated", (connection))?;
        Ok(__cordl_ret.into())
    }
    pub fn OperationCompleted(
        &mut self,
        group: quest_hook::libil2cpp::Gc<
            crate::System::Net::ServicePointScheduler_ConnectionGroup,
        >,
        operation: quest_hook::libil2cpp::Gc<crate::System::Net::WebOperation>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("OperationCompleted", (group, operation))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveIdleConnection(
        &mut self,
        connection: quest_hook::libil2cpp::Gc<crate::System::Net::WebConnection>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveIdleConnection", (connection))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveOperation(
        &mut self,
        operation: quest_hook::libil2cpp::Gc<crate::System::Net::WebOperation>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveOperation", (operation))?;
        Ok(__cordl_ret.into())
    }
    pub fn Run(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object.invoke("Run", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RunScheduler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("RunScheduler", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RunSchedulerIteration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RunSchedulerIteration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SchedulerIteration(
        &mut self,
        group: quest_hook::libil2cpp::Gc<
            crate::System::Net::ServicePointScheduler_ConnectionGroup,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("SchedulerIteration", (group))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendRequest(
        &mut self,
        operation: quest_hook::libil2cpp::Gc<crate::System::Net::WebOperation>,
        groupName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendRequest", (operation, groupName))?;
        Ok(__cordl_ret.into())
    }
    pub fn WaitAsync(
        workerTask: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        millisecondTimeout: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WaitAsync", (workerTask, millisecondTimeout))?;
        Ok(__cordl_ret.into())
    }
    pub fn _Run_b__31_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("<Run>b__31_0", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        servicePoint: quest_hook::libil2cpp::Gc<crate::System::Net::ServicePoint>,
        connectionLimit: i32,
        maxIdleTime: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (servicePoint, connectionLimit, maxIdleTime))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MaxIdleTime(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_MaxIdleTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ServicePoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::ServicePoint>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::ServicePoint> = __cordl_object
            .invoke("get_ServicePoint", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ServicePoint(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Net::ServicePoint>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ServicePoint", (value))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "System+Net+ServicePointScheduler+AsyncManualResetEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct ServicePointScheduler_AsyncManualResetEvent {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn New(
        state: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (state))?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Set(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object.invoke("Set", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WaitAsync(
        &mut self,
        millisecondTimeout: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = __cordl_object.invoke("WaitAsync", (millisecondTimeout))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _Scheduler_k__BackingField: *mut crate::System::Net::ServicePointScheduler,
    pub _Name_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn Cleanup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Cleanup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateOrReuseConnection(
        &mut self,
        operation: quest_hook::libil2cpp::Gc<crate::System::Net::WebOperation>,
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
        Ok(__cordl_ret.into())
    }
    pub fn EnqueueOperation(
        &mut self,
        operation: quest_hook::libil2cpp::Gc<crate::System::Net::WebOperation>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnqueueOperation", (operation))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindIdleConnection(
        &mut self,
        operation: quest_hook::libil2cpp::Gc<crate::System::Net::WebOperation>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::WebConnection>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::WebConnection> = __cordl_object
            .invoke("FindIdleConnection", (operation))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNextOperation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::WebOperation>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::WebOperation> = __cordl_object
            .invoke("GetNextOperation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEmpty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        scheduler: quest_hook::libil2cpp::Gc<crate::System::Net::ServicePointScheduler>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (scheduler, name))?;
        Ok(__cordl_object.into())
    }
    pub fn RemoveConnection(
        &mut self,
        connection: quest_hook::libil2cpp::Gc<crate::System::Net::WebConnection>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveConnection", (connection))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        scheduler: quest_hook::libil2cpp::Gc<crate::System::Net::ServicePointScheduler>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (scheduler, name))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Scheduler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::ServicePointScheduler>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::ServicePointScheduler,
        > = __cordl_object.invoke("get_Scheduler", ())?;
        Ok(__cordl_ret.into())
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
