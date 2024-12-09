#[cfg(feature = "BGLib+SaveDataCore+SaveDataResultExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct SaveDataResultExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BGLib+SaveDataCore+SaveDataResultExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::SaveDataCore::SaveDataResultExtensions =>
    "BGLib.SaveDataCore"."SaveDataResultExtensions"
);
#[cfg(feature = "BGLib+SaveDataCore+SaveDataResultExtensions")]
impl std::ops::Deref for crate::BGLib::SaveDataCore::SaveDataResultExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+SaveDataCore+SaveDataResultExtensions")]
impl std::ops::DerefMut for crate::BGLib::SaveDataCore::SaveDataResultExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+SaveDataCore+SaveDataResultExtensions")]
impl crate::BGLib::SaveDataCore::SaveDataResultExtensions {}
#[cfg(feature = "BGLib+SaveDataCore+SaveDataResultExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::SaveDataCore::SaveDataResultExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
