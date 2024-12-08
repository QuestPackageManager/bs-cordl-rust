#[cfg(feature = "System+Resources+SatelliteContractVersionAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct SatelliteContractVersionAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _Version_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "System+Resources+SatelliteContractVersionAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Resources::SatelliteContractVersionAttribute => "System.Resources"
    ."SatelliteContractVersionAttribute"
);
#[cfg(feature = "System+Resources+SatelliteContractVersionAttribute")]
impl std::ops::Deref for crate::System::Resources::SatelliteContractVersionAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Resources+SatelliteContractVersionAttribute")]
impl std::ops::DerefMut for crate::System::Resources::SatelliteContractVersionAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Resources+SatelliteContractVersionAttribute")]
impl crate::System::Resources::SatelliteContractVersionAttribute {
    pub fn _ctor(
        &mut self,
        version: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (version))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        version: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (version))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Resources+SatelliteContractVersionAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Resources::SatelliteContractVersionAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
