#[cfg(feature = "cordl_class_System+Linq+Enumerable")]
#[repr(C)]
#[derive(Debug)]
pub struct Enumerable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_System+Linq+Enumerable")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Linq::Enumerable {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq";
    const CLASS_NAME: &'static str = "Enumerable";
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
#[cfg(feature = "System+Linq+Enumerable")]
impl std::ops::Deref for crate::System::Linq::Enumerable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Enumerable")]
impl std::ops::DerefMut for crate::System::Linq::Enumerable {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Enumerable")]
impl crate::System::Linq::Enumerable {
    #[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
    pub type Iterator_1<TSource: quest_hook::libil2cpp::Type> = crate::System::Linq::Enumerable_Iterator_1<
        TSource,
    >;
    #[cfg(feature = "System+Linq+Enumerable+WhereArrayIterator_1")]
    pub type WhereArrayIterator_1<TSource: quest_hook::libil2cpp::Type> = crate::System::Linq::Enumerable_WhereArrayIterator_1<
        TSource,
    >;
    #[cfg(feature = "System+Linq+Enumerable+WhereEnumerableIterator_1")]
    pub type WhereEnumerableIterator_1<TSource: quest_hook::libil2cpp::Type> = crate::System::Linq::Enumerable_WhereEnumerableIterator_1<
        TSource,
    >;
    #[cfg(feature = "System+Linq+Enumerable+WhereListIterator_1")]
    pub type WhereListIterator_1<TSource: quest_hook::libil2cpp::Type> = crate::System::Linq::Enumerable_WhereListIterator_1<
        TSource,
    >;
    #[cfg(feature = "System+Linq+Enumerable+WhereSelectArrayIterator_2")]
    pub type WhereSelectArrayIterator_2<
        TSource: quest_hook::libil2cpp::Type,
        TResult: quest_hook::libil2cpp::Type,
    > = crate::System::Linq::Enumerable_WhereSelectArrayIterator_2<TSource, TResult>;
    #[cfg(feature = "System+Linq+Enumerable+WhereSelectEnumerableIterator_2")]
    pub type WhereSelectEnumerableIterator_2<
        TSource: quest_hook::libil2cpp::Type,
        TResult: quest_hook::libil2cpp::Type,
    > = crate::System::Linq::Enumerable_WhereSelectEnumerableIterator_2<
        TSource,
        TResult,
    >;
    #[cfg(feature = "System+Linq+Enumerable+WhereSelectListIterator_2")]
    pub type WhereSelectListIterator_2<
        TSource: quest_hook::libil2cpp::Type,
        TResult: quest_hook::libil2cpp::Type,
    > = crate::System::Linq::Enumerable_WhereSelectListIterator_2<TSource, TResult>;
    pub fn Aggregate_Gc0<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        func: quest_hook::libil2cpp::Gc<TSource, TSource, TSource>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource, TSource, TSource>,
                        ),
                        TSource,
                        2usize,
                    >("Aggregate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Aggregate", 2usize
                        )
                    })
            });
        let __cordl_ret: TSource = unsafe {
            cordl_method_info.invoke_unchecked((), (source, func))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Aggregate_TAccumulate_Gc1<TSource, TAccumulate>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        seed: TAccumulate,
        func: quest_hook::libil2cpp::Gc<TAccumulate, TSource, TAccumulate>,
    ) -> quest_hook::libil2cpp::Result<TAccumulate>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TAccumulate: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            TAccumulate,
                            quest_hook::libil2cpp::Gc<TAccumulate, TSource, TAccumulate>,
                        ),
                        TAccumulate,
                        3usize,
                    >("Aggregate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Aggregate", 3usize
                        )
                    })
            });
        let __cordl_ret: TAccumulate = unsafe {
            cordl_method_info.invoke_unchecked((), (source, seed, func))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn All<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource, bool>,
                        ),
                        bool,
                        2usize,
                    >("All")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "All",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (source, predicate))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Any_Gc0<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<TSource>),
                        bool,
                        1usize,
                    >("Any")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Any",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (source))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Any_Gc1<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource, bool>,
                        ),
                        bool,
                        2usize,
                    >("Any")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Any",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (source, predicate))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Average(
        source: quest_hook::libil2cpp::Gc<i32>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<i32>),
                        f64,
                        1usize,
                    >("Average")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Average",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe {
            cordl_method_info.invoke_unchecked((), (source))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Cast<TResult>(
        source: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::IEnumerable,
                        >),
                        quest_hook::libil2cpp::Gc<TResult>,
                        1usize,
                    >("Cast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Cast",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = unsafe {
            cordl_method_info.invoke_unchecked((), (source))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CastIterator<TResult>(
        source: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::IEnumerable,
                        >),
                        quest_hook::libil2cpp::Gc<TResult>,
                        1usize,
                    >("CastIterator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CastIterator", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = unsafe {
            cordl_method_info.invoke_unchecked((), (source))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CombinePredicates<TSource>(
        predicate1: quest_hook::libil2cpp::Gc<TSource, bool>,
        predicate2: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource, bool>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource, bool>,
                            quest_hook::libil2cpp::Gc<TSource, bool>,
                        ),
                        quest_hook::libil2cpp::Gc<TSource, bool>,
                        2usize,
                    >("CombinePredicates")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CombinePredicates", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource, bool> = unsafe {
            cordl_method_info.invoke_unchecked((), (predicate1, predicate2))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CombineSelectors<TSource, TMiddle, TResult>(
        selector1: quest_hook::libil2cpp::Gc<TSource, TMiddle>,
        selector2: quest_hook::libil2cpp::Gc<TMiddle, TResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource, TResult>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TMiddle: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource, TMiddle>,
                            quest_hook::libil2cpp::Gc<TMiddle, TResult>,
                        ),
                        quest_hook::libil2cpp::Gc<TSource, TResult>,
                        2usize,
                    >("CombineSelectors")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CombineSelectors", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource, TResult> = unsafe {
            cordl_method_info.invoke_unchecked((), (selector1, selector2))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Concat<TSource>(
        first: quest_hook::libil2cpp::Gc<TSource>,
        second: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource>,
                        ),
                        quest_hook::libil2cpp::Gc<TSource>,
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked((), (first, second))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConcatIterator<TSource>(
        first: quest_hook::libil2cpp::Gc<TSource>,
        second: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource>,
                        ),
                        quest_hook::libil2cpp::Gc<TSource>,
                        2usize,
                    >("ConcatIterator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConcatIterator", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked((), (first, second))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Contains_Gc1<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        value: TSource,
        comparer: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            TSource,
                            quest_hook::libil2cpp::Gc<TSource>,
                        ),
                        bool,
                        3usize,
                    >("Contains")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Contains", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (source, value, comparer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Contains_Gc_TSource0<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        value: TSource,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<TSource>, TSource),
                        bool,
                        2usize,
                    >("Contains")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Contains", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (source, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Count_Gc0<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<TSource>),
                        i32,
                        1usize,
                    >("Count")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Count",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (source))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Count_Gc1<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource, bool>,
                        ),
                        i32,
                        2usize,
                    >("Count")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Count",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (source, predicate))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DefaultIfEmpty<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        defaultValue: TSource,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<TSource>, TSource),
                        quest_hook::libil2cpp::Gc<TSource>,
                        2usize,
                    >("DefaultIfEmpty")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DefaultIfEmpty", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked((), (source, defaultValue))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DefaultIfEmptyIterator<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        defaultValue: TSource,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<TSource>, TSource),
                        quest_hook::libil2cpp::Gc<TSource>,
                        2usize,
                    >("DefaultIfEmptyIterator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DefaultIfEmptyIterator", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked((), (source, defaultValue))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Distinct<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<TSource>),
                        quest_hook::libil2cpp::Gc<TSource>,
                        1usize,
                    >("Distinct")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Distinct", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked((), (source))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DistinctIterator<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        comparer: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource>,
                        ),
                        quest_hook::libil2cpp::Gc<TSource>,
                        2usize,
                    >("DistinctIterator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DistinctIterator", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked((), (source, comparer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ElementAt<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<TSource>, i32),
                        TSource,
                        2usize,
                    >("ElementAt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ElementAt", 2usize
                        )
                    })
            });
        let __cordl_ret: TSource = unsafe {
            cordl_method_info.invoke_unchecked((), (source, index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Empty<TResult>() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<TResult>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<TResult>,
                        0usize,
                    >("Empty")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Empty",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Except<TSource>(
        first: quest_hook::libil2cpp::Gc<TSource>,
        second: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource>,
                        ),
                        quest_hook::libil2cpp::Gc<TSource>,
                        2usize,
                    >("Except")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Except",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked((), (first, second))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExceptIterator<TSource>(
        first: quest_hook::libil2cpp::Gc<TSource>,
        second: quest_hook::libil2cpp::Gc<TSource>,
        comparer: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource>,
                        ),
                        quest_hook::libil2cpp::Gc<TSource>,
                        3usize,
                    >("ExceptIterator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExceptIterator", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked((), (first, second, comparer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FirstOrDefault_Gc0<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<TSource>),
                        TSource,
                        1usize,
                    >("FirstOrDefault")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FirstOrDefault", 1usize
                        )
                    })
            });
        let __cordl_ret: TSource = unsafe {
            cordl_method_info.invoke_unchecked((), (source))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FirstOrDefault_Gc1<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource, bool>,
                        ),
                        TSource,
                        2usize,
                    >("FirstOrDefault")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FirstOrDefault", 2usize
                        )
                    })
            });
        let __cordl_ret: TSource = unsafe {
            cordl_method_info.invoke_unchecked((), (source, predicate))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn First_Gc0<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<TSource>),
                        TSource,
                        1usize,
                    >("First")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "First",
                            1usize
                        )
                    })
            });
        let __cordl_ret: TSource = unsafe {
            cordl_method_info.invoke_unchecked((), (source))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn First_Gc1<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource, bool>,
                        ),
                        TSource,
                        2usize,
                    >("First")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "First",
                            2usize
                        )
                    })
            });
        let __cordl_ret: TSource = unsafe {
            cordl_method_info.invoke_unchecked((), (source, predicate))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GroupBy<TSource, TKey>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        keySelector: quest_hook::libil2cpp::Gc<TSource, TKey>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Gc<TKey, TSource>>,
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
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource, TKey>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Gc<TKey, TSource>,
                        >,
                        2usize,
                    >("GroupBy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "GroupBy",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<TKey, TSource>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (source, keySelector))? };
        Ok(__cordl_ret.into())
    }
    pub fn Intersect<TSource>(
        first: quest_hook::libil2cpp::Gc<TSource>,
        second: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource>,
                        ),
                        quest_hook::libil2cpp::Gc<TSource>,
                        2usize,
                    >("Intersect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Intersect", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked((), (first, second))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IntersectIterator<TSource>(
        first: quest_hook::libil2cpp::Gc<TSource>,
        second: quest_hook::libil2cpp::Gc<TSource>,
        comparer: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource>,
                        ),
                        quest_hook::libil2cpp::Gc<TSource>,
                        3usize,
                    >("IntersectIterator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IntersectIterator", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked((), (first, second, comparer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Last<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<TSource>),
                        TSource,
                        1usize,
                    >("Last")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Last",
                            1usize
                        )
                    })
            });
        let __cordl_ret: TSource = unsafe {
            cordl_method_info.invoke_unchecked((), (source))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LastOrDefault_Gc0<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<TSource>),
                        TSource,
                        1usize,
                    >("LastOrDefault")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LastOrDefault", 1usize
                        )
                    })
            });
        let __cordl_ret: TSource = unsafe {
            cordl_method_info.invoke_unchecked((), (source))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LastOrDefault_Gc1<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource, bool>,
                        ),
                        TSource,
                        2usize,
                    >("LastOrDefault")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LastOrDefault", 2usize
                        )
                    })
            });
        let __cordl_ret: TSource = unsafe {
            cordl_method_info.invoke_unchecked((), (source, predicate))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Max_Gc0(
        source: quest_hook::libil2cpp::Gc<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<i32>),
                        i32,
                        1usize,
                    >("Max")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Max",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (source))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Max_Gc1(
        source: quest_hook::libil2cpp::Gc<f32>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<f32>),
                        f32,
                        1usize,
                    >("Max")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Max",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked((), (source))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Max_Gc2<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<TSource>),
                        TSource,
                        1usize,
                    >("Max")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Max",
                            1usize
                        )
                    })
            });
        let __cordl_ret: TSource = unsafe {
            cordl_method_info.invoke_unchecked((), (source))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Max_Gc3<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        selector: quest_hook::libil2cpp::Gc<TSource, i32>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource, i32>,
                        ),
                        i32,
                        2usize,
                    >("Max")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Max",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (source, selector))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Max_Gc4<TSource, TResult>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        selector: quest_hook::libil2cpp::Gc<TSource, TResult>,
    ) -> quest_hook::libil2cpp::Result<TResult>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource, TResult>,
                        ),
                        TResult,
                        2usize,
                    >("Max")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Max",
                            2usize
                        )
                    })
            });
        let __cordl_ret: TResult = unsafe {
            cordl_method_info.invoke_unchecked((), (source, selector))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Min_Gc0(
        source: quest_hook::libil2cpp::Gc<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<i32>),
                        i32,
                        1usize,
                    >("Min")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Min",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (source))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Min_Gc1(
        source: quest_hook::libil2cpp::Gc<f32>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<f32>),
                        f32,
                        1usize,
                    >("Min")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Min",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked((), (source))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Min_Gc2<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<TSource>),
                        TSource,
                        1usize,
                    >("Min")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Min",
                            1usize
                        )
                    })
            });
        let __cordl_ret: TSource = unsafe {
            cordl_method_info.invoke_unchecked((), (source))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Min_Gc3<TSource, TResult>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        selector: quest_hook::libil2cpp::Gc<TSource, TResult>,
    ) -> quest_hook::libil2cpp::Result<TResult>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource, TResult>,
                        ),
                        TResult,
                        2usize,
                    >("Min")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Min",
                            2usize
                        )
                    })
            });
        let __cordl_ret: TResult = unsafe {
            cordl_method_info.invoke_unchecked((), (source, selector))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OfType<TResult>(
        source: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::IEnumerable,
                        >),
                        quest_hook::libil2cpp::Gc<TResult>,
                        1usize,
                    >("OfType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "OfType",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = unsafe {
            cordl_method_info.invoke_unchecked((), (source))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OfTypeIterator<TResult>(
        source: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::IEnumerable,
                        >),
                        quest_hook::libil2cpp::Gc<TResult>,
                        1usize,
                    >("OfTypeIterator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OfTypeIterator", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = unsafe {
            cordl_method_info.invoke_unchecked((), (source))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OrderByDescending<TSource, TKey>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        keySelector: quest_hook::libil2cpp::Gc<TSource, TKey>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
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
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource, TKey>,
                        ),
                        quest_hook::libil2cpp::Gc<TSource>,
                        2usize,
                    >("OrderByDescending")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OrderByDescending", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked((), (source, keySelector))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OrderBy_Gc1<TSource, TKey>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        keySelector: quest_hook::libil2cpp::Gc<TSource, TKey>,
        comparer: quest_hook::libil2cpp::Gc<TKey>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
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
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource, TKey>,
                            quest_hook::libil2cpp::Gc<TKey>,
                        ),
                        quest_hook::libil2cpp::Gc<TSource>,
                        3usize,
                    >("OrderBy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "OrderBy",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked((), (source, keySelector, comparer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OrderBy_Gc_Gc0<TSource, TKey>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        keySelector: quest_hook::libil2cpp::Gc<TSource, TKey>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
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
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource, TKey>,
                        ),
                        quest_hook::libil2cpp::Gc<TSource>,
                        2usize,
                    >("OrderBy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "OrderBy",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked((), (source, keySelector))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Range(
        start: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<i32>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32, i32),
                        quest_hook::libil2cpp::Gc<i32>,
                        2usize,
                    >("Range")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Range",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<i32> = unsafe {
            cordl_method_info.invoke_unchecked((), (start, count))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RangeIterator(
        start: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<i32>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32, i32),
                        quest_hook::libil2cpp::Gc<i32>,
                        2usize,
                    >("RangeIterator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RangeIterator", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<i32> = unsafe {
            cordl_method_info.invoke_unchecked((), (start, count))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Reverse<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<TSource>),
                        quest_hook::libil2cpp::Gc<TSource>,
                        1usize,
                    >("Reverse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Reverse",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked((), (source))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReverseIterator<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<TSource>),
                        quest_hook::libil2cpp::Gc<TSource>,
                        1usize,
                    >("ReverseIterator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReverseIterator", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked((), (source))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SelectIterator<TSource, TResult>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        selector: quest_hook::libil2cpp::Gc<TSource, i32, TResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource, i32, TResult>,
                        ),
                        quest_hook::libil2cpp::Gc<TResult>,
                        2usize,
                    >("SelectIterator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SelectIterator", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = unsafe {
            cordl_method_info.invoke_unchecked((), (source, selector))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SelectMany<TSource, TResult>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        selector: quest_hook::libil2cpp::Gc<TSource, quest_hook::libil2cpp::Gc<TResult>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<
                                TSource,
                                quest_hook::libil2cpp::Gc<TResult>,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<TResult>,
                        2usize,
                    >("SelectMany")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SelectMany", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = unsafe {
            cordl_method_info.invoke_unchecked((), (source, selector))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SelectManyIterator<TSource, TResult>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        selector: quest_hook::libil2cpp::Gc<TSource, quest_hook::libil2cpp::Gc<TResult>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<
                                TSource,
                                quest_hook::libil2cpp::Gc<TResult>,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<TResult>,
                        2usize,
                    >("SelectManyIterator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SelectManyIterator", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = unsafe {
            cordl_method_info.invoke_unchecked((), (source, selector))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Select_Gc_Gc0<TSource, TResult>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        selector: quest_hook::libil2cpp::Gc<TSource, TResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource, TResult>,
                        ),
                        quest_hook::libil2cpp::Gc<TResult>,
                        2usize,
                    >("Select")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Select",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = unsafe {
            cordl_method_info.invoke_unchecked((), (source, selector))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Select_Gc_Gc1<TSource, TResult>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        selector: quest_hook::libil2cpp::Gc<TSource, i32, TResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource, i32, TResult>,
                        ),
                        quest_hook::libil2cpp::Gc<TResult>,
                        2usize,
                    >("Select")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Select",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = unsafe {
            cordl_method_info.invoke_unchecked((), (source, selector))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SequenceEqual_Gc1<TSource>(
        first: quest_hook::libil2cpp::Gc<TSource>,
        second: quest_hook::libil2cpp::Gc<TSource>,
        comparer: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource>,
                        ),
                        bool,
                        3usize,
                    >("SequenceEqual")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SequenceEqual", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (first, second, comparer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SequenceEqual_Gc_Gc0<TSource>(
        first: quest_hook::libil2cpp::Gc<TSource>,
        second: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource>,
                        ),
                        bool,
                        2usize,
                    >("SequenceEqual")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SequenceEqual", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (first, second))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SingleOrDefault_Gc0<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<TSource>),
                        TSource,
                        1usize,
                    >("SingleOrDefault")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SingleOrDefault", 1usize
                        )
                    })
            });
        let __cordl_ret: TSource = unsafe {
            cordl_method_info.invoke_unchecked((), (source))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SingleOrDefault_Gc1<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource, bool>,
                        ),
                        TSource,
                        2usize,
                    >("SingleOrDefault")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SingleOrDefault", 2usize
                        )
                    })
            });
        let __cordl_ret: TSource = unsafe {
            cordl_method_info.invoke_unchecked((), (source, predicate))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Single_Gc0<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<TSource>),
                        TSource,
                        1usize,
                    >("Single")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Single",
                            1usize
                        )
                    })
            });
        let __cordl_ret: TSource = unsafe {
            cordl_method_info.invoke_unchecked((), (source))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Single_Gc1<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource, bool>,
                        ),
                        TSource,
                        2usize,
                    >("Single")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Single",
                            2usize
                        )
                    })
            });
        let __cordl_ret: TSource = unsafe {
            cordl_method_info.invoke_unchecked((), (source, predicate))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Skip<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<TSource>, i32),
                        quest_hook::libil2cpp::Gc<TSource>,
                        2usize,
                    >("Skip")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Skip",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked((), (source, count))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SkipIterator<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<TSource>, i32),
                        quest_hook::libil2cpp::Gc<TSource>,
                        2usize,
                    >("SkipIterator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SkipIterator", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked((), (source, count))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Sum_Gc0(
        source: quest_hook::libil2cpp::Gc<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<i32>),
                        i32,
                        1usize,
                    >("Sum")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Sum",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (source))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Sum_Gc1(
        source: quest_hook::libil2cpp::Gc<f32>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<f32>),
                        f32,
                        1usize,
                    >("Sum")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Sum",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked((), (source))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Sum_Gc2<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        selector: quest_hook::libil2cpp::Gc<TSource, i32>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource, i32>,
                        ),
                        i32,
                        2usize,
                    >("Sum")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Sum",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (source, selector))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Take<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<TSource>, i32),
                        quest_hook::libil2cpp::Gc<TSource>,
                        2usize,
                    >("Take")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Take",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked((), (source, count))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TakeIterator<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<TSource>, i32),
                        quest_hook::libil2cpp::Gc<TSource>,
                        2usize,
                    >("TakeIterator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TakeIterator", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked((), (source, count))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ThenBy<TSource, TKey>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        keySelector: quest_hook::libil2cpp::Gc<TSource, TKey>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
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
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource, TKey>,
                        ),
                        quest_hook::libil2cpp::Gc<TSource>,
                        2usize,
                    >("ThenBy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "ThenBy",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked((), (source, keySelector))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToArray<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TSource>>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<TSource>),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<TSource>,
                        >,
                        1usize,
                    >("ToArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "ToArray",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<TSource>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (source))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToDictionary_Gc1<TSource, TKey, TElement>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        keySelector: quest_hook::libil2cpp::Gc<TSource, TKey>,
        elementSelector: quest_hook::libil2cpp::Gc<TSource, TElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TKey, TElement>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource, TKey>,
                            quest_hook::libil2cpp::Gc<TSource, TElement>,
                        ),
                        quest_hook::libil2cpp::Gc<TKey, TElement>,
                        3usize,
                    >("ToDictionary")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToDictionary", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TKey, TElement> = unsafe {
            cordl_method_info
                .invoke_unchecked((), (source, keySelector, elementSelector))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToDictionary_Gc_Gc0<TSource, TKey>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        keySelector: quest_hook::libil2cpp::Gc<TSource, TKey>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TKey, TSource>>
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
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource, TKey>,
                        ),
                        quest_hook::libil2cpp::Gc<TKey, TSource>,
                        2usize,
                    >("ToDictionary")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToDictionary", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TKey, TSource> = unsafe {
            cordl_method_info.invoke_unchecked((), (source, keySelector))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToDictionary_Gc_Gc2<TSource, TKey, TElement>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        keySelector: quest_hook::libil2cpp::Gc<TSource, TKey>,
        elementSelector: quest_hook::libil2cpp::Gc<TSource, TElement>,
        comparer: quest_hook::libil2cpp::Gc<TKey>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TKey, TElement>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource, TKey>,
                            quest_hook::libil2cpp::Gc<TSource, TElement>,
                            quest_hook::libil2cpp::Gc<TKey>,
                        ),
                        quest_hook::libil2cpp::Gc<TKey, TElement>,
                        4usize,
                    >("ToDictionary")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToDictionary", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TKey, TElement> = unsafe {
            cordl_method_info
                .invoke_unchecked((), (source, keySelector, elementSelector, comparer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToHashSet_Gc0<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<TSource>),
                        quest_hook::libil2cpp::Gc<TSource>,
                        1usize,
                    >("ToHashSet")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToHashSet", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked((), (source))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToHashSet_Gc1<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        comparer: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource>,
                        ),
                        quest_hook::libil2cpp::Gc<TSource>,
                        2usize,
                    >("ToHashSet")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToHashSet", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked((), (source, comparer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToList<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<TSource>),
                        quest_hook::libil2cpp::Gc<TSource>,
                        1usize,
                    >("ToList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "ToList",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked((), (source))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Union<TSource>(
        first: quest_hook::libil2cpp::Gc<TSource>,
        second: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource>,
                        ),
                        quest_hook::libil2cpp::Gc<TSource>,
                        2usize,
                    >("Union")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Union",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked((), (first, second))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnionIterator<TSource>(
        first: quest_hook::libil2cpp::Gc<TSource>,
        second: quest_hook::libil2cpp::Gc<TSource>,
        comparer: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource>,
                        ),
                        quest_hook::libil2cpp::Gc<TSource>,
                        3usize,
                    >("UnionIterator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnionIterator", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked((), (first, second, comparer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Where<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource, bool>,
                        ),
                        quest_hook::libil2cpp::Gc<TSource>,
                        2usize,
                    >("Where")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Where",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked((), (source, predicate))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Zip<TFirst, TSecond, TResult>(
        first: quest_hook::libil2cpp::Gc<TFirst>,
        second: quest_hook::libil2cpp::Gc<TSecond>,
        resultSelector: quest_hook::libil2cpp::Gc<TFirst, TSecond, TResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TFirst: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TSecond: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TFirst>,
                            quest_hook::libil2cpp::Gc<TSecond>,
                            quest_hook::libil2cpp::Gc<TFirst, TSecond, TResult>,
                        ),
                        quest_hook::libil2cpp::Gc<TResult>,
                        3usize,
                    >("Zip")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Zip",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = unsafe {
            cordl_method_info.invoke_unchecked((), (first, second, resultSelector))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ZipIterator<TFirst, TSecond, TResult>(
        first: quest_hook::libil2cpp::Gc<TFirst>,
        second: quest_hook::libil2cpp::Gc<TSecond>,
        resultSelector: quest_hook::libil2cpp::Gc<TFirst, TSecond, TResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TFirst: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TSecond: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<TFirst>,
                            quest_hook::libil2cpp::Gc<TSecond>,
                            quest_hook::libil2cpp::Gc<TFirst, TSecond, TResult>,
                        ),
                        quest_hook::libil2cpp::Gc<TResult>,
                        3usize,
                    >("ZipIterator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ZipIterator", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = unsafe {
            cordl_method_info.invoke_unchecked((), (first, second, resultSelector))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Linq+Enumerable")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Linq::Enumerable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_System+Linq+Enumerable+Iterator_1")]
#[repr(C)]
#[derive(Debug)]
pub struct Enumerable_Iterator_1<TSource: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub threadId: i32,
    pub state: i32,
    pub current: TSource,
    __cordl_phantom_TSource: std::marker::PhantomData<TSource>,
}
#[cfg(feature = "cordl_class_System+Linq+Enumerable+Iterator_1")]
unsafe impl<TSource: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq";
    const CLASS_NAME: &'static str = "Enumerable/Iterator`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "System.Linq",
                        "Enumerable/Iterator`1",
                    )
                    .unwrap()
                    .make_generic::<(TSource)>()
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
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<
    TSource: quest_hook::libil2cpp::Type,
> crate::System::Linq::Enumerable_Iterator_1<TSource> {
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<TSource>,
                        0usize,
                    >("Clone")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Clone",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<TSource>,
                        0usize,
                    >("GetEnumerator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetEnumerator", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("MoveNext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MoveNext", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Select<TResult>(
        &mut self,
        selector: quest_hook::libil2cpp::Gc<TSource, TResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<TSource, TResult>),
                        quest_hook::libil2cpp::Gc<TResult>,
                        1usize,
                    >("Select")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Select",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = unsafe {
            cordl_method_info.invoke_unchecked(self, (selector))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::IEnumerator,
                        >,
                        0usize,
                    >("System.Collections.IEnumerable.GetEnumerator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.Collections.IEnumerable.GetEnumerator", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerator_Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("System.Collections.IEnumerator.Reset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.Collections.IEnumerator.Reset", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerator_get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        0usize,
                    >("System.Collections.IEnumerator.get_Current")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.Collections.IEnumerator.get_Current", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Where(
        &mut self,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<TSource, bool>),
                        quest_hook::libil2cpp::Gc<TSource>,
                        1usize,
                    >("Where")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Where",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked(self, (predicate))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
    pub fn get_Current(&mut self) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), TSource, 0usize>("get_Current")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Current", 0usize
                        )
                    })
            });
        let __cordl_ret: TSource = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Linq+Enumerable+Iterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> AsRef<crate::System::Collections::IEnumerable>
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> AsMut<crate::System::Collections::IEnumerable>
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> AsRef<crate::System::Collections::IEnumerator>
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> AsMut<crate::System::Collections::IEnumerator>
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> AsRef<crate::System::IDisposable>
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> AsMut<crate::System::IDisposable>
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> AsRef<quest_hook::libil2cpp::Gc<TSource>>
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<TSource> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> AsMut<quest_hook::libil2cpp::Gc<TSource>>
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<TSource> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> AsRef<quest_hook::libil2cpp::Gc<TSource>>
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<TSource> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> AsMut<quest_hook::libil2cpp::Gc<TSource>>
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<TSource> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "cordl_class_System+Linq+Enumerable+WhereArrayIterator_1")]
#[repr(C)]
#[derive(Debug)]
pub struct Enumerable_WhereArrayIterator_1<TSource: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Gc<TSource>,
    pub source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TSource>>,
    pub predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    pub index: i32,
    __cordl_phantom_TSource: std::marker::PhantomData<TSource>,
}
#[cfg(feature = "cordl_class_System+Linq+Enumerable+WhereArrayIterator_1")]
unsafe impl<TSource: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::System::Linq::Enumerable_WhereArrayIterator_1<TSource> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq";
    const CLASS_NAME: &'static str = "Enumerable/WhereArrayIterator`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "System.Linq",
                        "Enumerable/WhereArrayIterator`1",
                    )
                    .unwrap()
                    .make_generic::<(TSource)>()
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
#[cfg(feature = "System+Linq+Enumerable+WhereArrayIterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Linq::Enumerable_WhereArrayIterator_1<TSource> {
    type Target = quest_hook::libil2cpp::Gc<TSource>;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereArrayIterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Linq::Enumerable_WhereArrayIterator_1<TSource> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereArrayIterator_1")]
impl<
    TSource: quest_hook::libil2cpp::Type,
> crate::System::Linq::Enumerable_WhereArrayIterator_1<TSource> {
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<TSource>,
                        0usize,
                    >("Clone")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Clone",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("MoveNext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MoveNext", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TSource>>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (source, predicate))?;
        Ok(__cordl_object.into())
    }
    pub fn Select<TResult>(
        &mut self,
        selector: quest_hook::libil2cpp::Gc<TSource, TResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<TSource, TResult>),
                        quest_hook::libil2cpp::Gc<TResult>,
                        1usize,
                    >("Select")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Select",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = unsafe {
            cordl_method_info.invoke_unchecked(self, (selector))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Where(
        &mut self,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<TSource, bool>),
                        quest_hook::libil2cpp::Gc<TSource>,
                        1usize,
                    >("Where")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Where",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked(self, (predicate))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TSource>>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TSource>,
                            >,
                            quest_hook::libil2cpp::Gc<TSource, bool>,
                        ),
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
            cordl_method_info.invoke_unchecked(self, (source, predicate))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Linq+Enumerable+WhereArrayIterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Enumerable_WhereArrayIterator_1<TSource> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_System+Linq+Enumerable+WhereEnumerableIterator_1")]
#[repr(C)]
#[derive(Debug)]
pub struct Enumerable_WhereEnumerableIterator_1<TSource: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Gc<TSource>,
    pub source: quest_hook::libil2cpp::Gc<TSource>,
    pub predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    pub enumerator: quest_hook::libil2cpp::Gc<TSource>,
    __cordl_phantom_TSource: std::marker::PhantomData<TSource>,
}
#[cfg(feature = "cordl_class_System+Linq+Enumerable+WhereEnumerableIterator_1")]
unsafe impl<TSource: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::System::Linq::Enumerable_WhereEnumerableIterator_1<TSource> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq";
    const CLASS_NAME: &'static str = "Enumerable/WhereEnumerableIterator`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "System.Linq",
                        "Enumerable/WhereEnumerableIterator`1",
                    )
                    .unwrap()
                    .make_generic::<(TSource)>()
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
#[cfg(feature = "System+Linq+Enumerable+WhereEnumerableIterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Linq::Enumerable_WhereEnumerableIterator_1<TSource> {
    type Target = quest_hook::libil2cpp::Gc<TSource>;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereEnumerableIterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Linq::Enumerable_WhereEnumerableIterator_1<TSource> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereEnumerableIterator_1")]
impl<
    TSource: quest_hook::libil2cpp::Type,
> crate::System::Linq::Enumerable_WhereEnumerableIterator_1<TSource> {
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<TSource>,
                        0usize,
                    >("Clone")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Clone",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("MoveNext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MoveNext", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (source, predicate))?;
        Ok(__cordl_object.into())
    }
    pub fn Select<TResult>(
        &mut self,
        selector: quest_hook::libil2cpp::Gc<TSource, TResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<TSource, TResult>),
                        quest_hook::libil2cpp::Gc<TResult>,
                        1usize,
                    >("Select")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Select",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = unsafe {
            cordl_method_info.invoke_unchecked(self, (selector))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Where(
        &mut self,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<TSource, bool>),
                        quest_hook::libil2cpp::Gc<TSource>,
                        1usize,
                    >("Where")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Where",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked(self, (predicate))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource, bool>,
                        ),
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
            cordl_method_info.invoke_unchecked(self, (source, predicate))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Linq+Enumerable+WhereEnumerableIterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Enumerable_WhereEnumerableIterator_1<TSource> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_System+Linq+Enumerable+WhereListIterator_1")]
#[repr(C)]
#[derive(Debug)]
pub struct Enumerable_WhereListIterator_1<TSource: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Gc<TSource>,
    pub source: quest_hook::libil2cpp::Gc<TSource>,
    pub predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    pub enumerator: crate::System::Collections::Generic::List_1_Enumerator<TSource>,
    __cordl_phantom_TSource: std::marker::PhantomData<TSource>,
}
#[cfg(feature = "cordl_class_System+Linq+Enumerable+WhereListIterator_1")]
unsafe impl<TSource: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::System::Linq::Enumerable_WhereListIterator_1<TSource> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq";
    const CLASS_NAME: &'static str = "Enumerable/WhereListIterator`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "System.Linq",
                        "Enumerable/WhereListIterator`1",
                    )
                    .unwrap()
                    .make_generic::<(TSource)>()
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
#[cfg(feature = "System+Linq+Enumerable+WhereListIterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Linq::Enumerable_WhereListIterator_1<TSource> {
    type Target = quest_hook::libil2cpp::Gc<TSource>;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereListIterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Linq::Enumerable_WhereListIterator_1<TSource> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereListIterator_1")]
impl<
    TSource: quest_hook::libil2cpp::Type,
> crate::System::Linq::Enumerable_WhereListIterator_1<TSource> {
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<TSource>,
                        0usize,
                    >("Clone")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Clone",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("MoveNext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MoveNext", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (source, predicate))?;
        Ok(__cordl_object.into())
    }
    pub fn Select<TResult>(
        &mut self,
        selector: quest_hook::libil2cpp::Gc<TSource, TResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<TSource, TResult>),
                        quest_hook::libil2cpp::Gc<TResult>,
                        1usize,
                    >("Select")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Select",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = unsafe {
            cordl_method_info.invoke_unchecked(self, (selector))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Where(
        &mut self,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<TSource, bool>),
                        quest_hook::libil2cpp::Gc<TSource>,
                        1usize,
                    >("Where")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Where",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = unsafe {
            cordl_method_info.invoke_unchecked(self, (predicate))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource, bool>,
                        ),
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
            cordl_method_info.invoke_unchecked(self, (source, predicate))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Linq+Enumerable+WhereListIterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Enumerable_WhereListIterator_1<TSource> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_System+Linq+Enumerable+WhereSelectArrayIterator_2")]
#[repr(C)]
#[derive(Debug)]
pub struct Enumerable_WhereSelectArrayIterator_2<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Gc<TResult>,
    pub source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TSource>>,
    pub predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    pub selector: quest_hook::libil2cpp::Gc<TSource, TResult>,
    pub index: i32,
    __cordl_phantom_TSource: std::marker::PhantomData<TSource>,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
}
#[cfg(feature = "cordl_class_System+Linq+Enumerable+WhereSelectArrayIterator_2")]
unsafe impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::System::Linq::Enumerable_WhereSelectArrayIterator_2<TSource, TResult> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq";
    const CLASS_NAME: &'static str = "Enumerable/WhereSelectArrayIterator`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "System.Linq",
                        "Enumerable/WhereSelectArrayIterator`2",
                    )
                    .unwrap()
                    .make_generic::<(TSource, TResult)>()
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
#[cfg(feature = "System+Linq+Enumerable+WhereSelectArrayIterator_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::System::Linq::Enumerable_WhereSelectArrayIterator_2<TSource, TResult> {
    type Target = quest_hook::libil2cpp::Gc<TResult>;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereSelectArrayIterator_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::System::Linq::Enumerable_WhereSelectArrayIterator_2<TSource, TResult> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereSelectArrayIterator_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> crate::System::Linq::Enumerable_WhereSelectArrayIterator_2<TSource, TResult> {
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<TResult>,
                        0usize,
                    >("Clone")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Clone",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("MoveNext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MoveNext", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TSource>>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
        selector: quest_hook::libil2cpp::Gc<TSource, TResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (source, predicate, selector))?;
        Ok(__cordl_object.into())
    }
    pub fn Select<TResult2>(
        &mut self,
        selector: quest_hook::libil2cpp::Gc<TResult, TResult2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult2>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<TResult, TResult2>),
                        quest_hook::libil2cpp::Gc<TResult2>,
                        1usize,
                    >("Select")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Select",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult2> = unsafe {
            cordl_method_info.invoke_unchecked(self, (selector))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Where(
        &mut self,
        predicate: quest_hook::libil2cpp::Gc<TResult, bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<TResult, bool>),
                        quest_hook::libil2cpp::Gc<TResult>,
                        1usize,
                    >("Where")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Where",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = unsafe {
            cordl_method_info.invoke_unchecked(self, (predicate))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TSource>>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
        selector: quest_hook::libil2cpp::Gc<TSource, TResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TSource>,
                            >,
                            quest_hook::libil2cpp::Gc<TSource, bool>,
                            quest_hook::libil2cpp::Gc<TSource, TResult>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (source, predicate, selector))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Linq+Enumerable+WhereSelectArrayIterator_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Enumerable_WhereSelectArrayIterator_2<TSource, TResult> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_System+Linq+Enumerable+WhereSelectEnumerableIterator_2")]
#[repr(C)]
#[derive(Debug)]
pub struct Enumerable_WhereSelectEnumerableIterator_2<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Gc<TResult>,
    pub source: quest_hook::libil2cpp::Gc<TSource>,
    pub predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    pub selector: quest_hook::libil2cpp::Gc<TSource, TResult>,
    pub enumerator: quest_hook::libil2cpp::Gc<TSource>,
    __cordl_phantom_TSource: std::marker::PhantomData<TSource>,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
}
#[cfg(feature = "cordl_class_System+Linq+Enumerable+WhereSelectEnumerableIterator_2")]
unsafe impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::System::Linq::Enumerable_WhereSelectEnumerableIterator_2<TSource, TResult> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq";
    const CLASS_NAME: &'static str = "Enumerable/WhereSelectEnumerableIterator`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "System.Linq",
                        "Enumerable/WhereSelectEnumerableIterator`2",
                    )
                    .unwrap()
                    .make_generic::<(TSource, TResult)>()
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
#[cfg(feature = "System+Linq+Enumerable+WhereSelectEnumerableIterator_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::System::Linq::Enumerable_WhereSelectEnumerableIterator_2<TSource, TResult> {
    type Target = quest_hook::libil2cpp::Gc<TResult>;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereSelectEnumerableIterator_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::System::Linq::Enumerable_WhereSelectEnumerableIterator_2<TSource, TResult> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereSelectEnumerableIterator_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> crate::System::Linq::Enumerable_WhereSelectEnumerableIterator_2<TSource, TResult> {
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<TResult>,
                        0usize,
                    >("Clone")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Clone",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("MoveNext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MoveNext", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
        selector: quest_hook::libil2cpp::Gc<TSource, TResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (source, predicate, selector))?;
        Ok(__cordl_object.into())
    }
    pub fn Select<TResult2>(
        &mut self,
        selector: quest_hook::libil2cpp::Gc<TResult, TResult2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult2>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<TResult, TResult2>),
                        quest_hook::libil2cpp::Gc<TResult2>,
                        1usize,
                    >("Select")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Select",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult2> = unsafe {
            cordl_method_info.invoke_unchecked(self, (selector))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Where(
        &mut self,
        predicate: quest_hook::libil2cpp::Gc<TResult, bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<TResult, bool>),
                        quest_hook::libil2cpp::Gc<TResult>,
                        1usize,
                    >("Where")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Where",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = unsafe {
            cordl_method_info.invoke_unchecked(self, (predicate))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
        selector: quest_hook::libil2cpp::Gc<TSource, TResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource, bool>,
                            quest_hook::libil2cpp::Gc<TSource, TResult>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (source, predicate, selector))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Linq+Enumerable+WhereSelectEnumerableIterator_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Enumerable_WhereSelectEnumerableIterator_2<TSource, TResult> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_System+Linq+Enumerable+WhereSelectListIterator_2")]
#[repr(C)]
#[derive(Debug)]
pub struct Enumerable_WhereSelectListIterator_2<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Gc<TResult>,
    pub source: quest_hook::libil2cpp::Gc<TSource>,
    pub predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    pub selector: quest_hook::libil2cpp::Gc<TSource, TResult>,
    pub enumerator: crate::System::Collections::Generic::List_1_Enumerator<TSource>,
    __cordl_phantom_TSource: std::marker::PhantomData<TSource>,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
}
#[cfg(feature = "cordl_class_System+Linq+Enumerable+WhereSelectListIterator_2")]
unsafe impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::System::Linq::Enumerable_WhereSelectListIterator_2<TSource, TResult> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq";
    const CLASS_NAME: &'static str = "Enumerable/WhereSelectListIterator`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "System.Linq",
                        "Enumerable/WhereSelectListIterator`2",
                    )
                    .unwrap()
                    .make_generic::<(TSource, TResult)>()
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
#[cfg(feature = "System+Linq+Enumerable+WhereSelectListIterator_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::System::Linq::Enumerable_WhereSelectListIterator_2<TSource, TResult> {
    type Target = quest_hook::libil2cpp::Gc<TResult>;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereSelectListIterator_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::System::Linq::Enumerable_WhereSelectListIterator_2<TSource, TResult> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereSelectListIterator_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> crate::System::Linq::Enumerable_WhereSelectListIterator_2<TSource, TResult> {
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<TResult>,
                        0usize,
                    >("Clone")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Clone",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("MoveNext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MoveNext", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
        selector: quest_hook::libil2cpp::Gc<TSource, TResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (source, predicate, selector))?;
        Ok(__cordl_object.into())
    }
    pub fn Select<TResult2>(
        &mut self,
        selector: quest_hook::libil2cpp::Gc<TResult, TResult2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult2>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<TResult, TResult2>),
                        quest_hook::libil2cpp::Gc<TResult2>,
                        1usize,
                    >("Select")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Select",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult2> = unsafe {
            cordl_method_info.invoke_unchecked(self, (selector))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Where(
        &mut self,
        predicate: quest_hook::libil2cpp::Gc<TResult, bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<TResult, bool>),
                        quest_hook::libil2cpp::Gc<TResult>,
                        1usize,
                    >("Where")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Where",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = unsafe {
            cordl_method_info.invoke_unchecked(self, (predicate))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
        selector: quest_hook::libil2cpp::Gc<TSource, TResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<TSource>,
                            quest_hook::libil2cpp::Gc<TSource, bool>,
                            quest_hook::libil2cpp::Gc<TSource, TResult>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (source, predicate, selector))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Linq+Enumerable+WhereSelectListIterator_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Enumerable_WhereSelectListIterator_2<TSource, TResult> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
