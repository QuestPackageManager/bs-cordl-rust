#[cfg(feature = "Unity+IO+Archive+ArchiveFileInterface")]
#[repr(C)]
#[derive(Debug)]
pub struct ArchiveFileInterface {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+IO+Archive+ArchiveFileInterface")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::IO::Archive::ArchiveFileInterface =>
    "Unity.IO.Archive"."ArchiveFileInterface"
);
#[cfg(feature = "Unity+IO+Archive+ArchiveFileInterface")]
impl std::ops::Deref for crate::Unity::IO::Archive::ArchiveFileInterface {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+IO+Archive+ArchiveFileInterface")]
impl std::ops::DerefMut for crate::Unity::IO::Archive::ArchiveFileInterface {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+IO+Archive+ArchiveFileInterface")]
impl crate::Unity::IO::Archive::ArchiveFileInterface {}
#[cfg(feature = "Unity+IO+Archive+ArchiveFileInterface")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::IO::Archive::ArchiveFileInterface {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
