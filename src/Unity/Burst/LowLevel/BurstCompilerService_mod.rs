#[cfg(feature = "Unity+Burst+LowLevel+BurstCompilerService")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstCompilerService {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+LowLevel+BurstCompilerService")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::LowLevel::BurstCompilerService =>
    "Unity.Burst.LowLevel"."BurstCompilerService"
);
#[cfg(feature = "Unity+Burst+LowLevel+BurstCompilerService")]
impl std::ops::Deref for crate::Unity::Burst::LowLevel::BurstCompilerService {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+LowLevel+BurstCompilerService")]
impl std::ops::DerefMut for crate::Unity::Burst::LowLevel::BurstCompilerService {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+LowLevel+BurstCompilerService")]
impl crate::Unity::Burst::LowLevel::BurstCompilerService {
    #[cfg(feature = "Unity+Burst+LowLevel+BurstCompilerService+BurstLogType")]
    pub type BurstLogType = crate::Unity::Burst::LowLevel::BurstCompilerService_BurstLogType;
}
#[cfg(feature = "Unity+Burst+LowLevel+BurstCompilerService")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Burst::LowLevel::BurstCompilerService {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+LowLevel+BurstCompilerService+BurstLogType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BurstCompilerService_BurstLogType {
    Error = 2i32,
    Info = 0i32,
    Warning = 1i32,
}
#[cfg(feature = "Unity+Burst+LowLevel+BurstCompilerService+BurstLogType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Burst::LowLevel::BurstCompilerService_BurstLogType => "Unity.Burst.LowLevel"
    ."BurstCompilerService/BurstLogType"
);
