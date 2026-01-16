#[cfg(feature = "cordl_class_Unity+Collections+LowLevel+Unsafe+UnsafeParallelHashMapBucketData")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct UnsafeParallelHashMapBucketData {
    pub values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub keys: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub next: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub buckets: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub bucketCapacityMask: i32,
}
#[cfg(feature = "cordl_class_Unity+Collections+LowLevel+Unsafe+UnsafeParallelHashMapBucketData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Unity::Collections::LowLevel::Unsafe::UnsafeParallelHashMapBucketData
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Collections.LowLevel.Unsafe";
    const CLASS_NAME: &'static str = "UnsafeParallelHashMapBucketData";
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
#[cfg(feature = "cordl_class_Unity+Collections+LowLevel+Unsafe+UnsafeParallelHashMapBucketData")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::Unity::Collections::LowLevel::Unsafe::UnsafeParallelHashMapBucketData
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+LowLevel+Unsafe+UnsafeParallelHashMapBucketData")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::Unity::Collections::LowLevel::Unsafe::UnsafeParallelHashMapBucketData
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
#[cfg(feature = "cordl_class_Unity+Collections+LowLevel+Unsafe+UnsafeParallelHashMapBucketData")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::Unity::Collections::LowLevel::Unsafe::UnsafeParallelHashMapBucketData
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
#[cfg(feature = "cordl_class_Unity+Collections+LowLevel+Unsafe+UnsafeParallelHashMapBucketData")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::Unity::Collections::LowLevel::Unsafe::UnsafeParallelHashMapBucketData
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
#[cfg(feature = "cordl_class_Unity+Collections+LowLevel+Unsafe+UnsafeParallelHashMapBucketData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::Unity::Collections::LowLevel::Unsafe::UnsafeParallelHashMapBucketData
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+UnsafeParallelHashMapBucketData")]
impl crate::Unity::Collections::LowLevel::Unsafe::UnsafeParallelHashMapBucketData {
    pub fn _ctor(
        &mut self,
        v: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        k: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        n: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bcm: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (v, k, n, b, bcm))? };
        Ok(__cordl_ret.into())
    }
}
