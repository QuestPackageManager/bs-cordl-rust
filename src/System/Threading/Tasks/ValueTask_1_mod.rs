#[cfg(feature = "System+Threading+Tasks+ValueTask_1+ValueTaskSourceAsTask")]
#[repr(C)]
#[derive(Debug)]
pub struct ValueTask_1_ValueTaskSourceAsTask<TResult: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Threading::Tasks::Task_1<TResult>,
    pub _source: *mut crate::System::Threading::Tasks::Sources::IValueTaskSource_1<
        TResult,
    >,
    pub _token: i16,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
}
#[cfg(feature = "System+Threading+Tasks+ValueTask_1+ValueTaskSourceAsTask")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::Tasks::ValueTask_1_ValueTaskSourceAsTask < TResult > =>
    "System.Threading.Tasks"."ValueTask`1/ValueTaskSourceAsTask" < TResult >
);
#[cfg(feature = "System+Threading+Tasks+ValueTask_1+ValueTaskSourceAsTask")]
impl<TResult: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Threading::Tasks::ValueTask_1_ValueTaskSourceAsTask<TResult> {
    type Target = crate::System::Threading::Tasks::Task_1<TResult>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+ValueTask_1+ValueTaskSourceAsTask")]
impl<TResult: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Threading::Tasks::ValueTask_1_ValueTaskSourceAsTask<TResult> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+ValueTask_1+ValueTaskSourceAsTask")]
impl<
    TResult: quest_hook::libil2cpp::Type,
> crate::System::Threading::Tasks::ValueTask_1_ValueTaskSourceAsTask<TResult> {
    #[cfg(feature = "System+Threading+Tasks+ValueTask_1+ValueTaskSourceAsTask+__c")]
    pub type __c = crate::System::Threading::Tasks::ValueTaskSourceAsTask___c<TResult>;
    pub fn _ctor(
        &mut self,
        source: *mut crate::System::Threading::Tasks::Sources::IValueTaskSource_1<
            TResult,
        >,
        token: i16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (source, token))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        source: *mut crate::System::Threading::Tasks::Sources::IValueTaskSource_1<
            TResult,
        >,
        token: i16,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (source, token))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Threading+Tasks+ValueTask_1+ValueTaskSourceAsTask")]
impl<TResult: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::ValueTask_1_ValueTaskSourceAsTask<TResult> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Threading+Tasks+ValueTask_1")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ValueTask_1<TResult: quest_hook::libil2cpp::Type> {
    pub _obj: *mut crate::System::Object,
    pub _result: TResult,
    pub _token: i16,
    pub _continueOnCapturedContext: bool,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
}
#[cfg(feature = "System+Threading+Tasks+ValueTask_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Tasks::ValueTask_1 < TResult
    > => "System.Threading.Tasks"."ValueTask`1<TResult>" < TResult >
);
#[cfg(feature = "System+Threading+Tasks+ValueTask_1")]
unsafe impl<TResult: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::System::Threading::Tasks::ValueTask_1<TResult> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Threading+Tasks+ValueTask_1")]
impl<
    TResult: quest_hook::libil2cpp::Type,
> crate::System::Threading::Tasks::ValueTask_1<TResult> {
    #[cfg(feature = "System+Threading+Tasks+ValueTask_1+ValueTaskSourceAsTask")]
    pub type ValueTaskSourceAsTask = crate::System::Threading::Tasks::ValueTask_1_ValueTaskSourceAsTask<
        TResult,
    >;
    pub fn Equals_Object0(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_ValueTask_1_1(
        &mut self,
        other: crate::System::Threading::Tasks::ValueTask_1<TResult>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetAwaiter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::CompilerServices::ValueTaskAwaiter_1<TResult>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::System::Runtime::CompilerServices::ValueTaskAwaiter_1<
            TResult,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "GetAwaiter", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_TResult0(
        &mut self,
        result: TResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (result),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Task_1_1(
        &mut self,
        task: *mut crate::System::Threading::Tasks::Task_1<TResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (task),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IValueTaskSource_1_i16_2(
        &mut self,
        source: *mut crate::System::Threading::Tasks::Sources::IValueTaskSource_1<
            TResult,
        >,
        token: i16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (source, token),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Object_TResult_i16__cordl_bool3(
        &mut self,
        obj: *mut crate::System::Object,
        result: TResult,
        token: i16,
        continueOnCapturedContext: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (obj, result, token, continueOnCapturedContext),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_IsCompleted(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsCompleted",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn AsTask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<TResult>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<TResult> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AsTask",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetTaskForValueTaskSource(
        &mut self,
        t: *mut crate::System::Threading::Tasks::Sources::IValueTaskSource_1<TResult>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<TResult>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<TResult> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetTaskForValueTaskSource",
            (t),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureAwait(
        &mut self,
        continueOnCapturedContext: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::CompilerServices::ConfiguredValueTaskAwaitable_1<TResult>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::System::Runtime::CompilerServices::ConfiguredValueTaskAwaitable_1<
            TResult,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ConfigureAwait",
            (continueOnCapturedContext),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_IsCompletedSuccessfully(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsCompletedSuccessfully",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Result(&mut self) -> quest_hook::libil2cpp::Result<TResult>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TResult = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Result",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
