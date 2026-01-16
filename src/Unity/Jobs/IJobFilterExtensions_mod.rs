#[cfg(feature = "cordl_class_Unity+Jobs+IJobFilterExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct IJobFilterExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Unity+Jobs+IJobFilterExtensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Jobs::IJobFilterExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Jobs";
    const CLASS_NAME: &'static str = "IJobFilterExtensions";
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
#[cfg(feature = "Unity+Jobs+IJobFilterExtensions")]
impl std::ops::Deref for crate::Unity::Jobs::IJobFilterExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Jobs+IJobFilterExtensions")]
impl std::ops::DerefMut for crate::Unity::Jobs::IJobFilterExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Jobs+IJobFilterExtensions")]
impl crate::Unity::Jobs::IJobFilterExtensions {
    #[cfg(feature = "Unity+Jobs+IJobFilterExtensions+JobFilterProducer_1")]
    pub type JobFilterProducer_1<T: quest_hook::libil2cpp::Type> =
        crate::Unity::Jobs::IJobFilterExtensions_JobFilterProducer_1<T>;
    pub fn EarlyJobInit<T>() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>("EarlyJobInit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EarlyJobInit",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetReflectionData<T>() -> quest_hook::libil2cpp::Result<crate::System::IntPtr>
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
                    .find_static_method::<(), crate::System::IntPtr, 0usize>("GetReflectionData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetReflectionData",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::IntPtr =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn RunAppend<T>(
        jobData: T,
        indices: crate::Unity::Collections::NativeList_1<i32>,
        arrayLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
                    .find_static_method::<
                        (T, crate::Unity::Collections::NativeList_1<i32>, i32),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("RunAppend")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RunAppend", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (jobData, indices, arrayLength))? };
        Ok(__cordl_ret.into())
    }
    pub fn RunAppendByRef<T>(
        jobData: quest_hook::libil2cpp::ByRefMut<T>,
        indices: crate::Unity::Collections::NativeList_1<i32>,
        arrayLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
                        quest_hook::libil2cpp::ByRefMut<T>,
                        crate::Unity::Collections::NativeList_1<i32>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>("RunAppendByRef")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RunAppendByRef",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (jobData, indices, arrayLength))? };
        Ok(__cordl_ret.into())
    }
    pub fn RunFilter<T>(
        jobData: T,
        indices: crate::Unity::Collections::NativeList_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
                    .find_static_method::<
                        (T, crate::Unity::Collections::NativeList_1<i32>),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("RunFilter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RunFilter", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (jobData, indices))? };
        Ok(__cordl_ret.into())
    }
    pub fn RunFilterByRef<T>(
        jobData: quest_hook::libil2cpp::ByRefMut<T>,
        indices: crate::Unity::Collections::NativeList_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
                        quest_hook::libil2cpp::ByRefMut<T>,
                        crate::Unity::Collections::NativeList_1<i32>,
                    ), quest_hook::libil2cpp::Void, 2usize>("RunFilterByRef")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RunFilterByRef",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (jobData, indices))? };
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleAppend<T>(
        jobData: T,
        indices: crate::Unity::Collections::NativeList_1<i32>,
        arrayLength: i32,
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
                    ), crate::Unity::Jobs::JobHandle, 4usize>("ScheduleAppend")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ScheduleAppend",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle = unsafe {
            cordl_method_info.invoke_unchecked((), (jobData, indices, arrayLength, dependsOn))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleAppendByRef<T>(
        jobData: quest_hook::libil2cpp::ByRefMut<T>,
        indices: crate::Unity::Collections::NativeList_1<i32>,
        arrayLength: i32,
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
                        quest_hook::libil2cpp::ByRefMut<T>,
                        crate::Unity::Collections::NativeList_1<i32>,
                        i32,
                        crate::Unity::Jobs::JobHandle,
                    ), crate::Unity::Jobs::JobHandle, 4usize>(
                        "ScheduleAppendByRef"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ScheduleAppendByRef",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle = unsafe {
            cordl_method_info.invoke_unchecked((), (jobData, indices, arrayLength, dependsOn))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleFilter<T>(
        jobData: T,
        indices: crate::Unity::Collections::NativeList_1<i32>,
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
                        crate::Unity::Jobs::JobHandle,
                    ), crate::Unity::Jobs::JobHandle, 3usize>("ScheduleFilter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ScheduleFilter",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle =
            unsafe { cordl_method_info.invoke_unchecked((), (jobData, indices, dependsOn))? };
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleFilterByRef<T>(
        jobData: quest_hook::libil2cpp::ByRefMut<T>,
        indices: crate::Unity::Collections::NativeList_1<i32>,
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
                        quest_hook::libil2cpp::ByRefMut<T>,
                        crate::Unity::Collections::NativeList_1<i32>,
                        crate::Unity::Jobs::JobHandle,
                    ), crate::Unity::Jobs::JobHandle, 3usize>(
                        "ScheduleFilterByRef"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ScheduleFilterByRef",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle =
            unsafe { cordl_method_info.invoke_unchecked((), (jobData, indices, dependsOn))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Jobs+IJobFilterExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Jobs::IJobFilterExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_Unity+Jobs+IJobFilterExtensions+JobFilterProducer_1")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct IJobFilterExtensions_JobFilterProducer_1<T: quest_hook::libil2cpp::Type> {
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "cordl_class_Unity+Jobs+IJobFilterExtensions+JobFilterProducer_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
    for crate::Unity::Jobs::IJobFilterExtensions_JobFilterProducer_1<T>
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Jobs";
    const CLASS_NAME: &'static str = "IJobFilterExtensions/JobFilterProducer`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find(
                "Unity.Jobs",
                "IJobFilterExtensions/JobFilterProducer`1",
            )
            .unwrap()
            .make_generic::<(T)>()
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_Unity+Jobs+IJobFilterExtensions+JobFilterProducer_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Argument
    for crate::Unity::Jobs::IJobFilterExtensions_JobFilterProducer_1<T>
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Unity+Jobs+IJobFilterExtensions+JobFilterProducer_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Parameter
    for crate::Unity::Jobs::IJobFilterExtensions_JobFilterProducer_1<T>
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
#[cfg(feature = "cordl_class_Unity+Jobs+IJobFilterExtensions+JobFilterProducer_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Returned
    for crate::Unity::Jobs::IJobFilterExtensions_JobFilterProducer_1<T>
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
#[cfg(feature = "cordl_class_Unity+Jobs+IJobFilterExtensions+JobFilterProducer_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Return
    for crate::Unity::Jobs::IJobFilterExtensions_JobFilterProducer_1<T>
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
#[cfg(feature = "cordl_class_Unity+Jobs+IJobFilterExtensions+JobFilterProducer_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
    for crate::Unity::Jobs::IJobFilterExtensions_JobFilterProducer_1<T>
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Jobs+IJobFilterExtensions+JobFilterProducer_1")]
impl<T: quest_hook::libil2cpp::Type>
    crate::Unity::Jobs::IJobFilterExtensions_JobFilterProducer_1<T>
{
    #[cfg(feature = "Unity+Jobs+IJobFilterExtensions+JobFilterProducer_1+ExecuteJobFunction")]
    pub type ExecuteJobFunction =
        crate::Unity::Jobs::JobFilterProducer_1_IJobFilterExtensions_ExecuteJobFunction<T>;
    #[cfg(feature = "Unity+Jobs+IJobFilterExtensions+JobFilterProducer_1+JobWrapper")]
    pub type JobWrapper =
        crate::Unity::Jobs::JobFilterProducer_1_IJobFilterExtensions_JobWrapper<T>;
    pub fn Execute(
        jobWrapper: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Jobs::JobFilterProducer_1_IJobFilterExtensions_JobWrapper<T>,
        >,
        additionalPtr: crate::System::IntPtr,
        bufferRangePatchData: crate::System::IntPtr,
        ranges: quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::LowLevel::Unsafe::JobRanges>,
        jobIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Jobs::JobFilterProducer_1_IJobFilterExtensions_JobWrapper<
                                T,
                            >,
                        >,
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Jobs::LowLevel::Unsafe::JobRanges,
                        >,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    jobWrapper,
                    additionalPtr,
                    bufferRangePatchData,
                    ranges,
                    jobIndex,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteAppend(
        jobWrapper: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Jobs::JobFilterProducer_1_IJobFilterExtensions_JobWrapper<T>,
        >,
        bufferRangePatchData: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Jobs::JobFilterProducer_1_IJobFilterExtensions_JobWrapper<
                                T,
                            >,
                        >,
                        crate::System::IntPtr,
                    ), quest_hook::libil2cpp::Void, 2usize>("ExecuteAppend")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExecuteAppend",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (jobWrapper, bufferRangePatchData))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteFilter(
        jobWrapper: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Jobs::JobFilterProducer_1_IJobFilterExtensions_JobWrapper<T>,
        >,
        bufferRangePatchData: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Jobs::JobFilterProducer_1_IJobFilterExtensions_JobWrapper<
                                T,
                            >,
                        >,
                        crate::System::IntPtr,
                    ), quest_hook::libil2cpp::Void, 2usize>("ExecuteFilter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExecuteFilter",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (jobWrapper, bufferRangePatchData))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>("Initialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Initialize",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_Unity+Jobs+IJobFilterExtensions+JobFilterProducer_1+ExecuteJobFunction"
)]
#[repr(C)]
#[derive(Debug)]
pub struct JobFilterProducer_1_IJobFilterExtensions_ExecuteJobFunction<
    T: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::MulticastDelegate,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(
    feature = "cordl_class_Unity+Jobs+IJobFilterExtensions+JobFilterProducer_1+ExecuteJobFunction"
)]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
    for crate::Unity::Jobs::JobFilterProducer_1_IJobFilterExtensions_ExecuteJobFunction<T>
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Jobs";
    const CLASS_NAME: &'static str = "IJobFilterExtensions/JobFilterProducer`1/ExecuteJobFunction";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find(
                "Unity.Jobs",
                "IJobFilterExtensions/JobFilterProducer`1/ExecuteJobFunction",
            )
            .unwrap()
            .make_generic::<(T)>()
            .unwrap()
            .unwrap()
        })
    }
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
#[cfg(feature = "Unity+Jobs+IJobFilterExtensions+JobFilterProducer_1+ExecuteJobFunction")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
    for crate::Unity::Jobs::JobFilterProducer_1_IJobFilterExtensions_ExecuteJobFunction<T>
{
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Jobs+IJobFilterExtensions+JobFilterProducer_1+ExecuteJobFunction")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
    for crate::Unity::Jobs::JobFilterProducer_1_IJobFilterExtensions_ExecuteJobFunction<T>
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Jobs+IJobFilterExtensions+JobFilterProducer_1+ExecuteJobFunction")]
impl<T: quest_hook::libil2cpp::Type>
    crate::Unity::Jobs::JobFilterProducer_1_IJobFilterExtensions_ExecuteJobFunction<T>
{
    pub fn BeginInvoke(
        &mut self,
        jobWrapper: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Jobs::JobFilterProducer_1_IJobFilterExtensions_JobWrapper<T>,
        >,
        additionalPtr: crate::System::IntPtr,
        bufferRangePatchData: crate::System::IntPtr,
        ranges: quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::LowLevel::Unsafe::JobRanges>,
        jobIndex: i32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Jobs::JobFilterProducer_1_IJobFilterExtensions_JobWrapper<
                                T,
                            >,
                        >,
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Jobs::LowLevel::Unsafe::JobRanges,
                        >,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>, 7usize>(
                        "BeginInvoke",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BeginInvoke",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    jobWrapper,
                    additionalPtr,
                    bufferRangePatchData,
                    ranges,
                    jobIndex,
                    callback,
                    object,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        jobWrapper: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Jobs::JobFilterProducer_1_IJobFilterExtensions_JobWrapper<T>,
        >,
        ranges: quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::LowLevel::Unsafe::JobRanges>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Jobs::JobFilterProducer_1_IJobFilterExtensions_JobWrapper<
                                T,
                            >,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Jobs::LowLevel::Unsafe::JobRanges,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                    ), quest_hook::libil2cpp::Void, 3usize>("EndInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EndInvoke",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (jobWrapper, ranges, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        jobWrapper: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Jobs::JobFilterProducer_1_IJobFilterExtensions_JobWrapper<T>,
        >,
        additionalPtr: crate::System::IntPtr,
        bufferRangePatchData: crate::System::IntPtr,
        ranges: quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::LowLevel::Unsafe::JobRanges>,
        jobIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Jobs::JobFilterProducer_1_IJobFilterExtensions_JobWrapper<
                                T,
                            >,
                        >,
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Jobs::LowLevel::Unsafe::JobRanges,
                        >,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Invoke",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    jobWrapper,
                    additionalPtr,
                    bufferRangePatchData,
                    ranges,
                    jobIndex,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
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
#[cfg(
    feature = "cordl_class_Unity+Jobs+IJobFilterExtensions+JobFilterProducer_1+ExecuteJobFunction"
)]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
    for crate::Unity::Jobs::JobFilterProducer_1_IJobFilterExtensions_ExecuteJobFunction<T>
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_Unity+Jobs+IJobFilterExtensions+JobFilterProducer_1+JobWrapper")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct JobFilterProducer_1_IJobFilterExtensions_JobWrapper<T: quest_hook::libil2cpp::Type> {
    pub outputIndices: crate::Unity::Collections::NativeList_1<i32>,
    pub appendCount: i32,
    pub JobData: T,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "cordl_class_Unity+Jobs+IJobFilterExtensions+JobFilterProducer_1+JobWrapper")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
    for crate::Unity::Jobs::JobFilterProducer_1_IJobFilterExtensions_JobWrapper<T>
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Jobs";
    const CLASS_NAME: &'static str = "IJobFilterExtensions/JobFilterProducer`1/JobWrapper";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find(
                "Unity.Jobs",
                "IJobFilterExtensions/JobFilterProducer`1/JobWrapper",
            )
            .unwrap()
            .make_generic::<(T)>()
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_Unity+Jobs+IJobFilterExtensions+JobFilterProducer_1+JobWrapper")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Argument
    for crate::Unity::Jobs::JobFilterProducer_1_IJobFilterExtensions_JobWrapper<T>
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Unity+Jobs+IJobFilterExtensions+JobFilterProducer_1+JobWrapper")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Parameter
    for crate::Unity::Jobs::JobFilterProducer_1_IJobFilterExtensions_JobWrapper<T>
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
#[cfg(feature = "cordl_class_Unity+Jobs+IJobFilterExtensions+JobFilterProducer_1+JobWrapper")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Returned
    for crate::Unity::Jobs::JobFilterProducer_1_IJobFilterExtensions_JobWrapper<T>
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
#[cfg(feature = "cordl_class_Unity+Jobs+IJobFilterExtensions+JobFilterProducer_1+JobWrapper")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Return
    for crate::Unity::Jobs::JobFilterProducer_1_IJobFilterExtensions_JobWrapper<T>
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
#[cfg(feature = "cordl_class_Unity+Jobs+IJobFilterExtensions+JobFilterProducer_1+JobWrapper")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
    for crate::Unity::Jobs::JobFilterProducer_1_IJobFilterExtensions_JobWrapper<T>
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Jobs+IJobFilterExtensions+JobFilterProducer_1+JobWrapper")]
impl<T: quest_hook::libil2cpp::Type>
    crate::Unity::Jobs::JobFilterProducer_1_IJobFilterExtensions_JobWrapper<T>
{
}
