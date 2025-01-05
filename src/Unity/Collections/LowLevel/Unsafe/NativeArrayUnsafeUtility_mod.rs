#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+NativeArrayUnsafeUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct NativeArrayUnsafeUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+NativeArrayUnsafeUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Collections::LowLevel::Unsafe::NativeArrayUnsafeUtility =>
    "Unity.Collections.LowLevel.Unsafe"."NativeArrayUnsafeUtility"
);
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+NativeArrayUnsafeUtility")]
impl std::ops::Deref
for crate::Unity::Collections::LowLevel::Unsafe::NativeArrayUnsafeUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+NativeArrayUnsafeUtility")]
impl std::ops::DerefMut
for crate::Unity::Collections::LowLevel::Unsafe::NativeArrayUnsafeUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+NativeArrayUnsafeUtility")]
impl crate::Unity::Collections::LowLevel::Unsafe::NativeArrayUnsafeUtility {
    pub fn ConvertExistingDataToNativeArray<T>(
        dataPointer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i32,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeArray_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ConvertExistingDataToNativeArray",
                (dataPointer, length, allocator),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUnsafeBufferPointerWithoutChecks<T>(
        nativeArray: crate::Unity::Collections::NativeArray_1<T>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUnsafeBufferPointerWithoutChecks", (nativeArray))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUnsafePtr<T>(
        nativeArray: crate::Unity::Collections::NativeArray_1<T>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUnsafePtr", (nativeArray))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUnsafeReadOnlyPtr<T>(
        nativeArray: crate::Unity::Collections::NativeArray_1<T>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUnsafeReadOnlyPtr", (nativeArray))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+NativeArrayUnsafeUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Collections::LowLevel::Unsafe::NativeArrayUnsafeUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
