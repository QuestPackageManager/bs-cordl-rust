#[cfg(feature = "System+Threading+Tasks+ContinuationTaskFromTask")]
#[repr(C)]
#[derive(Debug)]
pub struct ContinuationTaskFromTask {
    __cordl_parent: crate::System::Threading::Tasks::Task,
    pub m_antecedent: *mut crate::System::Threading::Tasks::Task,
}
#[cfg(feature = "System+Threading+Tasks+ContinuationTaskFromTask")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::Tasks::ContinuationTaskFromTask => "System.Threading.Tasks"
    ."ContinuationTaskFromTask"
);
#[cfg(feature = "System+Threading+Tasks+ContinuationTaskFromTask")]
impl std::ops::Deref for crate::System::Threading::Tasks::ContinuationTaskFromTask {
    type Target = crate::System::Threading::Tasks::Task;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+ContinuationTaskFromTask")]
impl std::ops::DerefMut for crate::System::Threading::Tasks::ContinuationTaskFromTask {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+ContinuationTaskFromTask")]
impl crate::System::Threading::Tasks::ContinuationTaskFromTask {
    pub fn _ctor(
        &mut self,
        antecedent: *mut crate::System::Threading::Tasks::Task,
        action: *mut crate::System::Delegate,
        state: *mut crate::System::Object,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        internalOptions: crate::System::Threading::Tasks::InternalTaskOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (antecedent, action, state, creationOptions, internalOptions),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InnerInvoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InnerInvoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        antecedent: *mut crate::System::Threading::Tasks::Task,
        action: *mut crate::System::Delegate,
        state: *mut crate::System::Object,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        internalOptions: crate::System::Threading::Tasks::InternalTaskOptions,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (antecedent, action, state, creationOptions, internalOptions),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Threading+Tasks+ContinuationTaskFromTask")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::ContinuationTaskFromTask {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
