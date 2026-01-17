#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+Sorting")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct Sorting {}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+Sorting")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::Universal::Sorting {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "Sorting";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+Sorting")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::Rendering::Universal::Sorting {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+Sorting")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::Rendering::Universal::Sorting {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+Sorting")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::Rendering::Universal::Sorting {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+Sorting")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::Rendering::Universal::Sorting {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+Sorting")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::Universal::Sorting
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+Sorting")]
impl crate::UnityEngine::Rendering::Universal::Sorting {
    pub fn InsertionSort_Func_3_0<T>(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        compare: quest_hook::libil2cpp::Gc<crate::System::Func_3<T, T, i32>>,
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
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
                        quest_hook::libil2cpp::Gc<crate::System::Func_3<T, T, i32>>,
                    ), quest_hook::libil2cpp::Void, 2usize>("InsertionSort")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InsertionSort",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (data, compare))? };
        Ok(__cordl_ret.into())
    }
    pub fn InsertionSort_i32_i32_Func_3_1<T>(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        start: i32,
        end: i32,
        compare: quest_hook::libil2cpp::Gc<crate::System::Func_3<T, T, i32>>,
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
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::System::Func_3<T, T, i32>>,
                    ), quest_hook::libil2cpp::Void, 4usize>("InsertionSort")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InsertionSort",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (data, start, end, compare))? };
        Ok(__cordl_ret.into())
    }
    pub fn Median3Pivot<T>(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        start: i32,
        pivot: i32,
        end: i32,
        compare: quest_hook::libil2cpp::Gc<crate::System::Func_3<T, T, i32>>,
    ) -> quest_hook::libil2cpp::Result<T>
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
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
                        i32,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::System::Func_3<T, T, i32>>,
                    ), T, 5usize>("Median3Pivot")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Median3Pivot",
                            5usize
                        )
                    })
            });
        let __cordl_ret: T =
            unsafe { cordl_method_info.invoke_unchecked((), (data, start, pivot, end, compare))? };
        Ok(__cordl_ret.into())
    }
    pub fn Partition<T>(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        start: i32,
        end: i32,
        compare: quest_hook::libil2cpp::Gc<crate::System::Func_3<T, T, i32>>,
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
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::System::Func_3<T, T, i32>>,
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
            unsafe { cordl_method_info.invoke_unchecked((), (data, start, end, compare))? };
        Ok(__cordl_ret.into())
    }
    pub fn QuickSort_Func_3_0<T>(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        compare: quest_hook::libil2cpp::Gc<crate::System::Func_3<T, T, i32>>,
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
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
                        quest_hook::libil2cpp::Gc<crate::System::Func_3<T, T, i32>>,
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
            unsafe { cordl_method_info.invoke_unchecked((), (data, compare))? };
        Ok(__cordl_ret.into())
    }
    pub fn QuickSort_i32_i32_Func_3_1<T>(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        start: i32,
        end: i32,
        compare: quest_hook::libil2cpp::Gc<crate::System::Func_3<T, T, i32>>,
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
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::System::Func_3<T, T, i32>>,
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
            unsafe { cordl_method_info.invoke_unchecked((), (data, start, end, compare))? };
        Ok(__cordl_ret.into())
    }
    pub fn _Median3Pivot_g__Swap_4_0<T>(
        a: i32,
        b: i32,
        _cordl_fixed_empty_name_whitespace: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::ValueTypePadding<T>,
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
                        i32,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<quest_hook::libil2cpp::ValueTypePadding<T>>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "<Median3Pivot>g__Swap|4_0"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "<Median3Pivot>g__Swap|4_0",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (a, b, _cordl_fixed_empty_name_whitespace))?
        };
        Ok(__cordl_ret.into())
    }
}
