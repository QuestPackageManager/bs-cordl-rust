#[cfg(feature = "System+Threading+Tasks+TaskExceptionHolder")]
#[repr(C)]
#[derive(Debug)]
pub struct TaskExceptionHolder {
    __cordl_parent: crate::System::Object,
    pub m_task: *mut crate::System::Threading::Tasks::Task,
    pub m_faultExceptions: *mut crate::System::Collections::Generic::LowLevelListWithIList_1<
        *mut crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
    >,
    pub m_cancellationException: *mut crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
    pub m_isHandled: bool,
}
#[cfg(feature = "System+Threading+Tasks+TaskExceptionHolder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Tasks::TaskExceptionHolder =>
    "System.Threading.Tasks"."TaskExceptionHolder"
);
#[cfg(feature = "System+Threading+Tasks+TaskExceptionHolder")]
impl std::ops::Deref for crate::System::Threading::Tasks::TaskExceptionHolder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskExceptionHolder")]
impl std::ops::DerefMut for crate::System::Threading::Tasks::TaskExceptionHolder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskExceptionHolder")]
impl crate::System::Threading::Tasks::TaskExceptionHolder {
    pub fn _ctor(
        &mut self,
        task: *mut crate::System::Threading::Tasks::Task,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (task))?;
        Ok(__cordl_ret)
    }
    pub fn get_ContainsFaultList(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ContainsFaultList", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetCancellationException(
        &mut self,
        exceptionObject: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCancellationException", (exceptionObject))?;
        Ok(__cordl_ret)
    }
    pub fn GetExceptionDispatchInfos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            *mut crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            *mut crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
        > = __cordl_object.invoke("GetExceptionDispatchInfos", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddFaultException(
        &mut self,
        exceptionObject: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddFaultException", (exceptionObject))?;
        Ok(__cordl_ret)
    }
    pub fn MarkAsHandled(
        &mut self,
        calledFromFinalizer: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkAsHandled", (calledFromFinalizer))?;
        Ok(__cordl_ret)
    }
    pub fn GetCancellationExceptionDispatchInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo = __cordl_object
            .invoke("GetCancellationExceptionDispatchInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn Add(
        &mut self,
        exceptionObject: *mut crate::System::Object,
        representsCancellation: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (exceptionObject, representsCancellation))?;
        Ok(__cordl_ret)
    }
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret)
    }
    pub fn MarkAsUnhandled(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkAsUnhandled", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateExceptionObject(
        &mut self,
        calledFromFinalizer: bool,
        includeThisException: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::AggregateException> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::AggregateException = __cordl_object
            .invoke(
                "CreateExceptionObject",
                (calledFromFinalizer, includeThisException),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        task: *mut crate::System::Threading::Tasks::Task,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (task))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskExceptionHolder")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::TaskExceptionHolder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
