#[cfg(feature = "System+Runtime+CompilerServices+TaskAwaiter_1")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TaskAwaiter_1<TResult: quest_hook::libil2cpp::Type> {
    pub m_task: quest_hook::libil2cpp::Gc<
        crate::System::Threading::Tasks::Task_1<TResult>,
    >,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
}
#[cfg(feature = "System+Runtime+CompilerServices+TaskAwaiter_1")]
unsafe impl<TResult: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::System::Runtime::CompilerServices::TaskAwaiter_1<TResult> {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Runtime.CompilerServices";
    const CLASS_NAME: &'static str = "TaskAwaiter`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "System.Runtime.CompilerServices",
                        "TaskAwaiter`1",
                    )
                    .unwrap()
                    .make_generic::<(TResult)>()
                    .unwrap()
                    .unwrap()
            })
    }
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TaskAwaiter_1")]
unsafe impl<TResult: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Argument
for crate::System::Runtime::CompilerServices::TaskAwaiter_1<TResult> {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TaskAwaiter_1")]
unsafe impl<TResult: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Parameter
for crate::System::Runtime::CompilerServices::TaskAwaiter_1<TResult> {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TaskAwaiter_1")]
unsafe impl<TResult: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Returned
for crate::System::Runtime::CompilerServices::TaskAwaiter_1<TResult> {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TaskAwaiter_1")]
unsafe impl<TResult: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Return
for crate::System::Runtime::CompilerServices::TaskAwaiter_1<TResult> {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TaskAwaiter_1")]
unsafe impl<TResult: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::System::Runtime::CompilerServices::TaskAwaiter_1<TResult> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TaskAwaiter_1")]
impl<
    TResult: quest_hook::libil2cpp::Type,
> crate::System::Runtime::CompilerServices::TaskAwaiter_1<TResult> {
    pub fn GetResult(&mut self) -> quest_hook::libil2cpp::Result<TResult>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::CompilerServices::TaskAwaiter_1<
            TResult,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), TResult, 0usize>("GetResult")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::CompilerServices::TaskAwaiter_1 < TResult >
                    as quest_hook::libil2cpp::Type > ::class(), "GetResult", 0usize
                )
            });
        let __cordl_ret: TResult = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn OnCompleted(
        &mut self,
        continuation: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::CompilerServices::TaskAwaiter_1<
            TResult,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnCompleted")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::CompilerServices::TaskAwaiter_1 < TResult >
                    as quest_hook::libil2cpp::Type > ::class(), "OnCompleted", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (continuation))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnsafeOnCompleted(
        &mut self,
        continuation: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::CompilerServices::TaskAwaiter_1<
            TResult,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("UnsafeOnCompleted")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::CompilerServices::TaskAwaiter_1 < TResult >
                    as quest_hook::libil2cpp::Type > ::class(), "UnsafeOnCompleted",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (continuation))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TResult>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::CompilerServices::TaskAwaiter_1<
            TResult,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Threading::Tasks::Task_1<TResult>,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::CompilerServices::TaskAwaiter_1 < TResult >
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (task))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCompleted(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::CompilerServices::TaskAwaiter_1<
            TResult,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_IsCompleted")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::CompilerServices::TaskAwaiter_1 < TResult >
                    as quest_hook::libil2cpp::Type > ::class(), "get_IsCompleted", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TaskAwaiter_1")]
impl<
    TResult: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion>
for crate::System::Runtime::CompilerServices::TaskAwaiter_1<TResult> {
    fn as_ref(
        &self,
    ) -> &crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion {
        todo!()
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TaskAwaiter_1")]
impl<
    TResult: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion>
for crate::System::Runtime::CompilerServices::TaskAwaiter_1<TResult> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion {
        todo!()
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TaskAwaiter_1")]
impl<
    TResult: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Runtime::CompilerServices::INotifyCompletion>
for crate::System::Runtime::CompilerServices::TaskAwaiter_1<TResult> {
    fn as_ref(&self) -> &crate::System::Runtime::CompilerServices::INotifyCompletion {
        todo!()
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TaskAwaiter_1")]
impl<
    TResult: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Runtime::CompilerServices::INotifyCompletion>
for crate::System::Runtime::CompilerServices::TaskAwaiter_1<TResult> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::CompilerServices::INotifyCompletion {
        todo!()
    }
}
