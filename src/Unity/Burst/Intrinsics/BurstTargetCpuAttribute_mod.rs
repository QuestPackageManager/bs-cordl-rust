#[cfg(feature = "Unity+Burst+Intrinsics+BurstTargetCpuAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstTargetCpuAttribute {
    __cordl_parent: crate::System::Attribute,
    pub TargetCpu: crate::Unity::Burst::BurstTargetCpu,
}
#[cfg(feature = "Unity+Burst+Intrinsics+BurstTargetCpuAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Burst::Intrinsics::BurstTargetCpuAttribute => "Unity.Burst.Intrinsics"
    ."BurstTargetCpuAttribute"
);
#[cfg(feature = "Unity+Burst+Intrinsics+BurstTargetCpuAttribute")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::BurstTargetCpuAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+BurstTargetCpuAttribute")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::BurstTargetCpuAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+BurstTargetCpuAttribute")]
impl crate::Unity::Burst::Intrinsics::BurstTargetCpuAttribute {
    pub fn New(
        TargetCpu: crate::Unity::Burst::BurstTargetCpu,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (TargetCpu))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        TargetCpu: crate::Unity::Burst::BurstTargetCpu,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (TargetCpu))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+BurstTargetCpuAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Burst::Intrinsics::BurstTargetCpuAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
