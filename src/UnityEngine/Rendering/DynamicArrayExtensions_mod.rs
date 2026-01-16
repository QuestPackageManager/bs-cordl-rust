#[cfg(feature = "cordl_class_UnityEngine+Rendering+DynamicArrayExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct DynamicArrayExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+DynamicArrayExtensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::DynamicArrayExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "DynamicArrayExtensions";
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
#[cfg(feature = "UnityEngine+Rendering+DynamicArrayExtensions")]
impl std::ops::Deref for crate::UnityEngine::Rendering::DynamicArrayExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+DynamicArrayExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::DynamicArrayExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+DynamicArrayExtensions")]
impl crate::UnityEngine::Rendering::DynamicArrayExtensions {
    pub fn Partition_DynamicArray_1_SortComparer1<T>(
        data: crate::System::Span_1<T>,
        left: i32,
        right: i32,
        comparer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DynamicArray_1_SortComparer<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32>
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
                        crate::System::Span_1<T>,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DynamicArray_1_SortComparer<T>,
                        >,
                    ), i32, 4usize>("Partition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Partition",
                            4usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (data, left, right, comparer))? };
        Ok(__cordl_ret.into())
    }
    pub fn Partition_Span_1_i32_i32_0<T>(
        data: crate::System::Span_1<T>,
        left: i32,
        right: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
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
                    .find_static_method::<(crate::System::Span_1<T>, i32, i32), i32, 3usize>(
                        "Partition",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Partition",
                            3usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (data, left, right))? };
        Ok(__cordl_ret.into())
    }
    pub fn QuickSort_DynamicArray_1_2<T>(
        array: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DynamicArray_1<T>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DynamicArray_1<T>,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("QuickSort")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "QuickSort", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (array))? };
        Ok(__cordl_ret.into())
    }
    pub fn QuickSort_DynamicArray_1_DynamicArray_1_SortComparer3<T>(
        array: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DynamicArray_1<T>>,
        comparer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DynamicArray_1_SortComparer<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DynamicArray_1<T>>,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DynamicArray_1_SortComparer<T>,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("QuickSort")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "QuickSort",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (array, comparer))? };
        Ok(__cordl_ret.into())
    }
    pub fn QuickSort_Span_1_i32_i32_0<T>(
        data: crate::System::Span_1<T>,
        left: i32,
        right: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
                        (crate::System::Span_1<T>, i32, i32),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("QuickSort")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "QuickSort", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (data, left, right))? };
        Ok(__cordl_ret.into())
    }
    pub fn QuickSort_Span_1_i32_i32_DynamicArray_1_SortComparer1<T>(
        data: crate::System::Span_1<T>,
        left: i32,
        right: i32,
        comparer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DynamicArray_1_SortComparer<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
                        crate::System::Span_1<T>,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DynamicArray_1_SortComparer<T>,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>("QuickSort")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "QuickSort",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (data, left, right, comparer))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+DynamicArrayExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::DynamicArrayExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
