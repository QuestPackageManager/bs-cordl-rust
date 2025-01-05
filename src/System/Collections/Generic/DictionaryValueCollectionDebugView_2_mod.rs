#[cfg(feature = "System+Collections+Generic+DictionaryValueCollectionDebugView_2")]
#[repr(C)]
#[derive(Debug)]
pub struct DictionaryValueCollectionDebugView_2<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_TKey: std::marker::PhantomData<TKey>,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "System+Collections+Generic+DictionaryValueCollectionDebugView_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Collections::Generic::DictionaryValueCollectionDebugView_2 < TKey, TValue >
    => "System.Collections.Generic"."DictionaryValueCollectionDebugView`2" < TKey, TValue
    >
);
#[cfg(feature = "System+Collections+Generic+DictionaryValueCollectionDebugView_2")]
impl<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::System::Collections::Generic::DictionaryValueCollectionDebugView_2<
    TKey,
    TValue,
> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Generic+DictionaryValueCollectionDebugView_2")]
impl<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::System::Collections::Generic::DictionaryValueCollectionDebugView_2<
    TKey,
    TValue,
> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Generic+DictionaryValueCollectionDebugView_2")]
impl<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> crate::System::Collections::Generic::DictionaryValueCollectionDebugView_2<
    TKey,
    TValue,
> {}
#[cfg(feature = "System+Collections+Generic+DictionaryValueCollectionDebugView_2")]
impl<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::System::Collections::Generic::DictionaryValueCollectionDebugView_2<
    TKey,
    TValue,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
