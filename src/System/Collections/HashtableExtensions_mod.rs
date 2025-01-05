#[cfg(feature = "System+Collections+HashtableExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct HashtableExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Collections+HashtableExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Collections::HashtableExtensions =>
    "System.Collections"."HashtableExtensions"
);
#[cfg(feature = "System+Collections+HashtableExtensions")]
impl std::ops::Deref for crate::System::Collections::HashtableExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+HashtableExtensions")]
impl std::ops::DerefMut for crate::System::Collections::HashtableExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+HashtableExtensions")]
impl crate::System::Collections::HashtableExtensions {
    pub fn TryGetValue<T>(
        table: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetValue", (table, key, value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Collections+HashtableExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Collections::HashtableExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
