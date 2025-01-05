#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+NativeSliceUnsafeUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct NativeSliceUnsafeUtility {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+NativeSliceUnsafeUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Collections::LowLevel::Unsafe::NativeSliceUnsafeUtility =>
    "Unity.Collections.LowLevel.Unsafe"."NativeSliceUnsafeUtility"
);
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+NativeSliceUnsafeUtility")]
impl std::ops::Deref
for crate::Unity::Collections::LowLevel::Unsafe::NativeSliceUnsafeUtility {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+NativeSliceUnsafeUtility")]
impl std::ops::DerefMut
for crate::Unity::Collections::LowLevel::Unsafe::NativeSliceUnsafeUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+NativeSliceUnsafeUtility")]
impl crate::Unity::Collections::LowLevel::Unsafe::NativeSliceUnsafeUtility {
    pub fn ConvertExistingDataToNativeSlice<T>(
        dataPointer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        stride: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeSlice_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::Unity::Collections::NativeSlice_1<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertExistingDataToNativeSlice", (dataPointer, stride, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUnsafePtr<T>(
        nativeSlice: crate::Unity::Collections::NativeSlice_1<T>,
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
            .invoke("GetUnsafePtr", (nativeSlice))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUnsafeReadOnlyPtr<T>(
        nativeSlice: crate::Unity::Collections::NativeSlice_1<T>,
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
            .invoke("GetUnsafeReadOnlyPtr", (nativeSlice))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+NativeSliceUnsafeUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Collections::LowLevel::Unsafe::NativeSliceUnsafeUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
