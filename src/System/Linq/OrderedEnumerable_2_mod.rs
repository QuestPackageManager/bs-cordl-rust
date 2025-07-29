#[cfg(feature = "cordl_class_System+Linq+OrderedEnumerable_2")]
#[repr(C)]
#[derive(Debug)]
pub struct OrderedEnumerable_2<
    TElement: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::Linq::OrderedEnumerable_1<TElement>,
    pub parent: quest_hook::libil2cpp::Gc<
        crate::System::Linq::OrderedEnumerable_1<TElement>,
    >,
    pub keySelector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TElement, TKey>>,
    pub comparer: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IComparer_1<TKey>,
    >,
    pub descending: bool,
    __cordl_phantom_TElement: std::marker::PhantomData<TElement>,
    __cordl_phantom_TKey: std::marker::PhantomData<TKey>,
}
#[cfg(feature = "cordl_class_System+Linq+OrderedEnumerable_2")]
unsafe impl<
    TElement: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::System::Linq::OrderedEnumerable_2<TElement, TKey> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq";
    const CLASS_NAME: &'static str = "OrderedEnumerable`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "System.Linq",
                        "OrderedEnumerable`2",
                    )
                    .unwrap()
                    .make_generic::<(TElement, TKey)>()
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
#[cfg(feature = "System+Linq+OrderedEnumerable_2")]
impl<
    TElement: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
> std::ops::Deref for crate::System::Linq::OrderedEnumerable_2<TElement, TKey> {
    type Target = crate::System::Linq::OrderedEnumerable_1<TElement>;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+OrderedEnumerable_2")]
impl<
    TElement: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
> std::ops::DerefMut for crate::System::Linq::OrderedEnumerable_2<TElement, TKey> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+OrderedEnumerable_2")]
impl<
    TElement: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
> crate::System::Linq::OrderedEnumerable_2<TElement, TKey> {
    pub fn GetEnumerableSorter(
        &mut self,
        next: quest_hook::libil2cpp::Gc<
            crate::System::Linq::EnumerableSorter_1<TElement>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::EnumerableSorter_1<TElement>>,
    >
    where
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Linq::EnumerableSorter_1<TElement>,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::EnumerableSorter_1<TElement>,
                        >,
                        1usize,
                    >("GetEnumerableSorter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetEnumerableSorter", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::EnumerableSorter_1<TElement>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (next))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TElement>,
        >,
        keySelector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TElement, TKey>>,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IComparer_1<TKey>,
        >,
        descending: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (source, keySelector, comparer, descending))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TElement>,
        >,
        keySelector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TElement, TKey>>,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IComparer_1<TKey>,
        >,
        descending: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IEnumerable_1<TElement>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_2<TElement, TKey>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IComparer_1<TKey>,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (source, keySelector, comparer, descending))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Linq+OrderedEnumerable_2")]
impl<
    TElement: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::System::Linq::OrderedEnumerable_2<TElement, TKey> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
