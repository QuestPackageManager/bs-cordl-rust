#[cfg(feature = "System+Net+WebCompletionSource_1")]
#[repr(C)]
#[derive(Debug)]
pub struct WebCompletionSource_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub completion: quest_hook::libil2cpp::Gc<
        crate::System::Threading::Tasks::TaskCompletionSource_1<
            *mut crate::System::Net::WebCompletionSource_1_Result<T>,
        >,
    >,
    pub currentResult: quest_hook::libil2cpp::Gc<
        crate::System::Net::WebCompletionSource_1_Result<T>,
    >,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+Net+WebCompletionSource_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::WebCompletionSource_1 < T > =>
    "System.Net"."WebCompletionSource`1" < T >
);
#[cfg(feature = "System+Net+WebCompletionSource_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Net::WebCompletionSource_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+WebCompletionSource_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Net::WebCompletionSource_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+WebCompletionSource_1")]
impl<T: quest_hook::libil2cpp::Type> crate::System::Net::WebCompletionSource_1<T> {
    #[cfg(feature = "System+Net+WebCompletionSource_1+Result")]
    pub type Result = crate::System::Net::WebCompletionSource_1_Result<T>;
    #[cfg(feature = "System+Net+WebCompletionSource_1+Status")]
    pub type Status = crate::System::Net::WebCompletionSource_1_Status;
    pub fn New(
        runAsync: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (runAsync))?;
        Ok(__cordl_object.into())
    }
    pub fn ThrowOnError(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowOnError", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TrySetCanceled_0(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TrySetCanceled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TrySetCanceled_OperationCanceledException1(
        &mut self,
        error: quest_hook::libil2cpp::Gc<crate::System::OperationCanceledException>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TrySetCanceled", (error))?;
        Ok(__cordl_ret.into())
    }
    pub fn TrySetCompleted_1(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TrySetCompleted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TrySetCompleted_T0(
        &mut self,
        argument: T,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TrySetCompleted", (argument))?;
        Ok(__cordl_ret.into())
    }
    pub fn TrySetException(
        &mut self,
        error: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TrySetException", (error))?;
        Ok(__cordl_ret.into())
    }
    pub fn WaitForCompletion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<T>,
        > = __cordl_object.invoke("WaitForCompletion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        runAsync: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (runAsync))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CurrentResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::WebCompletionSource_1_Result<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::WebCompletionSource_1_Result<T>,
        > = __cordl_object.invoke("get_CurrentResult", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Task(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("get_Task", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+WebCompletionSource_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Net::WebCompletionSource_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+WebCompletionSource_1+Result")]
#[repr(C)]
#[derive(Debug)]
pub struct WebCompletionSource_1_Result<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _Status_k__BackingField: crate::System::Net::WebCompletionSource_1_Status<T>,
    pub _Error_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
    >,
    pub _Argument_k__BackingField: T,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+Net+WebCompletionSource_1+Result")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::WebCompletionSource_1_Result < T >
    => "System.Net"."WebCompletionSource`1/Result" < T >
);
#[cfg(feature = "System+Net+WebCompletionSource_1+Result")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Net::WebCompletionSource_1_Result<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+WebCompletionSource_1+Result")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Net::WebCompletionSource_1_Result<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+WebCompletionSource_1+Result")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::System::Net::WebCompletionSource_1_Result<T> {
    pub fn New_T0(
        argument: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (argument))?;
        Ok(__cordl_object.into())
    }
    pub fn New_WebCompletionSource_1_Status_ExceptionDispatchInfo1(
        state: crate::System::Net::WebCompletionSource_1_Status<T>,
        error: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (state, error))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_T0(
        &mut self,
        argument: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (argument))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_WebCompletionSource_1_Status_ExceptionDispatchInfo1(
        &mut self,
        state: crate::System::Net::WebCompletionSource_1_Status<T>,
        error: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (state, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Argument(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("get_Argument", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Error(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
        > = __cordl_object.invoke("get_Error", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Status(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Net::WebCompletionSource_1_Status<T>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::WebCompletionSource_1_Status<T> = __cordl_object
            .invoke("get_Status", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Success(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Success", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+WebCompletionSource_1+Result")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Net::WebCompletionSource_1_Result<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+WebCompletionSource_1+Status")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebCompletionSource_1_Status {
    Canceled = 2i32,
    Completed = 1i32,
    Faulted = 3i32,
    Running = 0i32,
}
#[cfg(feature = "System+Net+WebCompletionSource_1+Status")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::WebCompletionSource_1_Status =>
    "System.Net"."WebCompletionSource`1/Status<T>"
);
