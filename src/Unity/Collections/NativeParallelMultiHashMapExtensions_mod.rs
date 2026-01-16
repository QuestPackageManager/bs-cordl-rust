#[cfg(feature = "cordl_class_Unity+Collections+NativeParallelMultiHashMapExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct NativeParallelMultiHashMapExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Unity+Collections+NativeParallelMultiHashMapExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Collections::NativeParallelMultiHashMapExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "NativeParallelMultiHashMapExtensions";
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
#[cfg(feature = "Unity+Collections+NativeParallelMultiHashMapExtensions")]
impl std::ops::Deref
for crate::Unity::Collections::NativeParallelMultiHashMapExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+NativeParallelMultiHashMapExtensions")]
impl std::ops::DerefMut
for crate::Unity::Collections::NativeParallelMultiHashMapExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+NativeParallelMultiHashMapExtensions")]
impl crate::Unity::Collections::NativeParallelMultiHashMapExtensions {
    pub fn Initialize<TKey, TValue, U>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelMultiHashMap_2<TKey, TValue>,
        >,
        capacity: i32,
        allocator: quest_hook::libil2cpp::ByRefMut<U>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::NativeParallelMultiHashMap_2<
                                    TKey,
                                    TValue,
                                >,
                            >,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<U>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("Initialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (container, capacity, allocator))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+NativeParallelMultiHashMapExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Collections::NativeParallelMultiHashMapExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
