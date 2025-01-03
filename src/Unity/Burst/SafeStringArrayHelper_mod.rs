#[cfg(feature = "Unity+Burst+SafeStringArrayHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct SafeStringArrayHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+SafeStringArrayHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::SafeStringArrayHelper =>
    "Unity.Burst"."SafeStringArrayHelper"
);
#[cfg(feature = "Unity+Burst+SafeStringArrayHelper")]
impl std::ops::Deref for crate::Unity::Burst::SafeStringArrayHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+SafeStringArrayHelper")]
impl std::ops::DerefMut for crate::Unity::Burst::SafeStringArrayHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+SafeStringArrayHelper")]
impl crate::Unity::Burst::SafeStringArrayHelper {
    pub fn DeserialiseStringArraySafe(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeserialiseStringArraySafe", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn SerialiseStringArraySafe(
        array: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SerialiseStringArraySafe", (array))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+SafeStringArrayHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::SafeStringArrayHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
