#[cfg(feature = "System+Linq+EnumerableSorter_2")]
#[repr(C)]
#[derive(Debug)]
pub struct EnumerableSorter_2<
    TElement: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::Linq::EnumerableSorter_1<TElement>,
    pub keySelector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TElement, TKey>>,
    pub comparer: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IComparer_1<TKey>,
    >,
    pub descending: bool,
    pub next: quest_hook::libil2cpp::Gc<
        crate::System::Linq::EnumerableSorter_1<TElement>,
    >,
    pub keys: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TKey>>,
    __cordl_phantom_TElement: std::marker::PhantomData<TElement>,
    __cordl_phantom_TKey: std::marker::PhantomData<TKey>,
}
#[cfg(feature = "System+Linq+EnumerableSorter_2")]
unsafe impl<
    TElement: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::System::Linq::EnumerableSorter_2<TElement, TKey> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq";
    const CLASS_NAME: &'static str = "EnumerableSorter`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "System.Linq",
                        "EnumerableSorter`2",
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
#[cfg(feature = "System+Linq+EnumerableSorter_2")]
impl<
    TElement: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
> std::ops::Deref for crate::System::Linq::EnumerableSorter_2<TElement, TKey> {
    type Target = crate::System::Linq::EnumerableSorter_1<TElement>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+EnumerableSorter_2")]
impl<
    TElement: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
> std::ops::DerefMut for crate::System::Linq::EnumerableSorter_2<TElement, TKey> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+EnumerableSorter_2")]
impl<
    TElement: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
> crate::System::Linq::EnumerableSorter_2<TElement, TKey> {
    pub fn CompareKeys(
        &mut self,
        index1: i32,
        index2: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32), i32, 2usize>("CompareKeys")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompareKeys", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (index1, index2))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeKeys(
        &mut self,
        elements: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<TElement>,
        >,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<TElement>,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ComputeKeys")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ComputeKeys", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (elements, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        keySelector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TElement, TKey>>,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IComparer_1<TKey>,
        >,
        descending: bool,
        next: quest_hook::libil2cpp::Gc<
            crate::System::Linq::EnumerableSorter_1<TElement>,
        >,
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
            .invoke_void(".ctor", (keySelector, comparer, descending, next))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        keySelector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TElement, TKey>>,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IComparer_1<TKey>,
        >,
        descending: bool,
        next: quest_hook::libil2cpp::Gc<
            crate::System::Linq::EnumerableSorter_1<TElement>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Func_2<TElement, TKey>>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IComparer_1<TKey>,
                    >,
                    bool,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::EnumerableSorter_1<TElement>,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (keySelector, comparer, descending, next))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+EnumerableSorter_2")]
impl<
    TElement: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::System::Linq::EnumerableSorter_2<TElement, TKey> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
