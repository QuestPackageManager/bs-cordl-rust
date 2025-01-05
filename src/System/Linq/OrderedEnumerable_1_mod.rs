#[cfg(feature = "System+Linq+OrderedEnumerable_1")]
#[repr(C)]
#[derive(Debug)]
pub struct OrderedEnumerable_1<TElement: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub source: quest_hook::libil2cpp::Gc<TElement>,
    __cordl_phantom_TElement: std::marker::PhantomData<TElement>,
}
#[cfg(feature = "System+Linq+OrderedEnumerable_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::OrderedEnumerable_1 < TElement >
    => "System.Linq"."OrderedEnumerable`1" < TElement >
);
#[cfg(feature = "System+Linq+OrderedEnumerable_1")]
impl<TElement: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Linq::OrderedEnumerable_1<TElement> {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+OrderedEnumerable_1")]
impl<TElement: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Linq::OrderedEnumerable_1<TElement> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+OrderedEnumerable_1")]
impl<
    TElement: quest_hook::libil2cpp::Type,
> crate::System::Linq::OrderedEnumerable_1<TElement> {
    pub fn GetEnumerableSorter(
        &mut self,
        next: quest_hook::libil2cpp::Gc<TElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TElement>>
    where
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<TElement> = __cordl_object
            .invoke("GetEnumerableSorter", (next))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TElement>>
    where
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<TElement> = __cordl_object
            .invoke("GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    >
    where
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("System.Collections.IEnumerable.GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Linq_IOrderedEnumerable_TElement__CreateOrderedEnumerable<TKey>(
        &mut self,
        keySelector: quest_hook::libil2cpp::Gc<TElement, TKey>,
        comparer: quest_hook::libil2cpp::Gc<TKey>,
        descending: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TElement>>
    where
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<TElement> = __cordl_object
            .invoke(
                "System.Linq.IOrderedEnumerable<TElement>.CreateOrderedEnumerable",
                (keySelector, comparer, descending),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+OrderedEnumerable_1")]
impl<TElement: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Linq::OrderedEnumerable_1<TElement> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+OrderedEnumerable_1")]
impl<TElement: quest_hook::libil2cpp::Type> AsRef<quest_hook::libil2cpp::Gc<TElement>>
for crate::System::Linq::OrderedEnumerable_1<TElement> {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<TElement> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+OrderedEnumerable_1")]
impl<TElement: quest_hook::libil2cpp::Type> AsMut<quest_hook::libil2cpp::Gc<TElement>>
for crate::System::Linq::OrderedEnumerable_1<TElement> {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<TElement> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+OrderedEnumerable_1")]
impl<TElement: quest_hook::libil2cpp::Type> AsRef<quest_hook::libil2cpp::Gc<TElement>>
for crate::System::Linq::OrderedEnumerable_1<TElement> {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<TElement> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+OrderedEnumerable_1")]
impl<TElement: quest_hook::libil2cpp::Type> AsMut<quest_hook::libil2cpp::Gc<TElement>>
for crate::System::Linq::OrderedEnumerable_1<TElement> {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<TElement> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+OrderedEnumerable_1")]
impl<
    TElement: quest_hook::libil2cpp::Type,
> AsRef<quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>>
for crate::System::Linq::OrderedEnumerable_1<TElement> {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+OrderedEnumerable_1")]
impl<
    TElement: quest_hook::libil2cpp::Type,
> AsMut<quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>>
for crate::System::Linq::OrderedEnumerable_1<TElement> {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable> {
        unsafe { std::mem::transmute(self) }
    }
}
