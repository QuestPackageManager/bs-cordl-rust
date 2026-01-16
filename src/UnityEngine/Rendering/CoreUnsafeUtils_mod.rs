#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct CoreUnsafeUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::CoreUnsafeUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "CoreUnsafeUtils";
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
#[cfg(feature = "UnityEngine+Rendering+CoreUnsafeUtils")]
impl std::ops::Deref for crate::UnityEngine::Rendering::CoreUnsafeUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CoreUnsafeUtils")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::CoreUnsafeUtils {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CoreUnsafeUtils")]
impl crate::UnityEngine::Rendering::CoreUnsafeUtils {
    #[cfg(feature = "UnityEngine+Rendering+CoreUnsafeUtils+DefaultKeyGetter_1")]
    pub type DefaultKeyGetter_1<T: quest_hook::libil2cpp::Type> =
        crate::UnityEngine::Rendering::CoreUnsafeUtils_DefaultKeyGetter_1<T>;
    #[cfg(feature = "UnityEngine+Rendering+CoreUnsafeUtils+FixedBufferStringQueue")]
    pub type FixedBufferStringQueue =
        crate::UnityEngine::Rendering::CoreUnsafeUtils_FixedBufferStringQueue;
    #[cfg(feature = "UnityEngine+Rendering+CoreUnsafeUtils+IKeyGetter_2")]
    type IKeyGetter_2<TValue: quest_hook::libil2cpp::Type, TKey: quest_hook::libil2cpp::Type> =
        crate::UnityEngine::Rendering::CoreUnsafeUtils_IKeyGetter_2<TValue, TKey>;
    #[cfg(feature = "UnityEngine+Rendering+CoreUnsafeUtils+UintKeyGetter")]
    pub type UintKeyGetter = crate::UnityEngine::Rendering::CoreUnsafeUtils_UintKeyGetter;
    #[cfg(feature = "UnityEngine+Rendering+CoreUnsafeUtils+UlongKeyGetter")]
    pub type UlongKeyGetter = crate::UnityEngine::Rendering::CoreUnsafeUtils_UlongKeyGetter;
    pub fn CalculateRadixParams(
        radixBits: i32,
        bitStates: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32, quest_hook::libil2cpp::ByRefMut<i32>),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("CalculateRadixParams")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CalculateRadixParams", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (radixBits, bitStates))? };
        Ok(__cordl_ret.into())
    }
    pub fn CalculateRadixSortSupportArrays(
        bitStates: i32,
        arrayLength: i32,
        supportArray: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bucketIndices: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
        bucketSizes: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
        bucketPrefix: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
        arrayOutput: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        >,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "CalculateRadixSortSupportArrays"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CalculateRadixSortSupportArrays",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    bitStates,
                    arrayLength,
                    supportArray,
                    bucketIndices,
                    bucketSizes,
                    bucketPrefix,
                    arrayOutput,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CalculateRadixSupportSize(
        bitStates: i32,
        arrayLength: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32, i32), i32, 2usize>("CalculateRadixSupportSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CalculateRadixSupportSize",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (bitStates, arrayLength))? };
        Ok(__cordl_ret.into())
    }
    pub fn CombineHashes_i32_Il2CppObject_Il2CppObject0<TValue, TGetter>(
        count: i32,
        hashes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        outHash: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TGetter: quest_hook::libil2cpp::Type
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
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 3usize>("CombineHashes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CombineHashes",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (count, hashes, outHash))? };
        Ok(__cordl_ret.into())
    }
    pub fn CombineHashes_i32_Il2CppObject_Il2CppObject1(
        count: i32,
        hashes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        outHash: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 3usize>("CombineHashes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CombineHashes",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (count, hashes, outHash))? };
        Ok(__cordl_ret.into())
    }
    pub fn CompareHashes_i32_Il2CppObject_i32_Il2CppObject_Il2CppObject_Il2CppObject_ByRefMut_ByRefMut0<
        TOldValue,
        TOldGetter,
        TNewValue,
        TNewGetter,
    >(
        oldHashCount: i32,
        oldHashes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        newHashCount: i32,
        newHashes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        addIndices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        removeIndices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        addCount: quest_hook::libil2cpp::ByRefMut<i32>,
        remCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TOldValue: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TOldGetter: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TNewValue: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TNewGetter: quest_hook::libil2cpp::Type
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
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), i32, 8usize>("CompareHashes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CompareHashes",
                            8usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    oldHashCount,
                    oldHashes,
                    newHashCount,
                    newHashes,
                    addIndices,
                    removeIndices,
                    addCount,
                    remCount,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompareHashes_i32_Il2CppObject_i32_Il2CppObject_Il2CppObject_Il2CppObject_ByRefMut_ByRefMut1(
        oldHashCount: i32,
        oldHashes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        newHashCount: i32,
        newHashes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        addIndices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        removeIndices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        addCount: quest_hook::libil2cpp::ByRefMut<i32>,
        remCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), i32, 8usize>("CompareHashes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CompareHashes",
                            8usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    oldHashCount,
                    oldHashes,
                    newHashCount,
                    newHashes,
                    addIndices,
                    removeIndices,
                    addCount,
                    remCount,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyTo_Il2CppArray1<T>(
        list: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        count: i32,
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
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>("CopyTo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CopyTo",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (list, dest, count))? };
        Ok(__cordl_ret.into())
    }
    pub fn CopyTo_List_1_0<T>(
        list: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        count: i32,
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
                        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>("CopyTo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CopyTo",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (list, dest, count))? };
        Ok(__cordl_ret.into())
    }
    pub fn HaveDuplicates(
        arr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<i32>,
                        >),
                        bool,
                        1usize,
                    >("HaveDuplicates")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HaveDuplicates", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (arr))? };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf<T>(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        count: i32,
        v: T,
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
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        i32,
                        T,
                    ), i32, 3usize>("IndexOf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IndexOf",
                            3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (data, count, v))? };
        Ok(__cordl_ret.into())
    }
    pub fn InsertionSort_Il2CppArray1(
        arr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        sortSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                        i32,
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
            unsafe { cordl_method_info.invoke_unchecked((), (arr, sortSize))? };
        Ok(__cordl_ret.into())
    }
    pub fn InsertionSort_Il2CppObject0(
        arr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        i32,
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
            unsafe { cordl_method_info.invoke_unchecked((), (arr, length))? };
        Ok(__cordl_ret.into())
    }
    pub fn InsertionSort_NativeArray_1_2(
        arr: crate::Unity::Collections::NativeArray_1<u32>,
        sortSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::NativeArray_1<u32>, i32),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("InsertionSort")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InsertionSort", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (arr, sortSize))? };
        Ok(__cordl_ret.into())
    }
    pub fn MergeSort_Il2CppArray_i32_ByRefMut1(
        arr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        sortSize: i32,
        supportArray: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>("MergeSort")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "MergeSort",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (arr, sortSize, supportArray))? };
        Ok(__cordl_ret.into())
    }
    pub fn MergeSort_Il2CppObject_Il2CppObject_i32_0(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        support: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>("MergeSort")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "MergeSort",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (array, support, length))? };
        Ok(__cordl_ret.into())
    }
    pub fn MergeSort_NativeArray_1_i32_ByRefMut2(
        arr: crate::Unity::Collections::NativeArray_1<u32>,
        sortSize: i32,
        supportArray: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeArray_1<u32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::NativeArray_1<u32>,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeArray_1<u32>,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>("MergeSort")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "MergeSort",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (arr, sortSize, supportArray))? };
        Ok(__cordl_ret.into())
    }
    pub fn Partition<TValue, TKey, TGetter>(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        left: i32,
        right: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TValue: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TGetter: quest_hook::libil2cpp::Type
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
                    ), i32, 3usize>("Partition")
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
    pub fn QuickSort_Il2CppArray_i32_i32_0(
        arr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        left: i32,
        right: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>("QuickSort")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "QuickSort",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (arr, left, right))? };
        Ok(__cordl_ret.into())
    }
    pub fn QuickSort_Il2CppArray_i32_i32_1(
        arr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        left: i32,
        right: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>("QuickSort")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "QuickSort",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (arr, left, right))? };
        Ok(__cordl_ret.into())
    }
    pub fn QuickSort_Il2CppObject_i32_i32_4<TValue, TKey, TGetter>(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        left: i32,
        right: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TGetter: quest_hook::libil2cpp::Type
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
                    ), quest_hook::libil2cpp::Void, 3usize>("QuickSort")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "QuickSort",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (data, left, right))? };
        Ok(__cordl_ret.into())
    }
    pub fn QuickSort_i32_Il2CppObject2<T>(
        count: i32,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
            unsafe { cordl_method_info.invoke_unchecked((), (count, data))? };
        Ok(__cordl_ret.into())
    }
    pub fn QuickSort_i32_Il2CppObject3<TValue, TKey, TGetter>(
        count: i32,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TGetter: quest_hook::libil2cpp::Type
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
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
            unsafe { cordl_method_info.invoke_unchecked((), (count, data))? };
        Ok(__cordl_ret.into())
    }
    pub fn RadixSort_Il2CppArray_i32_ByRefMut1(
        arr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        sortSize: i32,
        supportArray: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        >,
        radixBits: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                        >,
                        i32,
                    ), quest_hook::libil2cpp::Void, 4usize>("RadixSort")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RadixSort",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (arr, sortSize, supportArray, radixBits))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RadixSort_Il2CppObject_Il2CppObject_i32_i32_0(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        support: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        radixBits: i32,
        bitStates: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("RadixSort")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RadixSort",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (array, support, radixBits, bitStates, length))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RadixSort_NativeArray_1_i32_ByRefMut2(
        array: crate::Unity::Collections::NativeArray_1<u32>,
        sortSize: i32,
        supportArray: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeArray_1<u32>,
        >,
        radixBits: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::NativeArray_1<u32>,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeArray_1<u32>,
                        >,
                        i32,
                    ), quest_hook::libil2cpp::Void, 4usize>("RadixSort")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RadixSort",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (array, sortSize, supportArray, radixBits))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::CoreUnsafeUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils+DefaultKeyGetter_1")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct CoreUnsafeUtils_DefaultKeyGetter_1<T: quest_hook::libil2cpp::Type> {
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils+DefaultKeyGetter_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_DefaultKeyGetter_1<T>
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "CoreUnsafeUtils/DefaultKeyGetter`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find(
                "UnityEngine.Rendering",
                "CoreUnsafeUtils/DefaultKeyGetter`1",
            )
            .unwrap()
            .make_generic::<(T)>()
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils+DefaultKeyGetter_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_DefaultKeyGetter_1<T>
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils+DefaultKeyGetter_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_DefaultKeyGetter_1<T>
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils+DefaultKeyGetter_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_DefaultKeyGetter_1<T>
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils+DefaultKeyGetter_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_DefaultKeyGetter_1<T>
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils+DefaultKeyGetter_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_DefaultKeyGetter_1<T>
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CoreUnsafeUtils+DefaultKeyGetter_1")]
impl<T: quest_hook::libil2cpp::Type>
    crate::UnityEngine::Rendering::CoreUnsafeUtils_DefaultKeyGetter_1<T>
{
    pub fn Get(&mut self, v: quest_hook::libil2cpp::ByRefMut<T>) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<T>), T, 1usize>("Get")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Get",
                            1usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe { cordl_method_info.invoke_unchecked(self, (v))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+CoreUnsafeUtils+DefaultKeyGetter_1")]
impl<T: quest_hook::libil2cpp::Type>
    AsRef<crate::UnityEngine::Rendering::CoreUnsafeUtils_IKeyGetter_2<T, T>>
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_DefaultKeyGetter_1<T>
{
    fn as_ref(&self) -> &crate::UnityEngine::Rendering::CoreUnsafeUtils_IKeyGetter_2<T, T> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+CoreUnsafeUtils+DefaultKeyGetter_1")]
impl<T: quest_hook::libil2cpp::Type>
    AsMut<crate::UnityEngine::Rendering::CoreUnsafeUtils_IKeyGetter_2<T, T>>
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_DefaultKeyGetter_1<T>
{
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Rendering::CoreUnsafeUtils_IKeyGetter_2<T, T> {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils+FixedBufferStringQueue")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct CoreUnsafeUtils_FixedBufferStringQueue {
    pub m_ReadCursor: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_WriteCursor: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_BufferEnd: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_BufferStart: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_BufferLength: i32,
    pub _Count_k__BackingField: i32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils+FixedBufferStringQueue")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_FixedBufferStringQueue
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "CoreUnsafeUtils/FixedBufferStringQueue";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils+FixedBufferStringQueue")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_FixedBufferStringQueue
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils+FixedBufferStringQueue")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_FixedBufferStringQueue
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils+FixedBufferStringQueue")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_FixedBufferStringQueue
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils+FixedBufferStringQueue")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_FixedBufferStringQueue
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils+FixedBufferStringQueue")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_FixedBufferStringQueue
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CoreUnsafeUtils+FixedBufferStringQueue")]
impl crate::UnityEngine::Rendering::CoreUnsafeUtils_FixedBufferStringQueue {
    pub fn Clear(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Clear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Clear",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn TryPop(
        &mut self,
        v: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >), bool, 1usize>("TryPop")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryPop",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (v))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryPush(
        &mut self,
        v: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        bool,
                        1usize,
                    >("TryPush")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "TryPush",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (v))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (ptr, length))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_Count")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_Count",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_Count(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("set_Count")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_Count",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils+IKeyGetter_2")]
#[derive(Debug)]
#[repr(C)]
pub struct CoreUnsafeUtils_IKeyGetter_2<
    TValue: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
    __cordl_phantom_TKey: std::marker::PhantomData<TKey>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils+IKeyGetter_2")]
unsafe impl<TValue: quest_hook::libil2cpp::Type, TKey: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_IKeyGetter_2<TValue, TKey>
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "CoreUnsafeUtils/IKeyGetter`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find(
                "UnityEngine.Rendering",
                "CoreUnsafeUtils/IKeyGetter`2",
            )
            .unwrap()
            .make_generic::<(TValue, TKey)>()
            .unwrap()
            .unwrap()
        })
    }
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
#[cfg(feature = "UnityEngine+Rendering+CoreUnsafeUtils+IKeyGetter_2")]
impl<TValue: quest_hook::libil2cpp::Type, TKey: quest_hook::libil2cpp::Type> std::ops::Deref
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_IKeyGetter_2<TValue, TKey>
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CoreUnsafeUtils+IKeyGetter_2")]
impl<TValue: quest_hook::libil2cpp::Type, TKey: quest_hook::libil2cpp::Type> std::ops::DerefMut
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_IKeyGetter_2<TValue, TKey>
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CoreUnsafeUtils+IKeyGetter_2")]
impl<TValue: quest_hook::libil2cpp::Type, TKey: quest_hook::libil2cpp::Type>
    crate::UnityEngine::Rendering::CoreUnsafeUtils_IKeyGetter_2<TValue, TKey>
{
    pub fn Get(
        &mut self,
        v: quest_hook::libil2cpp::ByRefMut<TValue>,
    ) -> quest_hook::libil2cpp::Result<TKey>
    where
        TValue: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
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
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<TValue>), TKey, 1usize>("Get")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Get",
                            1usize
                        )
                    })
            });
        let __cordl_ret: TKey = unsafe { cordl_method_info.invoke_unchecked(self, (v))? };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(object_param: *mut quest_hook::libil2cpp::Il2CppObject) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils+IKeyGetter_2")]
impl<TValue: quest_hook::libil2cpp::Type, TKey: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_IKeyGetter_2<TValue, TKey>
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils+UintKeyGetter")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct CoreUnsafeUtils_UintKeyGetter {}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils+UintKeyGetter")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_UintKeyGetter
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "CoreUnsafeUtils/UintKeyGetter";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils+UintKeyGetter")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_UintKeyGetter
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils+UintKeyGetter")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_UintKeyGetter
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils+UintKeyGetter")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_UintKeyGetter
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils+UintKeyGetter")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_UintKeyGetter
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils+UintKeyGetter")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_UintKeyGetter
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CoreUnsafeUtils+UintKeyGetter")]
impl crate::UnityEngine::Rendering::CoreUnsafeUtils_UintKeyGetter {
    pub fn Get(
        &mut self,
        v: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<u32>), u32, 1usize>("Get")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Get",
                            1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked(self, (v))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+CoreUnsafeUtils+UintKeyGetter")]
impl AsRef<crate::UnityEngine::Rendering::CoreUnsafeUtils_IKeyGetter_2<u32, u32>>
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_UintKeyGetter
{
    fn as_ref(&self) -> &crate::UnityEngine::Rendering::CoreUnsafeUtils_IKeyGetter_2<u32, u32> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+CoreUnsafeUtils+UintKeyGetter")]
impl AsMut<crate::UnityEngine::Rendering::CoreUnsafeUtils_IKeyGetter_2<u32, u32>>
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_UintKeyGetter
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::Rendering::CoreUnsafeUtils_IKeyGetter_2<u32, u32> {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils+UlongKeyGetter")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct CoreUnsafeUtils_UlongKeyGetter {}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils+UlongKeyGetter")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_UlongKeyGetter
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "CoreUnsafeUtils/UlongKeyGetter";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils+UlongKeyGetter")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_UlongKeyGetter
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils+UlongKeyGetter")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_UlongKeyGetter
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils+UlongKeyGetter")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_UlongKeyGetter
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils+UlongKeyGetter")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_UlongKeyGetter
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUnsafeUtils+UlongKeyGetter")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_UlongKeyGetter
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CoreUnsafeUtils+UlongKeyGetter")]
impl crate::UnityEngine::Rendering::CoreUnsafeUtils_UlongKeyGetter {
    pub fn Get(
        &mut self,
        v: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<u64>), u64, 1usize>("Get")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Get",
                            1usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe { cordl_method_info.invoke_unchecked(self, (v))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+CoreUnsafeUtils+UlongKeyGetter")]
impl AsRef<crate::UnityEngine::Rendering::CoreUnsafeUtils_IKeyGetter_2<u64, u64>>
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_UlongKeyGetter
{
    fn as_ref(&self) -> &crate::UnityEngine::Rendering::CoreUnsafeUtils_IKeyGetter_2<u64, u64> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+CoreUnsafeUtils+UlongKeyGetter")]
impl AsMut<crate::UnityEngine::Rendering::CoreUnsafeUtils_IKeyGetter_2<u64, u64>>
    for crate::UnityEngine::Rendering::CoreUnsafeUtils_UlongKeyGetter
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::Rendering::CoreUnsafeUtils_IKeyGetter_2<u64, u64> {
        todo!()
    }
}
