#[cfg(feature = "System+Threading+Tasks+CompletionActionInvoker")]
#[repr(C)]
#[derive(Debug)]
pub struct CompletionActionInvoker {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_action: *mut crate::System::Threading::Tasks::ITaskCompletionAction,
    pub m_completingTask: *mut crate::System::Threading::Tasks::Task,
}
#[cfg(feature = "System+Threading+Tasks+CompletionActionInvoker")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::Tasks::CompletionActionInvoker => "System.Threading.Tasks"
    ."CompletionActionInvoker"
);
#[cfg(feature = "System+Threading+Tasks+CompletionActionInvoker")]
impl std::ops::Deref for crate::System::Threading::Tasks::CompletionActionInvoker {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+CompletionActionInvoker")]
impl std::ops::DerefMut for crate::System::Threading::Tasks::CompletionActionInvoker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+CompletionActionInvoker")]
impl crate::System::Threading::Tasks::CompletionActionInvoker {
    pub fn MarkAborted(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Threading::ThreadAbortException>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkAborted", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        action: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::ITaskCompletionAction,
        >,
        completingTask: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (action, completingTask))?;
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
    pub fn _ctor(
        &mut self,
        action: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::ITaskCompletionAction,
        >,
        completingTask: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (action, completingTask))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+Tasks+CompletionActionInvoker")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::CompletionActionInvoker {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
