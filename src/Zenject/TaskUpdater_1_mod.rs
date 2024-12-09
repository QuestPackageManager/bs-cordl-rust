#[cfg(feature = "Zenject+TaskUpdater_1")]
#[repr(C)]
#[derive(Debug)]
pub struct TaskUpdater_1<TTask: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Object,
    pub _tasks: *mut crate::System::Collections::Generic::LinkedList_1<
        *mut crate::Zenject::TaskUpdater_1_TaskInfo<TTask>,
    >,
    pub _queuedTasks: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Zenject::TaskUpdater_1_TaskInfo<TTask>,
    >,
    __cordl_phantom_TTask: std::marker::PhantomData<TTask>,
}
#[cfg(feature = "Zenject+TaskUpdater_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::TaskUpdater_1 < TTask > => "Zenject"
    ."TaskUpdater`1" < TTask >
);
#[cfg(feature = "Zenject+TaskUpdater_1")]
impl<TTask: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Zenject::TaskUpdater_1<TTask> {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+TaskUpdater_1")]
impl<TTask: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Zenject::TaskUpdater_1<TTask> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+TaskUpdater_1")]
impl<TTask: quest_hook::libil2cpp::Type> crate::Zenject::TaskUpdater_1<TTask> {
    #[cfg(feature = "Zenject+TaskUpdater_1+TaskInfo")]
    pub type TaskInfo = crate::Zenject::TaskUpdater_1_TaskInfo<TTask>;
    #[cfg(feature = "Zenject+TaskUpdater_1+__c")]
    pub type __c = crate::Zenject::TaskUpdater_1___c<TTask>;
    #[cfg(feature = "Zenject+TaskUpdater_1+__c__DisplayClass8_0")]
    pub type __c__DisplayClass8_0 = crate::Zenject::TaskUpdater_1___c__DisplayClass8_0<
        TTask,
    >;
    pub fn AddQueuedTasks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TTask: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddQueuedTasks", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddTask(
        &mut self,
        task: TTask,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TTask: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddTask", (task, priority))?;
        Ok(__cordl_ret)
    }
    pub fn AddTaskInternal(
        &mut self,
        task: TTask,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TTask: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddTaskInternal", (task, priority))?;
        Ok(__cordl_ret)
    }
    pub fn ClearRemovedTasks(
        &mut self,
        tasks: *mut crate::System::Collections::Generic::LinkedList_1<
            *mut crate::Zenject::TaskUpdater_1_TaskInfo<TTask>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TTask: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearRemovedTasks", (tasks))?;
        Ok(__cordl_ret)
    }
    pub fn InsertTaskSorted(
        &mut self,
        task: *mut crate::Zenject::TaskUpdater_1_TaskInfo<TTask>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TTask: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InsertTaskSorted", (task))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnFrameStart(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TTask: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnFrameStart", ())?;
        Ok(__cordl_ret)
    }
    pub fn RemoveTask(
        &mut self,
        task: TTask,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TTask: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveTask", (task))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateAll(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TTask: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAll", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateItem(
        &mut self,
        task: TTask,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TTask: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateItem", (task))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateRange(
        &mut self,
        minPriority: i32,
        maxPriority: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TTask: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateRange", (minPriority, maxPriority))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TTask: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ActiveTasks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Zenject::TaskUpdater_1_TaskInfo<TTask>,
        >,
    >
    where
        TTask: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Zenject::TaskUpdater_1_TaskInfo<TTask>,
        > = __cordl_object.invoke("get_ActiveTasks", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AllTasks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Zenject::TaskUpdater_1_TaskInfo<TTask>,
        >,
    >
    where
        TTask: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Zenject::TaskUpdater_1_TaskInfo<TTask>,
        > = __cordl_object.invoke("get_AllTasks", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+TaskUpdater_1")]
impl<TTask: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Zenject::TaskUpdater_1<TTask> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+TaskUpdater_1+TaskInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct TaskUpdater_1_TaskInfo<TTask: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Object,
    pub Task: TTask,
    pub Priority: i32,
    pub IsRemoved: bool,
    __cordl_phantom_TTask: std::marker::PhantomData<TTask>,
}
#[cfg(feature = "Zenject+TaskUpdater_1+TaskInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::TaskUpdater_1_TaskInfo < TTask > =>
    "Zenject"."TaskUpdater`1/TaskInfo" < TTask >
);
#[cfg(feature = "Zenject+TaskUpdater_1+TaskInfo")]
impl<TTask: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Zenject::TaskUpdater_1_TaskInfo<TTask> {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+TaskUpdater_1+TaskInfo")]
impl<TTask: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Zenject::TaskUpdater_1_TaskInfo<TTask> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+TaskUpdater_1+TaskInfo")]
impl<TTask: quest_hook::libil2cpp::Type> crate::Zenject::TaskUpdater_1_TaskInfo<TTask> {
    pub fn New(task: TTask, priority: i32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (task, priority))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        task: TTask,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TTask: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (task, priority))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+TaskUpdater_1+TaskInfo")]
impl<TTask: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Zenject::TaskUpdater_1_TaskInfo<TTask> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
