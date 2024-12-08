#[cfg(feature = "UnityEngine+Pool+CollectionPool_2")]
#[repr(C)]
#[derive(Debug)]
pub struct CollectionPool_2<
    TCollection: quest_hook::libil2cpp::Type,
    TItem: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::Object,
    __cordl_phantom_TCollection: std::marker::PhantomData<TCollection>,
    __cordl_phantom_TItem: std::marker::PhantomData<TItem>,
}
#[cfg(feature = "UnityEngine+Pool+CollectionPool_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Pool::CollectionPool_2 <
    TCollection, TItem > => "UnityEngine.Pool"."CollectionPool`2" < TCollection, TItem >
);
#[cfg(feature = "UnityEngine+Pool+CollectionPool_2")]
impl<
    TCollection: quest_hook::libil2cpp::Type,
    TItem: quest_hook::libil2cpp::Type,
> std::ops::Deref for crate::UnityEngine::Pool::CollectionPool_2<TCollection, TItem> {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Pool+CollectionPool_2")]
impl<
    TCollection: quest_hook::libil2cpp::Type,
    TItem: quest_hook::libil2cpp::Type,
> std::ops::DerefMut for crate::UnityEngine::Pool::CollectionPool_2<TCollection, TItem> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Pool+CollectionPool_2")]
impl<
    TCollection: quest_hook::libil2cpp::Type,
    TItem: quest_hook::libil2cpp::Type,
> crate::UnityEngine::Pool::CollectionPool_2<TCollection, TItem> {
    #[cfg(feature = "UnityEngine+Pool+CollectionPool_2+__c")]
    pub type __c = crate::UnityEngine::Pool::CollectionPool_2___c<TCollection, TItem>;
}
#[cfg(feature = "UnityEngine+Pool+CollectionPool_2")]
impl<
    TCollection: quest_hook::libil2cpp::Type,
    TItem: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Pool::CollectionPool_2<TCollection, TItem> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
