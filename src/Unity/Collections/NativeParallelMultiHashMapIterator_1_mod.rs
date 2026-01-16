#[cfg(feature = "cordl_class_Unity+Collections+NativeParallelMultiHashMapIterator_1")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct NativeParallelMultiHashMapIterator_1<TKey: quest_hook::libil2cpp::Type> {
    pub key: TKey,
    pub NextEntryIndex: i32,
    pub EntryIndex: i32,
    __cordl_phantom_TKey: std::marker::PhantomData<TKey>,
}
#[cfg(feature = "cordl_class_Unity+Collections+NativeParallelMultiHashMapIterator_1")]
unsafe impl<TKey: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
    for crate::Unity::Collections::NativeParallelMultiHashMapIterator_1<TKey>
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "NativeParallelMultiHashMapIterator`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find(
                "Unity.Collections",
                "NativeParallelMultiHashMapIterator`1",
            )
            .unwrap()
            .make_generic::<(TKey)>()
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
#[cfg(feature = "cordl_class_Unity+Collections+NativeParallelMultiHashMapIterator_1")]
unsafe impl<TKey: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Argument
    for crate::Unity::Collections::NativeParallelMultiHashMapIterator_1<TKey>
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+NativeParallelMultiHashMapIterator_1")]
unsafe impl<TKey: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Parameter
    for crate::Unity::Collections::NativeParallelMultiHashMapIterator_1<TKey>
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
#[cfg(feature = "cordl_class_Unity+Collections+NativeParallelMultiHashMapIterator_1")]
unsafe impl<TKey: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Returned
    for crate::Unity::Collections::NativeParallelMultiHashMapIterator_1<TKey>
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
#[cfg(feature = "cordl_class_Unity+Collections+NativeParallelMultiHashMapIterator_1")]
unsafe impl<TKey: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Return
    for crate::Unity::Collections::NativeParallelMultiHashMapIterator_1<TKey>
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
#[cfg(feature = "cordl_class_Unity+Collections+NativeParallelMultiHashMapIterator_1")]
unsafe impl<TKey: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
    for crate::Unity::Collections::NativeParallelMultiHashMapIterator_1<TKey>
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Collections+NativeParallelMultiHashMapIterator_1")]
impl<TKey: quest_hook::libil2cpp::Type>
    crate::Unity::Collections::NativeParallelMultiHashMapIterator_1<TKey>
{
    pub fn GetEntryIndex(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TKey: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetEntryIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetEntryIndex",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
