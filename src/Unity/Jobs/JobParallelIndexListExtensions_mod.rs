#[cfg(feature = "cordl_class_Unity+Jobs+JobParallelIndexListExtensions")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct JobParallelIndexListExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Unity+Jobs+JobParallelIndexListExtensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Jobs::JobParallelIndexListExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Jobs";
    const CLASS_NAME: &'static str = "JobParallelIndexListExtensions";
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
#[cfg(feature = "Unity+Jobs+JobParallelIndexListExtensions")]
impl std::ops::Deref for crate::Unity::Jobs::JobParallelIndexListExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Jobs+JobParallelIndexListExtensions")]
impl std::ops::DerefMut for crate::Unity::Jobs::JobParallelIndexListExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Jobs+JobParallelIndexListExtensions")]
impl crate::Unity::Jobs::JobParallelIndexListExtensions {
    pub fn ScheduleAppend<T>(
        jobData: T,
        indices: crate::Unity::Collections::NativeList_1<i32>,
        arrayLength: i32,
        innerloopBatchCount: i32,
        dependsOn: crate::Unity::Jobs::JobHandle,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        T,
                        crate::Unity::Collections::NativeList_1<i32>,
                        i32,
                        i32,
                        crate::Unity::Jobs::JobHandle,
                    ), crate::Unity::Jobs::JobHandle, 5usize>("ScheduleAppend")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ScheduleAppend",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    jobData,
                    indices,
                    arrayLength,
                    innerloopBatchCount,
                    dependsOn,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleFilter<T>(
        jobData: T,
        indices: crate::Unity::Collections::NativeList_1<i32>,
        innerloopBatchCount: i32,
        dependsOn: crate::Unity::Jobs::JobHandle,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        T,
                        crate::Unity::Collections::NativeList_1<i32>,
                        i32,
                        crate::Unity::Jobs::JobHandle,
                    ), crate::Unity::Jobs::JobHandle, 4usize>("ScheduleFilter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ScheduleFilter",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle = unsafe {
            cordl_method_info
                .invoke_unchecked((), (jobData, indices, innerloopBatchCount, dependsOn))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Jobs+JobParallelIndexListExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Jobs::JobParallelIndexListExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
