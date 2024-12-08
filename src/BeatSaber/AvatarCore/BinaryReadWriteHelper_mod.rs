#[cfg(feature = "BeatSaber+AvatarCore+BinaryReadWriteHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct BinaryReadWriteHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BeatSaber+AvatarCore+BinaryReadWriteHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::AvatarCore::BinaryReadWriteHelper =>
    "BeatSaber.AvatarCore"."BinaryReadWriteHelper"
);
#[cfg(feature = "BeatSaber+AvatarCore+BinaryReadWriteHelper")]
impl std::ops::Deref for crate::BeatSaber::AvatarCore::BinaryReadWriteHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+BinaryReadWriteHelper")]
impl std::ops::DerefMut for crate::BeatSaber::AvatarCore::BinaryReadWriteHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+BinaryReadWriteHelper")]
impl crate::BeatSaber::AvatarCore::BinaryReadWriteHelper {}
#[cfg(feature = "BeatSaber+AvatarCore+BinaryReadWriteHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::AvatarCore::BinaryReadWriteHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
