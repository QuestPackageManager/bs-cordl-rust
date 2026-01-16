#[cfg(feature = "cordl_class_Unity+Collections+LowLevel+Unsafe+NativeSliceUnsafeUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct NativeSliceUnsafeUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Unity+Collections+LowLevel+Unsafe+NativeSliceUnsafeUtility")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Unity::Collections::LowLevel::Unsafe::NativeSliceUnsafeUtility
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Collections.LowLevel.Unsafe";
    const CLASS_NAME: &'static str = "NativeSliceUnsafeUtility";
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
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+NativeSliceUnsafeUtility")]
impl std::ops::Deref for crate::Unity::Collections::LowLevel::Unsafe::NativeSliceUnsafeUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+NativeSliceUnsafeUtility")]
impl std::ops::DerefMut for crate::Unity::Collections::LowLevel::Unsafe::NativeSliceUnsafeUtility {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        i32,
                        i32,
                    ), crate::Unity::Collections::NativeSlice_1<T>, 3usize>(
                        "ConvertExistingDataToNativeSlice",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ConvertExistingDataToNativeSlice",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeSlice_1<T> =
            unsafe { cordl_method_info.invoke_unchecked((), (dataPointer, stride, length))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetUnsafePtr<T>(
        nativeSlice: crate::Unity::Collections::NativeSlice_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>>
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
                        (crate::Unity::Collections::NativeSlice_1<T>),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        1usize,
                    >("GetUnsafePtr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetUnsafePtr", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject> =
            unsafe { cordl_method_info.invoke_unchecked((), (nativeSlice))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetUnsafeReadOnlyPtr<T>(
        nativeSlice: crate::Unity::Collections::NativeSlice_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>>
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
                        (crate::Unity::Collections::NativeSlice_1<T>),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        1usize,
                    >("GetUnsafeReadOnlyPtr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetUnsafeReadOnlyPtr", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject> =
            unsafe { cordl_method_info.invoke_unchecked((), (nativeSlice))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+LowLevel+Unsafe+NativeSliceUnsafeUtility")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Unity::Collections::LowLevel::Unsafe::NativeSliceUnsafeUtility
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
