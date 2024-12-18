#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+BurstRuntime")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstRuntime {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+BurstRuntime")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Collections::LowLevel::Unsafe::BurstRuntime =>
    "Unity.Collections.LowLevel.Unsafe"."BurstRuntime"
);
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+BurstRuntime")]
impl std::ops::Deref for crate::Unity::Collections::LowLevel::Unsafe::BurstRuntime {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+BurstRuntime")]
impl std::ops::DerefMut for crate::Unity::Collections::LowLevel::Unsafe::BurstRuntime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+BurstRuntime")]
impl crate::Unity::Collections::LowLevel::Unsafe::BurstRuntime {
    #[cfg(feature = "Unity+Collections+LowLevel+Unsafe+BurstRuntime+HashCode64_1")]
    pub type HashCode64_1<T: quest_hook::libil2cpp::Type> = crate::Unity::Collections::LowLevel::Unsafe::BurstRuntime_HashCode64_1<
        T,
    >;
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+BurstRuntime")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Collections::LowLevel::Unsafe::BurstRuntime {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+BurstRuntime+HashCode64_1")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BurstRuntime_HashCode64_1<T: quest_hook::libil2cpp::Type> {
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+BurstRuntime+HashCode64_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Collections::LowLevel::Unsafe::BurstRuntime_HashCode64_1 < T > =>
    "Unity.Collections.LowLevel.Unsafe"."BurstRuntime/HashCode64`1<T>" < T >
);
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+BurstRuntime+HashCode64_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::Unity::Collections::LowLevel::Unsafe::BurstRuntime_HashCode64_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+BurstRuntime+HashCode64_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::Unity::Collections::LowLevel::Unsafe::BurstRuntime_HashCode64_1<T> {}
