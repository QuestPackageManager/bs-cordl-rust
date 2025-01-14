#[cfg(feature = "Unity+Collections+NativeSliceExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct NativeSliceExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Collections+NativeSliceExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Collections::NativeSliceExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "NativeSliceExtensions";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Collections::NativeArray_1<T>, i32, i32),
                crate::Unity::Collections::NativeSlice_1<T>,
                3usize,
            >("Slice")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Slice", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Collections::NativeSlice_1<T> = unsafe {
            method.invoke_unchecked((), (thisArray, start, length))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Collections::NativeSlice_1<T>, i32, i32),
                crate::Unity::Collections::NativeSlice_1<T>,
                3usize,
            >("Slice")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Slice", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Collections::NativeSlice_1<T> = unsafe {
            method.invoke_unchecked((), (thisSlice, start, length))
        };
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
