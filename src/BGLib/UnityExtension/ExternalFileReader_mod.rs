#[cfg(feature = "BGLib+UnityExtension+ExternalFileReader")]
#[repr(C)]
#[derive(Debug)]
pub struct ExternalFileReader {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "BGLib+UnityExtension+ExternalFileReader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::UnityExtension::ExternalFileReader =>
    "BGLib.UnityExtension"."ExternalFileReader"
);
#[cfg(feature = "BGLib+UnityExtension+ExternalFileReader")]
impl std::ops::Deref for crate::BGLib::UnityExtension::ExternalFileReader {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+ExternalFileReader")]
impl std::ops::DerefMut for crate::BGLib::UnityExtension::ExternalFileReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+ExternalFileReader")]
impl crate::BGLib::UnityExtension::ExternalFileReader {
    pub fn ExistsAsync(
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<bool>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<bool> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExistsAsync", (filePath))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+UnityExtension+ExternalFileReader")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::UnityExtension::ExternalFileReader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
