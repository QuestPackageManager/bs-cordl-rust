#[cfg(feature = "cordl_class_Unity+IO+Archive+ArchiveFileInterface")]
#[repr(C)]
#[derive(Debug)]
pub struct ArchiveFileInterface {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Unity+IO+Archive+ArchiveFileInterface")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::IO::Archive::ArchiveFileInterface {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.IO.Archive";
    const CLASS_NAME: &'static str = "ArchiveFileInterface";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Unity+IO+Archive+ArchiveFileInterface")]
impl std::ops::Deref for crate::Unity::IO::Archive::ArchiveFileInterface {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+IO+Archive+ArchiveFileInterface")]
impl std::ops::DerefMut for crate::Unity::IO::Archive::ArchiveFileInterface {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+IO+Archive+ArchiveFileInterface")]
impl crate::Unity::IO::Archive::ArchiveFileInterface {}
#[cfg(feature = "cordl_class_Unity+IO+Archive+ArchiveFileInterface")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::IO::Archive::ArchiveFileInterface {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
