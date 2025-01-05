#[cfg(feature = "System+Linq+EnumerableSorter_2")]
#[repr(C)]
#[derive(Debug)]
pub struct EnumerableSorter_2<
    TElement: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Gc<TElement>,
    pub keySelector: quest_hook::libil2cpp::Gc<TElement, TKey>,
    pub comparer: quest_hook::libil2cpp::Gc<TKey>,
    pub descending: bool,
    pub next: quest_hook::libil2cpp::Gc<TElement>,
    pub keys: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TKey>>,
    __cordl_phantom_TElement: std::marker::PhantomData<TElement>,
    __cordl_phantom_TKey: std::marker::PhantomData<TKey>,
}
#[cfg(feature = "System+Linq+EnumerableSorter_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::EnumerableSorter_2 < TElement,
    TKey > => "System.Linq"."EnumerableSorter`2" < TElement, TKey >
);
#[cfg(feature = "System+Linq+EnumerableSorter_2")]
impl<
    TElement: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
> std::ops::Deref for crate::System::Linq::EnumerableSorter_2<TElement, TKey> {
    type Target = quest_hook::libil2cpp::Gc<TElement>;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareKeys", (index1, index2))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ComputeKeys", (elements, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        keySelector: quest_hook::libil2cpp::Gc<TElement, TKey>,
        comparer: quest_hook::libil2cpp::Gc<TKey>,
        descending: bool,
        next: quest_hook::libil2cpp::Gc<TElement>,
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
        keySelector: quest_hook::libil2cpp::Gc<TElement, TKey>,
        comparer: quest_hook::libil2cpp::Gc<TKey>,
        descending: bool,
        next: quest_hook::libil2cpp::Gc<TElement>,
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
            .invoke(".ctor", (keySelector, comparer, descending, next))?;
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
