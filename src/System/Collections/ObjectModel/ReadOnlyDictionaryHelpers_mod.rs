#[cfg(feature = "System+Collections+ObjectModel+ReadOnlyDictionaryHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct ReadOnlyDictionaryHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Collections+ObjectModel+ReadOnlyDictionaryHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Collections::ObjectModel::ReadOnlyDictionaryHelpers =>
    "System.Collections.ObjectModel"."ReadOnlyDictionaryHelpers"
);
#[cfg(feature = "System+Collections+ObjectModel+ReadOnlyDictionaryHelpers")]
impl std::ops::Deref
for crate::System::Collections::ObjectModel::ReadOnlyDictionaryHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+ObjectModel+ReadOnlyDictionaryHelpers")]
impl std::ops::DerefMut
for crate::System::Collections::ObjectModel::ReadOnlyDictionaryHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+ObjectModel+ReadOnlyDictionaryHelpers")]
impl crate::System::Collections::ObjectModel::ReadOnlyDictionaryHelpers {}
#[cfg(feature = "System+Collections+ObjectModel+ReadOnlyDictionaryHelpers")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Collections::ObjectModel::ReadOnlyDictionaryHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
