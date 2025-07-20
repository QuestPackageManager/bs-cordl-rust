#[cfg(feature = "System+Collections+Generic+ArraySortHelper_2")]
#[repr(C)]
#[derive(Debug)]
pub struct ArraySortHelper_2<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_TKey: std::marker::PhantomData<TKey>,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "System+Collections+Generic+ArraySortHelper_2")]
unsafe impl<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::System::Collections::Generic::ArraySortHelper_2<TKey, TValue> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Collections.Generic";
    const CLASS_NAME: &'static str = "ArraySortHelper`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "System.Collections.Generic",
                        "ArraySortHelper`2",
                    )
                    .unwrap()
                    .make_generic::<(TKey, TValue)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "System+Collections+Generic+ArraySortHelper_2")]
impl<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::System::Collections::Generic::ArraySortHelper_2<TKey, TValue> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Generic+ArraySortHelper_2")]
impl<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::System::Collections::Generic::ArraySortHelper_2<TKey, TValue> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Generic+ArraySortHelper_2")]
impl<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> crate::System::Collections::Generic::ArraySortHelper_2<TKey, TValue> {
    pub fn DownHeap(
        keys: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TKey>>,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        i: i32,
        n: i32,
        lo: i32,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IComparer_1<TKey>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TKey>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TValue>,
                            >,
                            i32,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IComparer_1<TKey>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("DownHeap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DownHeap", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (keys, values, i, n, lo, comparer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Heapsort(
        keys: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TKey>>,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        lo: i32,
        hi: i32,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IComparer_1<TKey>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TKey>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TValue>,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IComparer_1<TKey>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("Heapsort")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Heapsort", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (keys, values, lo, hi, comparer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InsertionSort(
        keys: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TKey>>,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        lo: i32,
        hi: i32,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IComparer_1<TKey>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TKey>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TValue>,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IComparer_1<TKey>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("InsertionSort")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InsertionSort", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (keys, values, lo, hi, comparer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IntroSort(
        keys: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TKey>>,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        lo: i32,
        hi: i32,
        depthLimit: i32,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IComparer_1<TKey>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TKey>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TValue>,
                            >,
                            i32,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IComparer_1<TKey>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("IntroSort")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IntroSort", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (keys, values, lo, hi, depthLimit, comparer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IntrospectiveSort(
        keys: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TKey>>,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        left: i32,
        length: i32,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IComparer_1<TKey>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TKey>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TValue>,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IComparer_1<TKey>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("IntrospectiveSort")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IntrospectiveSort", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (keys, values, left, length, comparer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PickPivotAndPartition(
        keys: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TKey>>,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        lo: i32,
        hi: i32,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IComparer_1<TKey>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TKey>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TValue>,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IComparer_1<TKey>,
                            >,
                        ),
                        i32,
                        5usize,
                    >("PickPivotAndPartition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PickPivotAndPartition", 5usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (keys, values, lo, hi, comparer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Sort(
        &mut self,
        keys: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TKey>>,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        index: i32,
        length: i32,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IComparer_1<TKey>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TKey>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TValue>,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IComparer_1<TKey>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("Sort")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Sort",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (keys, values, index, length, comparer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Swap(
        keys: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TKey>>,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        i: i32,
        j: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TKey>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TValue>,
                            >,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("Swap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Swap",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (keys, values, i, j))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SwapIfGreaterWithItems(
        keys: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TKey>>,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IComparer_1<TKey>,
        >,
        a: i32,
        b: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TKey>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TValue>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IComparer_1<TKey>,
                            >,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("SwapIfGreaterWithItems")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SwapIfGreaterWithItems", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (keys, values, comparer, a, b))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Default() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::ArraySortHelper_2<TKey, TValue>,
        >,
    >
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::ArraySortHelper_2<
                                TKey,
                                TValue,
                            >,
                        >,
                        0usize,
                    >("get_Default")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Default", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::ArraySortHelper_2<TKey, TValue>,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Collections+Generic+ArraySortHelper_2")]
impl<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::System::Collections::Generic::ArraySortHelper_2<TKey, TValue> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
