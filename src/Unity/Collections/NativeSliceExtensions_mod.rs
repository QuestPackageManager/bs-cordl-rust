#[cfg(feature = "Unity+Collections+NativeSliceExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct NativeSliceExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Collections+NativeSliceExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Collections::NativeSliceExtensions =>
    "Unity.Collections"."NativeSliceExtensions"
);
#[cfg(feature = "Unity+Collections+NativeSliceExtensions")]
impl std::ops::Deref for crate::Unity::Collections::NativeSliceExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+NativeSliceExtensions")]
impl std::ops::DerefMut for crate::Unity::Collections::NativeSliceExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+NativeSliceExtensions")]
impl crate::Unity::Collections::NativeSliceExtensions {
    pub fn Slice_NativeArray_1_0<T>(
        thisArray: crate::Unity::Collections::NativeArray_1<T>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeSlice_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::Unity::Collections::NativeSlice_1<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Slice", (thisArray, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn Slice_NativeSlice_1_1<T>(
        thisSlice: crate::Unity::Collections::NativeSlice_1<T>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeSlice_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::Unity::Collections::NativeSlice_1<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Slice", (thisSlice, start, length))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Collections+NativeSliceExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Collections::NativeSliceExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
