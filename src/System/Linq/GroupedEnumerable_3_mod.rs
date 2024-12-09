#[cfg(feature = "System+Linq+GroupedEnumerable_3")]
#[repr(C)]
#[derive(Debug)]
pub struct GroupedEnumerable_3<
    TSource: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::Object,
    pub source: *mut crate::System::Collections::Generic::IEnumerable_1<TSource>,
    pub keySelector: *mut crate::System::Func_2<TSource, TKey>,
    pub elementSelector: *mut crate::System::Func_2<TSource, TElement>,
    pub comparer: *mut crate::System::Collections::Generic::IEqualityComparer_1<TKey>,
    __cordl_phantom_TSource: std::marker::PhantomData<TSource>,
    __cordl_phantom_TKey: std::marker::PhantomData<TKey>,
    __cordl_phantom_TElement: std::marker::PhantomData<TElement>,
}
#[cfg(feature = "System+Linq+GroupedEnumerable_3")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::GroupedEnumerable_3 < TSource,
    TKey, TElement > => "System.Linq"."GroupedEnumerable`3" < TSource, TKey, TElement >
);
#[cfg(feature = "System+Linq+GroupedEnumerable_3")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> std::ops::Deref for crate::System::Linq::GroupedEnumerable_3<TSource, TKey, TElement> {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+GroupedEnumerable_3")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::System::Linq::GroupedEnumerable_3<TSource, TKey, TElement> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+GroupedEnumerable_3")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> crate::System::Linq::GroupedEnumerable_3<TSource, TKey, TElement> {
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerator_1<
            *mut crate::System::Linq::IGrouping_2<TKey, TElement>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerator_1<
            *mut crate::System::Linq::IGrouping_2<TKey, TElement>,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        source: *mut crate::System::Collections::Generic::IEnumerable_1<TSource>,
        keySelector: *mut crate::System::Func_2<TSource, TKey>,
        elementSelector: *mut crate::System::Func_2<TSource, TElement>,
        comparer: *mut crate::System::Collections::Generic::IEqualityComparer_1<TKey>,
    ) -> quest_hook::libil2cpp::Result<*mut Self>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (source, keySelector, elementSelector, comparer))?;
        Ok(__cordl_object)
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("System.Collections.IEnumerable.GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        source: *mut crate::System::Collections::Generic::IEnumerable_1<TSource>,
        keySelector: *mut crate::System::Func_2<TSource, TKey>,
        elementSelector: *mut crate::System::Func_2<TSource, TElement>,
        comparer: *mut crate::System::Collections::Generic::IEqualityComparer_1<TKey>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (source, keySelector, elementSelector, comparer))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Linq+GroupedEnumerable_3")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::System::Linq::GroupedEnumerable_3<TSource, TKey, TElement> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
