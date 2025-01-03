#[cfg(feature = "BeatSaber+AvatarCore+BinaryReadWriteHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct BinaryReadWriteHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatSaber+AvatarCore+BinaryReadWriteHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::AvatarCore::BinaryReadWriteHelper =>
    "BeatSaber.AvatarCore"."BinaryReadWriteHelper"
);
#[cfg(feature = "BeatSaber+AvatarCore+BinaryReadWriteHelper")]
impl std::ops::Deref for crate::BeatSaber::AvatarCore::BinaryReadWriteHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::BeatSaber::AvatarCore::BinaryReadWriteHelper {
    pub fn ReadColor(
        binaryReader: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryReader>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadColor", (binaryReader))?;
        Ok(__cordl_ret.into())
    }
    pub fn Write(
        binaryWriter: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryWriter>,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Write", (binaryWriter, color))?;
        Ok(__cordl_ret.into())
    }
}
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
