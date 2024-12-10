#[cfg(feature = "System+Linq+IOrderedEnumerable_1")]
#[repr(C)]
#[derive(Debug)]
pub struct IOrderedEnumerable_1<TElement: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_TElement: std::marker::PhantomData<TElement>,
}
#[cfg(feature = "System+Linq+IOrderedEnumerable_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::IOrderedEnumerable_1 < TElement >
    => "System.Linq"."IOrderedEnumerable`1" < TElement >
);
#[cfg(feature = "System+Linq+IOrderedEnumerable_1")]
impl<TElement: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Linq::IOrderedEnumerable_1<TElement> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+IOrderedEnumerable_1")]
impl<TElement: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Linq::IOrderedEnumerable_1<TElement> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+IOrderedEnumerable_1")]
impl<
    TElement: quest_hook::libil2cpp::Type,
> crate::System::Linq::IOrderedEnumerable_1<TElement> {
    pub fn CreateOrderedEnumerable<TKey>(
        &mut self,
        keySelector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TElement, TKey>>,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IComparer_1<TKey>,
        >,
        descending: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::IOrderedEnumerable_1<TElement>>,
    >
    where
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::IOrderedEnumerable_1<TElement>,
        > = __cordl_object
            .invoke("CreateOrderedEnumerable", (keySelector, comparer, descending))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Linq+IOrderedEnumerable_1")]
impl<TElement: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Linq::IOrderedEnumerable_1<TElement> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+IOrderedEnumerable_1")]
impl<
    TElement: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IEnumerable_1<TElement>>
for crate::System::Linq::IOrderedEnumerable_1<TElement> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IEnumerable_1<TElement> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+IOrderedEnumerable_1")]
impl<
    TElement: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IEnumerable_1<TElement>>
for crate::System::Linq::IOrderedEnumerable_1<TElement> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerable_1<TElement> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+IOrderedEnumerable_1")]
impl<
    TElement: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::IEnumerable>
for crate::System::Linq::IOrderedEnumerable_1<TElement> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+IOrderedEnumerable_1")]
impl<
    TElement: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::IEnumerable>
for crate::System::Linq::IOrderedEnumerable_1<TElement> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
