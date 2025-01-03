#[cfg(feature = "System+Threading+Tasks+ValueTask")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ValueTask {
    pub _obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _token: i16,
    pub _continueOnCapturedContext: bool,
}
#[cfg(feature = "System+Threading+Tasks+ValueTask")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Tasks::ValueTask =>
    "System.Threading.Tasks"."ValueTask"
);
#[cfg(feature = "System+Threading+Tasks+ValueTask")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Threading::Tasks::ValueTask {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Threading+Tasks+ValueTask")]
impl crate::System::Threading::Tasks::ValueTask {
    #[cfg(feature = "System+Threading+Tasks+ValueTask+ValueTaskSourceAsTask")]
    pub type ValueTaskSourceAsTask = crate::System::Threading::Tasks::ValueTask_ValueTaskSourceAsTask;
    pub fn AsTask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "AsTask", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ConfigureAwait(
        &mut self,
        continueOnCapturedContext: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::CompilerServices::ConfiguredValueTaskAwaitable,
    > {
        let __cordl_ret: crate::System::Runtime::CompilerServices::ConfiguredValueTaskAwaitable = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ConfigureAwait",
            (continueOnCapturedContext),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_ValueTask1(
        &mut self,
        other: crate::System::Threading::Tasks::ValueTask,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAwaiter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::CompilerServices::ValueTaskAwaiter,
    > {
        let __cordl_ret: crate::System::Runtime::CompilerServices::ValueTaskAwaiter = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetAwaiter",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTaskForValueTaskSource(
        &mut self,
        t: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Sources::IValueTaskSource,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetTaskForValueTaskSource",
            (t),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowIfCompletedUnsuccessfully(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ThrowIfCompletedUnsuccessfully",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IValueTaskSource_i16_1(
        &mut self,
        source: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Sources::IValueTaskSource,
        >,
        token: i16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (source, token),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppObject_i16__cordl_bool2(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        token: i16,
        continueOnCapturedContext: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (obj, token, continueOnCapturedContext),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Task0(
        &mut self,
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (task),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CompletedTask() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CompletedTask", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCompleted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsCompleted",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+Tasks+ValueTask")]
impl AsRef<crate::System::IEquatable_1<crate::System::Threading::Tasks::ValueTask>>
for crate::System::Threading::Tasks::ValueTask {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::System::Threading::Tasks::ValueTask> {
        todo!()
    }
}
#[cfg(feature = "System+Threading+Tasks+ValueTask")]
impl AsMut<crate::System::IEquatable_1<crate::System::Threading::Tasks::ValueTask>>
for crate::System::Threading::Tasks::ValueTask {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::System::Threading::Tasks::ValueTask> {
        todo!()
    }
}
#[cfg(feature = "System+Threading+Tasks+ValueTask+ValueTaskSourceAsTask")]
#[repr(C)]
#[derive(Debug)]
pub struct ValueTask_ValueTaskSourceAsTask {
    __cordl_parent: crate::System::Threading::Tasks::Task_1<
        crate::System::Threading::Tasks::VoidTaskResult,
    >,
    pub _source: quest_hook::libil2cpp::Gc<
        crate::System::Threading::Tasks::Sources::IValueTaskSource,
    >,
    pub _token: i16,
}
#[cfg(feature = "System+Threading+Tasks+ValueTask+ValueTaskSourceAsTask")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::Tasks::ValueTask_ValueTaskSourceAsTask =>
    "System.Threading.Tasks"."ValueTask/ValueTaskSourceAsTask"
);
#[cfg(feature = "System+Threading+Tasks+ValueTask+ValueTaskSourceAsTask")]
impl std::ops::Deref
for crate::System::Threading::Tasks::ValueTask_ValueTaskSourceAsTask {
    type Target = crate::System::Threading::Tasks::Task_1<
        crate::System::Threading::Tasks::VoidTaskResult,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+ValueTask+ValueTaskSourceAsTask")]
impl std::ops::DerefMut
for crate::System::Threading::Tasks::ValueTask_ValueTaskSourceAsTask {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+ValueTask+ValueTaskSourceAsTask")]
impl crate::System::Threading::Tasks::ValueTask_ValueTaskSourceAsTask {
    pub fn New(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Sources::IValueTaskSource,
        >,
        token: i16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (source, token))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        source: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Sources::IValueTaskSource,
        >,
        token: i16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (source, token))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+Tasks+ValueTask+ValueTaskSourceAsTask")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::ValueTask_ValueTaskSourceAsTask {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
