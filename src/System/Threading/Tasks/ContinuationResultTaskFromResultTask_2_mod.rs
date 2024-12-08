#[cfg(feature = "System+Threading+Tasks+ContinuationResultTaskFromResultTask_2")]
#[repr(C)]
#[derive(Debug)]
pub struct ContinuationResultTaskFromResultTask_2<
    TAntecedentResult: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::Threading::Tasks::Task_1<TResult>,
    pub m_antecedent: *mut crate::System::Threading::Tasks::Task_1<TAntecedentResult>,
    __cordl_phantom_TAntecedentResult: std::marker::PhantomData<TAntecedentResult>,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
}
#[cfg(feature = "System+Threading+Tasks+ContinuationResultTaskFromResultTask_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::Tasks::ContinuationResultTaskFromResultTask_2 <
    TAntecedentResult, TResult > => "System.Threading.Tasks"
    ."ContinuationResultTaskFromResultTask`2" < TAntecedentResult, TResult >
);
#[cfg(feature = "System+Threading+Tasks+ContinuationResultTaskFromResultTask_2")]
impl<
    TAntecedentResult: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::System::Threading::Tasks::ContinuationResultTaskFromResultTask_2<
    TAntecedentResult,
    TResult,
> {
    type Target = crate::System::Threading::Tasks::Task_1<TResult>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+ContinuationResultTaskFromResultTask_2")]
impl<
    TAntecedentResult: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::System::Threading::Tasks::ContinuationResultTaskFromResultTask_2<
    TAntecedentResult,
    TResult,
> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+ContinuationResultTaskFromResultTask_2")]
impl<
    TAntecedentResult: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> crate::System::Threading::Tasks::ContinuationResultTaskFromResultTask_2<
    TAntecedentResult,
    TResult,
> {
    pub fn _ctor(
        &mut self,
        antecedent: *mut crate::System::Threading::Tasks::Task_1<TAntecedentResult>,
        function: *mut crate::System::Delegate,
        state: *mut crate::System::Object,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        internalOptions: crate::System::Threading::Tasks::InternalTaskOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TAntecedentResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (antecedent, function, state, creationOptions, internalOptions),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InnerInvoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TAntecedentResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
        function: *mut crate::System::Delegate,
        state: *mut crate::System::Object,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        internalOptions: crate::System::Threading::Tasks::InternalTaskOptions,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (antecedent, function, state, creationOptions, internalOptions),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Threading+Tasks+ContinuationResultTaskFromResultTask_2")]
impl<
    TAntecedentResult: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::ContinuationResultTaskFromResultTask_2<
    TAntecedentResult,
    TResult,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
