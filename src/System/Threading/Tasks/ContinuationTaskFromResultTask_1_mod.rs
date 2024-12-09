#[cfg(feature = "System+Threading+Tasks+ContinuationTaskFromResultTask_1")]
#[repr(C)]
#[derive(Debug)]
pub struct ContinuationTaskFromResultTask_1<
    TAntecedentResult: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::Threading::Tasks::Task,
    pub m_antecedent: *mut crate::System::Threading::Tasks::Task_1<TAntecedentResult>,
    __cordl_phantom_TAntecedentResult: std::marker::PhantomData<TAntecedentResult>,
}
#[cfg(feature = "System+Threading+Tasks+ContinuationTaskFromResultTask_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::Tasks::ContinuationTaskFromResultTask_1 < TAntecedentResult > =>
    "System.Threading.Tasks"."ContinuationTaskFromResultTask`1" < TAntecedentResult >
);
#[cfg(feature = "System+Threading+Tasks+ContinuationTaskFromResultTask_1")]
impl<TAntecedentResult: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Threading::Tasks::ContinuationTaskFromResultTask_1<
    TAntecedentResult,
> {
    type Target = crate::System::Threading::Tasks::Task;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+ContinuationTaskFromResultTask_1")]
impl<TAntecedentResult: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Threading::Tasks::ContinuationTaskFromResultTask_1<
    TAntecedentResult,
> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+ContinuationTaskFromResultTask_1")]
impl<
    TAntecedentResult: quest_hook::libil2cpp::Type,
> crate::System::Threading::Tasks::ContinuationTaskFromResultTask_1<TAntecedentResult> {
    pub fn InnerInvoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TAntecedentResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InnerInvoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        antecedent: *mut crate::System::Threading::Tasks::Task_1<TAntecedentResult>,
        action: *mut crate::System::Delegate,
        state: *mut crate::System::Object,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        internalOptions: crate::System::Threading::Tasks::InternalTaskOptions,
    ) -> quest_hook::libil2cpp::Result<*mut Self>
    where
        TAntecedentResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (antecedent, action, state, creationOptions, internalOptions),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        antecedent: *mut crate::System::Threading::Tasks::Task_1<TAntecedentResult>,
        action: *mut crate::System::Delegate,
        state: *mut crate::System::Object,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        internalOptions: crate::System::Threading::Tasks::InternalTaskOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TAntecedentResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
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
}
#[cfg(feature = "System+Threading+Tasks+ContinuationTaskFromResultTask_1")]
impl<TAntecedentResult: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::ContinuationTaskFromResultTask_1<
    TAntecedentResult,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
