#[cfg(feature = "System+Collections+Concurrent+IDictionaryDebugView_2")]
#[repr(C)]
#[derive(Debug)]
pub struct IDictionaryDebugView_2<
    K: quest_hook::libil2cpp::Type,
    V: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_K: std::marker::PhantomData<K>,
    __cordl_phantom_V: std::marker::PhantomData<V>,
}
#[cfg(feature = "System+Collections+Concurrent+IDictionaryDebugView_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Collections::Concurrent::IDictionaryDebugView_2 < K, V > =>
    "System.Collections.Concurrent"."IDictionaryDebugView`2" < K, V >
);
#[cfg(feature = "System+Collections+Concurrent+IDictionaryDebugView_2")]
impl<K: quest_hook::libil2cpp::Type, V: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Collections::Concurrent::IDictionaryDebugView_2<K, V> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Concurrent+IDictionaryDebugView_2")]
impl<K: quest_hook::libil2cpp::Type, V: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Collections::Concurrent::IDictionaryDebugView_2<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Concurrent+IDictionaryDebugView_2")]
impl<
    K: quest_hook::libil2cpp::Type,
    V: quest_hook::libil2cpp::Type,
> crate::System::Collections::Concurrent::IDictionaryDebugView_2<K, V> {}
#[cfg(feature = "System+Collections+Concurrent+IDictionaryDebugView_2")]
impl<
    K: quest_hook::libil2cpp::Type,
    V: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::System::Collections::Concurrent::IDictionaryDebugView_2<K, V> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
