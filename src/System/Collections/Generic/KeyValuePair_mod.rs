#[cfg(feature = "System+Collections+Generic+KeyValuePair")]
#[repr(C)]
#[derive(Debug)]
pub struct KeyValuePair {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Collections+Generic+KeyValuePair")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Collections::Generic::KeyValuePair =>
    "System.Collections.Generic"."KeyValuePair"
);
#[cfg(feature = "System+Collections+Generic+KeyValuePair")]
impl std::ops::Deref for crate::System::Collections::Generic::KeyValuePair {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Generic+KeyValuePair")]
impl std::ops::DerefMut for crate::System::Collections::Generic::KeyValuePair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Generic+KeyValuePair")]
impl crate::System::Collections::Generic::KeyValuePair {
    pub fn PairToString(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PairToString", (key, value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Collections+Generic+KeyValuePair")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Collections::Generic::KeyValuePair {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
