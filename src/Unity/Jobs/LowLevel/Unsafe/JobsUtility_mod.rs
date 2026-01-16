#[cfg(feature = "cordl_class_Unity+Jobs+LowLevel+Unsafe+JobsUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct JobsUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Unity+Jobs+LowLevel+Unsafe+JobsUtility")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Jobs.LowLevel.Unsafe";
    const CLASS_NAME: &'static str = "JobsUtility";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobsUtility")]
impl std::ops::Deref for crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobsUtility")]
impl std::ops::DerefMut for crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobsUtility")]
impl crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility {
    #[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobsUtility+JobScheduleParameters")]
    pub type JobScheduleParameters =
        crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters;
    #[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobsUtility+PanicFunction_")]
    pub type PanicFunction_ = crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_PanicFunction_;
    pub fn CreateJobReflectionData_Il2CppObject_Il2CppObject1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        managedJobFunction0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        managedJobFunction1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        managedJobFunction2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), crate::System::IntPtr, 4usize>("CreateJobReflectionData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateJobReflectionData",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _cordl_type,
                    managedJobFunction0,
                    managedJobFunction1,
                    managedJobFunction2,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateJobReflectionData_Type2(
        wrapperJobType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        userJobType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        managedJobFunction0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), crate::System::IntPtr, 3usize>("CreateJobReflectionData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateJobReflectionData",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            cordl_method_info
                .invoke_unchecked((), (wrapperJobType, userJobType, managedJobFunction0))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateJobReflectionData_Type_Il2CppObject_Il2CppObject0(
        wrapperJobType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        userJobType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        managedJobFunction0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        managedJobFunction1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        managedJobFunction2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), crate::System::IntPtr, 5usize>("CreateJobReflectionData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateJobReflectionData",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    wrapperJobType,
                    userJobType,
                    managedJobFunction0,
                    managedJobFunction1,
                    managedJobFunction2,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetJobQueueWorkerThreadCount() -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), i32, 0usize>("GetJobQueueWorkerThreadCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetJobQueueWorkerThreadCount",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetJobRange(
        ranges: quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::LowLevel::Unsafe::JobRanges>,
        jobIndex: i32,
        beginIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        endIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Jobs::LowLevel::Unsafe::JobRanges,
                        >,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), quest_hook::libil2cpp::Void, 4usize>("GetJobRange")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetJobRange",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (ranges, jobIndex, beginIndex, endIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetWorkStealingRange(
        ranges: quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::LowLevel::Unsafe::JobRanges>,
        jobIndex: i32,
        beginIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        endIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Jobs::LowLevel::Unsafe::JobRanges,
                        >,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), bool, 4usize>("GetWorkStealingRange")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetWorkStealingRange",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (ranges, jobIndex, beginIndex, endIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InvokePanicFunction() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "InvokePanicFunction",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InvokePanicFunction",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Schedule(
        parameters: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters,
                    >), crate::Unity::Jobs::JobHandle, 1usize>("Schedule")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Schedule",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle =
            unsafe { cordl_method_info.invoke_unchecked((), (parameters))? };
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleParallelFor(
        parameters: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters,
        >,
        arrayLength: i32,
        innerloopBatchCount: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters,
                        >,
                        i32,
                        i32,
                    ), crate::Unity::Jobs::JobHandle, 3usize>(
                        "ScheduleParallelFor"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ScheduleParallelFor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle = unsafe {
            cordl_method_info
                .invoke_unchecked((), (parameters, arrayLength, innerloopBatchCount))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleParallelForDeferArraySize(
        parameters: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters,
        >,
        innerloopBatchCount: i32,
        listData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        listDataAtomicSafetyHandle: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters,
                        >,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), crate::Unity::Jobs::JobHandle, 4usize>(
                        "ScheduleParallelForDeferArraySize"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ScheduleParallelForDeferArraySize",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    parameters,
                    innerloopBatchCount,
                    listData,
                    listDataAtomicSafetyHandle,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleParallelForDeferArraySize_Injected(
        parameters: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters,
        >,
        innerloopBatchCount: i32,
        listData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        listDataAtomicSafetyHandle: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters,
                        >,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "ScheduleParallelForDeferArraySize_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ScheduleParallelForDeferArraySize_Injected",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    parameters,
                    innerloopBatchCount,
                    listData,
                    listDataAtomicSafetyHandle,
                    ret,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleParallelForTransform(
        parameters: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters,
        >,
        transfromAccesssArray: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters,
                        >,
                        crate::System::IntPtr,
                    ), crate::Unity::Jobs::JobHandle, 2usize>(
                        "ScheduleParallelForTransform"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ScheduleParallelForTransform",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle =
            unsafe { cordl_method_info.invoke_unchecked((), (parameters, transfromAccesssArray))? };
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleParallelForTransform_Injected(
        parameters: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters,
        >,
        transfromAccesssArray: crate::System::IntPtr,
        ret: quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters,
                        >,
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "ScheduleParallelForTransform_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ScheduleParallelForTransform_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (parameters, transfromAccesssArray, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleParallelFor_Injected(
        parameters: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters,
        >,
        arrayLength: i32,
        innerloopBatchCount: i32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters,
                        >,
                        i32,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "ScheduleParallelFor_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ScheduleParallelFor_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (parameters, arrayLength, innerloopBatchCount, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Schedule_Injected(
        parameters: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters,
        >,
        ret: quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters,
                        >,
                        quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
                    ), quest_hook::libil2cpp::Void, 2usize>("Schedule_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Schedule_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (parameters, ret))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsExecutingJob() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>("get_IsExecutingJob")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_IsExecutingJob",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_JobWorkerCount() -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), i32, 0usize>("get_JobWorkerCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_JobWorkerCount",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ThreadIndex() -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), i32, 0usize>("get_ThreadIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_ThreadIndex",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ThreadIndexCount() -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), i32, 0usize>("get_ThreadIndexCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_ThreadIndexCount",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_JobCompilerEnabled(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "set_JobCompilerEnabled",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_JobCompilerEnabled",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Jobs+LowLevel+Unsafe+JobsUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_Unity+Jobs+LowLevel+Unsafe+JobsUtility+JobScheduleParameters")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct JobsUtility_JobScheduleParameters {
    pub Dependency: crate::Unity::Jobs::JobHandle,
    pub ScheduleMode: i32,
    pub ReflectionData: crate::System::IntPtr,
    pub JobDataPtr: crate::System::IntPtr,
}
#[cfg(feature = "cordl_class_Unity+Jobs+LowLevel+Unsafe+JobsUtility+JobScheduleParameters")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Jobs.LowLevel.Unsafe";
    const CLASS_NAME: &'static str = "JobsUtility/JobScheduleParameters";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_Unity+Jobs+LowLevel+Unsafe+JobsUtility+JobScheduleParameters")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Unity+Jobs+LowLevel+Unsafe+JobsUtility+JobScheduleParameters")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_Unity+Jobs+LowLevel+Unsafe+JobsUtility+JobScheduleParameters")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_Unity+Jobs+LowLevel+Unsafe+JobsUtility+JobScheduleParameters")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_Unity+Jobs+LowLevel+Unsafe+JobsUtility+JobScheduleParameters")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobsUtility+JobScheduleParameters")]
impl crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters {
    pub fn _ctor(
        &mut self,
        i_jobData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        i_reflectionData: crate::System::IntPtr,
        i_dependency: crate::Unity::Jobs::JobHandle,
        i_scheduleMode: crate::Unity::Jobs::LowLevel::Unsafe::ScheduleMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        crate::System::IntPtr,
                        crate::Unity::Jobs::JobHandle,
                        crate::Unity::Jobs::LowLevel::Unsafe::ScheduleMode,
                    ), quest_hook::libil2cpp::Void, 4usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (i_jobData, i_reflectionData, i_dependency, i_scheduleMode),
            )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Jobs+LowLevel+Unsafe+JobsUtility+PanicFunction_")]
#[repr(C)]
#[derive(Debug)]
pub struct JobsUtility_PanicFunction_ {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "cordl_class_Unity+Jobs+LowLevel+Unsafe+JobsUtility+PanicFunction_")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_PanicFunction_
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Jobs.LowLevel.Unsafe";
    const CLASS_NAME: &'static str = "JobsUtility/PanicFunction_";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobsUtility+PanicFunction_")]
impl std::ops::Deref for crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_PanicFunction_ {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobsUtility+PanicFunction_")]
impl std::ops::DerefMut for crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_PanicFunction_ {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobsUtility+PanicFunction_")]
impl crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_PanicFunction_ {
    pub fn Invoke(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Invoke",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        crate::System::IntPtr,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (object, method))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Jobs+LowLevel+Unsafe+JobsUtility+PanicFunction_")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_PanicFunction_
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
