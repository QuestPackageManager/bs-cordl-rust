#[cfg(feature = "System+Threading+Tasks+TaskToApm")]
#[repr(C)]
#[derive(Debug)]
pub struct TaskToApm {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Threading+Tasks+TaskToApm")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Tasks::TaskToApm =>
    "System.Threading.Tasks"."TaskToApm"
);
#[cfg(feature = "System+Threading+Tasks+TaskToApm")]
impl std::ops::Deref for crate::System::Threading::Tasks::TaskToApm {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskToApm")]
impl std::ops::DerefMut for crate::System::Threading::Tasks::TaskToApm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskToApm")]
impl crate::System::Threading::Tasks::TaskToApm {
    #[cfg(feature = "System+Threading+Tasks+TaskToApm+TaskWrapperAsyncResult")]
    pub type TaskWrapperAsyncResult = crate::System::Threading::Tasks::TaskToApm_TaskWrapperAsyncResult;
    #[cfg(feature = "System+Threading+Tasks+TaskToApm+__c__DisplayClass3_0")]
    pub type __c__DisplayClass3_0 = crate::System::Threading::Tasks::TaskToApm___c__DisplayClass3_0;
}
#[cfg(feature = "System+Threading+Tasks+TaskToApm")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::Tasks::TaskToApm {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskToApm+TaskWrapperAsyncResult")]
#[repr(C)]
#[derive(Debug)]
pub struct TaskToApm_TaskWrapperAsyncResult {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Task: *mut crate::System::Threading::Tasks::Task,
    pub _state: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _completedSynchronously: bool,
}
#[cfg(feature = "System+Threading+Tasks+TaskToApm+TaskWrapperAsyncResult")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::Tasks::TaskToApm_TaskWrapperAsyncResult =>
    "System.Threading.Tasks"."TaskToApm/TaskWrapperAsyncResult"
);
#[cfg(feature = "System+Threading+Tasks+TaskToApm+TaskWrapperAsyncResult")]
impl std::ops::Deref
for crate::System::Threading::Tasks::TaskToApm_TaskWrapperAsyncResult {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskToApm+TaskWrapperAsyncResult")]
impl std::ops::DerefMut
for crate::System::Threading::Tasks::TaskToApm_TaskWrapperAsyncResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskToApm+TaskWrapperAsyncResult")]
impl crate::System::Threading::Tasks::TaskToApm_TaskWrapperAsyncResult {
    pub fn New(
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        completedSynchronously: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (task, state, completedSynchronously))?;
        Ok(__cordl_object.into())
    }
    pub fn System_IAsyncResult_get_AsyncState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("System.IAsyncResult.get_AsyncState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IAsyncResult_get_AsyncWaitHandle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::WaitHandle>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::WaitHandle,
        > = __cordl_object.invoke("System.IAsyncResult.get_AsyncWaitHandle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IAsyncResult_get_CompletedSynchronously(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.IAsyncResult.get_CompletedSynchronously", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IAsyncResult_get_IsCompleted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.IAsyncResult.get_IsCompleted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        completedSynchronously: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (task, state, completedSynchronously))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskToApm+TaskWrapperAsyncResult")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::TaskToApm_TaskWrapperAsyncResult {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskToApm+TaskWrapperAsyncResult")]
impl AsRef<crate::System::IAsyncResult>
for crate::System::Threading::Tasks::TaskToApm_TaskWrapperAsyncResult {
    fn as_ref(&self) -> &crate::System::IAsyncResult {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskToApm+TaskWrapperAsyncResult")]
impl AsMut<crate::System::IAsyncResult>
for crate::System::Threading::Tasks::TaskToApm_TaskWrapperAsyncResult {
    fn as_mut(&mut self) -> &mut crate::System::IAsyncResult {
        unsafe { std::mem::transmute(self) }
    }
}
