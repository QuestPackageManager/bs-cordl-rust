#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+ArrayUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct ArrayUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+ArrayUtility")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ProBuilder::ArrayUtility {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder";
    const CLASS_NAME: &'static str = "ArrayUtility";
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
#[cfg(feature = "UnityEngine+ProBuilder+ArrayUtility")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::ArrayUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ArrayUtility")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::ArrayUtility {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ArrayUtility")]
impl crate::UnityEngine::ProBuilder::ArrayUtility {
    #[cfg(feature = "UnityEngine+ProBuilder+ArrayUtility+SearchRange")]
    pub type SearchRange = crate::UnityEngine::ProBuilder::ArrayUtility_SearchRange;
    pub fn Add<T>(
        arr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        val: T,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<T>,
                            >,
                            T,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
                        2usize,
                    >("Add")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Add",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (arr, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn AddOrAppend<T, K>(
        dictionary: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                T,
                quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<K>>,
            >,
        >,
        key: T,
        value: K,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::Dictionary_2<
                                    T,
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Collections::Generic::List_1<K>,
                                    >,
                                >,
                            >,
                            T,
                            K,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("AddOrAppend")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddOrAppend", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (dictionary, key, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddOrAppendRange<T, K>(
        dictionary: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                T,
                quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<K>>,
            >,
        >,
        key: T,
        value: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<K>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::Dictionary_2<
                                    T,
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Collections::Generic::List_1<K>,
                                    >,
                                >,
                            >,
                            T,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<K>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("AddOrAppendRange")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddOrAppendRange", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (dictionary, key, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddRange<T>(
        arr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        val: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<T>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<T>,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
                        2usize,
                    >("AddRange")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddRange", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (arr, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn AllIndexesOf<T>(
        list: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IList_1<T>>,
        lambda: quest_hook::libil2cpp::Gc<crate::System::Func_2<T, bool>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<T>,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Func_2<T, bool>>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<i32>,
                        >,
                        2usize,
                    >("AllIndexesOf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AllIndexesOf", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (list, lambda))? };
        Ok(__cordl_ret.into())
    }
    pub fn Concat<T>(
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<T>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<T>,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
                        2usize,
                    >("Concat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Concat",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (x, y))? };
        Ok(__cordl_ret.into())
    }
    pub fn ContainsMatch_ByRefMut_ByRefMut1<T>(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        index_a: quest_hook::libil2cpp::ByRefMut<i32>,
        index_b: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<T>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<T>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        bool,
                        4usize,
                    >("ContainsMatch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ContainsMatch", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (a, b, index_a, index_b))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ContainsMatch_Il2CppArray_Il2CppArray0<T>(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<T>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<T>,
                            >,
                        ),
                        bool,
                        2usize,
                    >("ContainsMatch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ContainsMatch", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (a, b))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DistinctBy<TSource, TKey>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        keySelector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TKey>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IEnumerable_1<TSource>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_2<TSource, TKey>,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<TSource>,
                        >,
                        2usize,
                    >("DistinctBy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DistinctBy", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (source, keySelector))? };
        Ok(__cordl_ret.into())
    }
    pub fn Fill_Func_2_i32_0<T>(
        ctor: quest_hook::libil2cpp::Gc<crate::System::Func_2<i32, T>>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Func_2<i32, T>>, i32),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<T>,
                        >,
                        2usize,
                    >("Fill")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Fill",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<T>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (ctor, length))? };
        Ok(__cordl_ret.into())
    }
    pub fn Fill_T_i32_1<T>(
        val: T,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (T, i32),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
                        2usize,
                    >("Fill")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Fill",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (val, length))? };
        Ok(__cordl_ret.into())
    }
    pub fn Fill_i32_Func_2_2<T>(
        count: i32,
        ctor: quest_hook::libil2cpp::Gc<crate::System::Func_2<i32, T>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32, quest_hook::libil2cpp::Gc<crate::System::Func_2<i32, T>>),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
                        2usize,
                    >("Fill")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Fill",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (count, ctor))? };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf<T>(
        InList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
            >,
        >,
        InValue: T,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Collections::Generic::List_1<T>,
                                    >,
                                >,
                            >,
                            T,
                        ),
                        i32,
                        2usize,
                    >("IndexOf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "IndexOf",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (InList, InValue))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NearestIndexPriorToValue<T>(
        sorted_list: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<T>,
        >,
        value: T,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<T>,
                            >,
                            T,
                        ),
                        i32,
                        2usize,
                    >("NearestIndexPriorToValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NearestIndexPriorToValue", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (sorted_list, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveAt_IList_1_IEnumerable_1_1<T>(
        list: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IList_1<T>>,
        indexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<T>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IEnumerable_1<i32>,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
                        2usize,
                    >("RemoveAt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RemoveAt", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (list, indexes))? };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveAt_Il2CppArray_i32_0<T>(
        arr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<T>,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
                        2usize,
                    >("RemoveAt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RemoveAt", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (arr, index))? };
        Ok(__cordl_ret.into())
    }
    pub fn Remove_IEnumerable_1_1<T>(
        arr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        val: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<T>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IEnumerable_1<T>,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
                        2usize,
                    >("Remove")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Remove",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (arr, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn Remove_T0<T>(
        arr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        val: T,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<T>,
                            >,
                            T,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
                        2usize,
                    >("Remove")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Remove",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (arr, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn SortedRemoveAt<T>(
        list: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IList_1<T>>,
        sorted: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<T>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<i32>,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
                        2usize,
                    >("SortedRemoveAt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SortedRemoveAt", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (list, sorted))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Dictionary_2_0<TKey, TValue>(
        dict: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<TKey, TValue>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::Dictionary_2<
                                TKey,
                                TValue,
                            >,
                        >),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToString", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked((), (dict))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_IEnumerable_1_Il2CppString1<T>(
        arr: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        >,
        separator: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IEnumerable_1<T>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        2usize,
                    >("ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToString", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked((), (arr, separator))? };
        Ok(__cordl_ret.into())
    }
    pub fn ValuesWithIndexes_Il2CppArray_Il2CppArray0<T>(
        arr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        indexes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<T>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<i32>,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
                        2usize,
                    >("ValuesWithIndexes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ValuesWithIndexes", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (arr, indexes))? };
        Ok(__cordl_ret.into())
    }
    pub fn ValuesWithIndexes_List_1_IList_1_1<T>(
        arr: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
        indexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<T>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<i32>,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<T>,
                        >,
                        2usize,
                    >("ValuesWithIndexes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ValuesWithIndexes", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<T>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (arr, indexes))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+ArrayUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::ArrayUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+ArrayUtility+SearchRange")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ArrayUtility_SearchRange {
    pub begin: i32,
    pub end: i32,
}
#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+ArrayUtility+SearchRange")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ProBuilder::ArrayUtility_SearchRange {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder";
    const CLASS_NAME: &'static str = "ArrayUtility/SearchRange";
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
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+ArrayUtility+SearchRange")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::ProBuilder::ArrayUtility_SearchRange {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+ArrayUtility+SearchRange")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::ProBuilder::ArrayUtility_SearchRange {
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
#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+ArrayUtility+SearchRange")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::ProBuilder::ArrayUtility_SearchRange {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+ArrayUtility+SearchRange")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::ProBuilder::ArrayUtility_SearchRange {
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
#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+ArrayUtility+SearchRange")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ProBuilder::ArrayUtility_SearchRange {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ArrayUtility+SearchRange")]
impl crate::UnityEngine::ProBuilder::ArrayUtility_SearchRange {
    pub fn Center(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("Center")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Center",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToString", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Valid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("Valid")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Valid",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        begin: i32,
        end: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, i32),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (begin, end))?
        };
        Ok(__cordl_ret.into())
    }
}
