#[cfg(feature = "Unity+Profiling+Memory+MemorySnapshotMetadata")]
#[repr(C)]
#[derive(Debug)]
pub struct MemorySnapshotMetadata {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _Description_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
    pub _Data_k__BackingField: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Unity+Profiling+Memory+MemorySnapshotMetadata")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Profiling::Memory::MemorySnapshotMetadata
    => "Unity.Profiling.Memory"."MemorySnapshotMetadata"
);
#[cfg(feature = "Unity+Profiling+Memory+MemorySnapshotMetadata")]
impl std::ops::Deref for crate::Unity::Profiling::Memory::MemorySnapshotMetadata {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Profiling+Memory+MemorySnapshotMetadata")]
impl std::ops::DerefMut for crate::Unity::Profiling::Memory::MemorySnapshotMetadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Profiling+Memory+MemorySnapshotMetadata")]
impl crate::Unity::Profiling::Memory::MemorySnapshotMetadata {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Data(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("get_Data", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Description(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_Description", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Description(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Description", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Unity+Profiling+Memory+MemorySnapshotMetadata")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Profiling::Memory::MemorySnapshotMetadata {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
