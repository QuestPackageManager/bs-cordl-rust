#[cfg(feature = "BinaryReadWriteExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct BinaryReadWriteExtensions {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "BinaryReadWriteExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BinaryReadWriteExtensions => ""
    ."BinaryReadWriteExtensions"
);
#[cfg(feature = "BinaryReadWriteExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::BinaryReadWriteExtensions {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BinaryReadWriteExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::BinaryReadWriteExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BinaryReadWriteExtensions")]
impl crate::GlobalNamespace::BinaryReadWriteExtensions {
    pub fn ReadColor(
        binaryReader: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryReader>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadColor", (binaryReader))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadListOf<T>(
        binaryReader: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryReader>,
        elementReader: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::IO::BinaryReader>,
            T,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadListOf", (binaryReader, elementReader))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadPose(
        binaryReader: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryReader>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Pose> {
        let __cordl_ret: crate::UnityEngine::Pose = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadPose", (binaryReader))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadQuaternion(
        binaryReader: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryReader>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadQuaternion", (binaryReader))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadVector3(
        binaryReader: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryReader>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadVector3", (binaryReader))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteListOf<T>(
        binaryWriter: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryWriter>,
        list: quest_hook::libil2cpp::Gc<T>,
        elementWriter: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::IO::BinaryWriter>,
            T,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteListOf", (binaryWriter, list, elementWriter))?;
        Ok(__cordl_ret.into())
    }
    pub fn Write_Color0(
        binaryWriter: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryWriter>,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Write", (binaryWriter, color))?;
        Ok(__cordl_ret.into())
    }
    pub fn Write_Pose3(
        binaryWriter: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryWriter>,
        pose: crate::UnityEngine::Pose,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Write", (binaryWriter, pose))?;
        Ok(__cordl_ret.into())
    }
    pub fn Write_Quaternion2(
        binaryWriter: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryWriter>,
        quaternion: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Write", (binaryWriter, quaternion))?;
        Ok(__cordl_ret.into())
    }
    pub fn Write_Vector3_1(
        binaryWriter: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryWriter>,
        vector: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Write", (binaryWriter, vector))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BinaryReadWriteExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BinaryReadWriteExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
