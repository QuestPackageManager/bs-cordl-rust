#[cfg(feature = "System+Linq+OrderedEnumerable_2")]
#[repr(C)]
#[derive(Debug)]
pub struct OrderedEnumerable_2<
    TElement: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::Linq::OrderedEnumerable_1<TElement>,
    pub parent: *mut crate::System::Linq::OrderedEnumerable_1<TElement>,
    pub keySelector: *mut crate::System::Func_2<TElement, TKey>,
    pub comparer: *mut crate::System::Collections::Generic::IComparer_1<TKey>,
    pub descending: bool,
    __cordl_phantom_TElement: std::marker::PhantomData<TElement>,
    __cordl_phantom_TKey: std::marker::PhantomData<TKey>,
}
#[cfg(feature = "System+Linq+OrderedEnumerable_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::OrderedEnumerable_2 < TElement,
    TKey > => "System.Linq"."OrderedEnumerable`2" < TElement, TKey >
);
#[cfg(feature = "System+Linq+OrderedEnumerable_2")]
impl<
    TElement: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
> std::ops::Deref for crate::System::Linq::OrderedEnumerable_2<TElement, TKey> {
    type Target = crate::System::Linq::OrderedEnumerable_1<TElement>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+OrderedEnumerable_2")]
impl<
    TElement: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
> std::ops::DerefMut for crate::System::Linq::OrderedEnumerable_2<TElement, TKey> {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
        next: *mut crate::System::Linq::EnumerableSorter_1<TElement>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::EnumerableSorter_1<TElement>,
    >
    where
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::EnumerableSorter_1<TElement> = __cordl_object
            .invoke("GetEnumerableSorter", (next))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        source: *mut crate::System::Collections::Generic::IEnumerable_1<TElement>,
        keySelector: *mut crate::System::Func_2<TElement, TKey>,
        comparer: *mut crate::System::Collections::Generic::IComparer_1<TKey>,
        descending: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self>
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
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        source: *mut crate::System::Collections::Generic::IEnumerable_1<TElement>,
        keySelector: *mut crate::System::Func_2<TElement, TKey>,
        comparer: *mut crate::System::Collections::Generic::IComparer_1<TKey>,
        descending: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (source, keySelector, comparer, descending))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Linq+OrderedEnumerable_2")]
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
