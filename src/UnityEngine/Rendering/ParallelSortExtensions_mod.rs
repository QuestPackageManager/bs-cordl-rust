#[cfg(feature = "cordl_class_UnityEngine+Rendering+ParallelSortExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct ParallelSortExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ParallelSortExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::ParallelSortExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "ParallelSortExtensions";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+Rendering+ParallelSortExtensions")]
impl std::ops::Deref for crate::UnityEngine::Rendering::ParallelSortExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ParallelSortExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::ParallelSortExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ParallelSortExtensions")]
impl crate::UnityEngine::Rendering::ParallelSortExtensions {
    pub const kMinRadixSortArraySize: i32 = 2048i32;
    pub const kMinRadixSortBatchSize: i32 = 256i32;
    #[cfg(
        feature = "UnityEngine+Rendering+ParallelSortExtensions+RadixSortBatchPrefixSumJob"
    )]
    pub type RadixSortBatchPrefixSumJob = crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortBatchPrefixSumJob;
    #[cfg(
        feature = "UnityEngine+Rendering+ParallelSortExtensions+RadixSortBucketCountJob"
    )]
    pub type RadixSortBucketCountJob = crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortBucketCountJob;
    #[cfg(
        feature = "UnityEngine+Rendering+ParallelSortExtensions+RadixSortBucketSortJob"
    )]
    pub type RadixSortBucketSortJob = crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortBucketSortJob;
    #[cfg(
        feature = "UnityEngine+Rendering+ParallelSortExtensions+RadixSortPrefixSumJob"
    )]
    pub type RadixSortPrefixSumJob = crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortPrefixSumJob;
    pub fn ParallelSort(
        array: crate::Unity::Collections::NativeArray_1<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::NativeArray_1<i32>),
                        crate::Unity::Jobs::JobHandle,
                        1usize,
                    >("ParallelSort")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ParallelSort", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle = unsafe {
            cordl_method_info.invoke_unchecked((), (array))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ParallelSort_g__Swap_2_0(
        a: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeArray_1<i32>,
        >,
        b: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeArray_1<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::NativeArray_1<i32>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::NativeArray_1<i32>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("<ParallelSort>g__Swap|2_0")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "<ParallelSort>g__Swap|2_0", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (a, b))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ParallelSortExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::ParallelSortExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ParallelSortExtensions+RadixSortBatchPrefixSumJob"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ParallelSortExtensions_RadixSortBatchPrefixSumJob {
    pub radix: i32,
    pub jobsCount: i32,
    pub array: crate::Unity::Collections::NativeArray_1<i32>,
    pub counter: crate::Unity::Collections::NativeArray_1<i32>,
    pub indicesSum: crate::Unity::Collections::NativeArray_1<i32>,
    pub buckets: crate::Unity::Collections::NativeArray_1<i32>,
    pub indices: crate::Unity::Collections::NativeArray_1<i32>,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ParallelSortExtensions+RadixSortBatchPrefixSumJob"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortBatchPrefixSumJob {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "ParallelSortExtensions/RadixSortBatchPrefixSumJob";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ParallelSortExtensions+RadixSortBatchPrefixSumJob"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortBatchPrefixSumJob {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ParallelSortExtensions+RadixSortBatchPrefixSumJob"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortBatchPrefixSumJob {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ParallelSortExtensions+RadixSortBatchPrefixSumJob"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortBatchPrefixSumJob {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ParallelSortExtensions+RadixSortBatchPrefixSumJob"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortBatchPrefixSumJob {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ParallelSortExtensions+RadixSortBatchPrefixSumJob"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortBatchPrefixSumJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+ParallelSortExtensions+RadixSortBatchPrefixSumJob"
)]
impl crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortBatchPrefixSumJob {
    pub fn AtomicIncrement(
        counter: crate::Unity::Collections::NativeArray_1<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::NativeArray_1<i32>),
                        i32,
                        1usize,
                    >("AtomicIncrement")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AtomicIncrement", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (counter))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Execute(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Execute",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn JobIndexPrefixSum(
        &mut self,
        sum: i32,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, i32), i32, 2usize>("JobIndexPrefixSum")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "JobIndexPrefixSum", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (sum, i))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+ParallelSortExtensions+RadixSortBatchPrefixSumJob"
)]
impl AsRef<crate::Unity::Jobs::IJobFor>
for crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortBatchPrefixSumJob {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobFor {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+ParallelSortExtensions+RadixSortBatchPrefixSumJob"
)]
impl AsMut<crate::Unity::Jobs::IJobFor>
for crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortBatchPrefixSumJob {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobFor {
        todo!()
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ParallelSortExtensions+RadixSortBucketCountJob"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ParallelSortExtensions_RadixSortBucketCountJob {
    pub radix: i32,
    pub jobsCount: i32,
    pub batchSize: i32,
    pub array: crate::Unity::Collections::NativeArray_1<i32>,
    pub buckets: crate::Unity::Collections::NativeArray_1<i32>,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ParallelSortExtensions+RadixSortBucketCountJob"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortBucketCountJob {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "ParallelSortExtensions/RadixSortBucketCountJob";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ParallelSortExtensions+RadixSortBucketCountJob"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortBucketCountJob {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ParallelSortExtensions+RadixSortBucketCountJob"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortBucketCountJob {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ParallelSortExtensions+RadixSortBucketCountJob"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortBucketCountJob {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ParallelSortExtensions+RadixSortBucketCountJob"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortBucketCountJob {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ParallelSortExtensions+RadixSortBucketCountJob"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortBucketCountJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ParallelSortExtensions+RadixSortBucketCountJob")]
impl crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortBucketCountJob {
    pub fn Execute(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Execute",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (index))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+ParallelSortExtensions+RadixSortBucketCountJob")]
impl AsRef<crate::Unity::Jobs::IJobFor>
for crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortBucketCountJob {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobFor {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+ParallelSortExtensions+RadixSortBucketCountJob")]
impl AsMut<crate::Unity::Jobs::IJobFor>
for crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortBucketCountJob {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobFor {
        todo!()
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ParallelSortExtensions+RadixSortBucketSortJob"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ParallelSortExtensions_RadixSortBucketSortJob {
    pub radix: i32,
    pub batchSize: i32,
    pub array: crate::Unity::Collections::NativeArray_1<i32>,
    pub indices: crate::Unity::Collections::NativeArray_1<i32>,
    pub arraySorted: crate::Unity::Collections::NativeArray_1<i32>,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ParallelSortExtensions+RadixSortBucketSortJob"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortBucketSortJob {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "ParallelSortExtensions/RadixSortBucketSortJob";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ParallelSortExtensions+RadixSortBucketSortJob"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortBucketSortJob {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ParallelSortExtensions+RadixSortBucketSortJob"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortBucketSortJob {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ParallelSortExtensions+RadixSortBucketSortJob"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortBucketSortJob {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ParallelSortExtensions+RadixSortBucketSortJob"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortBucketSortJob {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ParallelSortExtensions+RadixSortBucketSortJob"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortBucketSortJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ParallelSortExtensions+RadixSortBucketSortJob")]
impl crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortBucketSortJob {
    pub fn Execute(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Execute",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (index))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+ParallelSortExtensions+RadixSortBucketSortJob")]
impl AsRef<crate::Unity::Jobs::IJobFor>
for crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortBucketSortJob {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobFor {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+ParallelSortExtensions+RadixSortBucketSortJob")]
impl AsMut<crate::Unity::Jobs::IJobFor>
for crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortBucketSortJob {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobFor {
        todo!()
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ParallelSortExtensions+RadixSortPrefixSumJob"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ParallelSortExtensions_RadixSortPrefixSumJob {
    pub jobsCount: i32,
    pub indicesSum: crate::Unity::Collections::NativeArray_1<i32>,
    pub indices: crate::Unity::Collections::NativeArray_1<i32>,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ParallelSortExtensions+RadixSortPrefixSumJob"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortPrefixSumJob {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "ParallelSortExtensions/RadixSortPrefixSumJob";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ParallelSortExtensions+RadixSortPrefixSumJob"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortPrefixSumJob {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ParallelSortExtensions+RadixSortPrefixSumJob"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortPrefixSumJob {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ParallelSortExtensions+RadixSortPrefixSumJob"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortPrefixSumJob {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ParallelSortExtensions+RadixSortPrefixSumJob"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortPrefixSumJob {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ParallelSortExtensions+RadixSortPrefixSumJob"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortPrefixSumJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ParallelSortExtensions+RadixSortPrefixSumJob")]
impl crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortPrefixSumJob {
    pub fn Execute(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Execute",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (index))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+ParallelSortExtensions+RadixSortPrefixSumJob")]
impl AsRef<crate::Unity::Jobs::IJobFor>
for crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortPrefixSumJob {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobFor {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+ParallelSortExtensions+RadixSortPrefixSumJob")]
impl AsMut<crate::Unity::Jobs::IJobFor>
for crate::UnityEngine::Rendering::ParallelSortExtensions_RadixSortPrefixSumJob {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobFor {
        todo!()
    }
}
