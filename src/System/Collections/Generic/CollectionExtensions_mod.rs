#[cfg(feature = "System+Collections+Generic+CollectionExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct CollectionExtensions {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Collections+Generic+CollectionExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Collections::Generic::CollectionExtensions => "System.Collections.Generic"
    ."CollectionExtensions"
);
#[cfg(feature = "System+Collections+Generic+CollectionExtensions")]
impl std::ops::Deref for crate::System::Collections::Generic::CollectionExtensions {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Generic+CollectionExtensions")]
impl std::ops::DerefMut for crate::System::Collections::Generic::CollectionExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Generic+CollectionExtensions")]
impl crate::System::Collections::Generic::CollectionExtensions {
    pub fn GetValueOrDefault_Gc_TKey0<TKey, TValue>(
        dictionary: quest_hook::libil2cpp::Gc<TKey, TValue>,
        key: TKey,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TValue = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetValueOrDefault", (dictionary, key))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValueOrDefault_TValue1<TKey, TValue>(
        dictionary: quest_hook::libil2cpp::Gc<TKey, TValue>,
        key: TKey,
        defaultValue: TValue,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TValue = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetValueOrDefault", (dictionary, key, defaultValue))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Collections+Generic+CollectionExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Collections::Generic::CollectionExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
