#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct CollectionUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Utilities::CollectionUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Utilities";
    const CLASS_NAME: &'static str = "CollectionUtils";
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
#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::CollectionUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::CollectionUtils {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils")]
impl crate::Newtonsoft::Json::Utilities::CollectionUtils {
    #[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils+EmptyArrayContainer_1")]
    pub type EmptyArrayContainer_1<T: quest_hook::libil2cpp::Type> = crate::Newtonsoft::Json::Utilities::CollectionUtils_EmptyArrayContainer_1<
        T,
    >;
    pub fn AddDistinct_IEqualityComparer_1_1<T>(
        list: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IList_1<T>>,
        value: T,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEqualityComparer_1<T>,
        >,
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
                                crate::System::Collections::Generic::IList_1<T>,
                            >,
                            T,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IEqualityComparer_1<T>,
                            >,
                        ),
                        bool,
                        3usize,
                    >("AddDistinct")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddDistinct", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (list, value, comparer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddDistinct_IList_1_T0<T>(
        list: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IList_1<T>>,
        value: T,
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
                                crate::System::Collections::Generic::IList_1<T>,
                            >,
                            T,
                        ),
                        bool,
                        2usize,
                    >("AddDistinct")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddDistinct", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (list, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddRange<T>(
        initial: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<T>,
        >,
        collection: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
                                crate::System::Collections::Generic::IEnumerable_1<T>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
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
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (initial, collection))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddRangeDistinct<T>(
        list: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IList_1<T>>,
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        >,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEqualityComparer_1<T>,
        >,
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
                                crate::System::Collections::Generic::IList_1<T>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IEnumerable_1<T>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IEqualityComparer_1<T>,
                            >,
                        ),
                        bool,
                        3usize,
                    >("AddRangeDistinct")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddRangeDistinct", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (list, values, comparer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ArrayEmpty<T>() -> quest_hook::libil2cpp::Result<
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
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
                        0usize,
                    >("ArrayEmpty")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ArrayEmpty", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Contains<T>(
        list: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
        value: T,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEqualityComparer,
        >,
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
                                crate::System::Collections::Generic::List_1<T>,
                            >,
                            T,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::IEqualityComparer,
                            >,
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
            cordl_method_info.invoke_unchecked((), (list, value, comparer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ContainsValue<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        value: TSource,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEqualityComparer_1<TSource>,
        >,
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
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IEnumerable_1<TSource>,
                            >,
                            TSource,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IEqualityComparer_1<
                                    TSource,
                                >,
                            >,
                        ),
                        bool,
                        3usize,
                    >("ContainsValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ContainsValue", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (source, value, comparer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyFromJaggedToMultidimensionalArray(
        values: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        multidimensionalArray: quest_hook::libil2cpp::Gc<crate::System::Array>,
        indices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                            quest_hook::libil2cpp::Gc<crate::System::Array>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<i32>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("CopyFromJaggedToMultidimensionalArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CopyFromJaggedToMultidimensionalArray", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (values, multidimensionalArray, indices))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FastReverse<T>(
        list: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<T>,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("FastReverse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FastReverse", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (list))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDimensions(
        values: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        dimensionsCount: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IList_1<i32>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IList_1<i32>,
                        >,
                        2usize,
                    >("GetDimensions")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDimensions", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (values, dimensionsCount))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf<T>(
        collection: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        >,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<T, bool>>,
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
                                crate::System::Collections::Generic::IEnumerable_1<T>,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Func_2<T, bool>>,
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
            cordl_method_info.invoke_unchecked((), (collection, predicate))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOfReference<T>(
        list: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
        item: T,
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
                                crate::System::Collections::Generic::List_1<T>,
                            >,
                            T,
                        ),
                        i32,
                        2usize,
                    >("IndexOfReference")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IndexOfReference", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (list, item))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsDictionaryType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        bool,
                        1usize,
                    >("IsDictionaryType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsDictionaryType", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (_cordl_type))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsNullOrEmpty<T>(
        collection: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::ICollection_1<T>,
        >,
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
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::ICollection_1<T>,
                        >),
                        bool,
                        1usize,
                    >("IsNullOrEmpty")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsNullOrEmpty", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (collection))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn JaggedArrayGetValue(
        values: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        indices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<i32>,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        2usize,
                    >("JaggedArrayGetValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "JaggedArrayGetValue", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { cordl_method_info.invoke_unchecked((), (values, indices))? };
        Ok(__cordl_ret.into())
    }
    pub fn ResolveEnumerableCollectionConstructor_Type1(
        collectionType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        collectionItemType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        constructorArgumentType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::ConstructorInfo>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Reflection::ConstructorInfo,
                        >,
                        3usize,
                    >("ResolveEnumerableCollectionConstructor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ResolveEnumerableCollectionConstructor", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ConstructorInfo,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (collectionType, collectionItemType, constructorArgumentType),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResolveEnumerableCollectionConstructor_Type_Type0(
        collectionType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        collectionItemType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::ConstructorInfo>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Reflection::ConstructorInfo,
                        >,
                        2usize,
                    >("ResolveEnumerableCollectionConstructor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ResolveEnumerableCollectionConstructor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ConstructorInfo,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (collectionType, collectionItemType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToMultidimensionalArray(
        values: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        rank: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Array>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Array>,
                        3usize,
                    >("ToMultidimensionalArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToMultidimensionalArray", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Array> = unsafe {
            cordl_method_info.invoke_unchecked((), (values, _cordl_type, rank))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::CollectionUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils+EmptyArrayContainer_1")]
#[repr(C)]
#[derive(Debug)]
pub struct CollectionUtils_EmptyArrayContainer_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils+EmptyArrayContainer_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Utilities::CollectionUtils_EmptyArrayContainer_1<T> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Utilities";
    const CLASS_NAME: &'static str = "CollectionUtils/EmptyArrayContainer`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "Newtonsoft.Json.Utilities",
                        "CollectionUtils/EmptyArrayContainer`1",
                    )
                    .unwrap()
                    .make_generic::<(T)>()
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
#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils+EmptyArrayContainer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Newtonsoft::Json::Utilities::CollectionUtils_EmptyArrayContainer_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils+EmptyArrayContainer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Newtonsoft::Json::Utilities::CollectionUtils_EmptyArrayContainer_1<T> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils+EmptyArrayContainer_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::Newtonsoft::Json::Utilities::CollectionUtils_EmptyArrayContainer_1<T> {}
#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils+EmptyArrayContainer_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::CollectionUtils_EmptyArrayContainer_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
