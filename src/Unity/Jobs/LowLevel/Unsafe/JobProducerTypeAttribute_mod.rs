#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobProducerTypeAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct JobProducerTypeAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _ProducerType_k__BackingField: *mut crate::System::Type,
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobProducerTypeAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Jobs::LowLevel::Unsafe::JobProducerTypeAttribute =>
    "Unity.Jobs.LowLevel.Unsafe"."JobProducerTypeAttribute"
);
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobProducerTypeAttribute")]
impl std::ops::Deref for crate::Unity::Jobs::LowLevel::Unsafe::JobProducerTypeAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobProducerTypeAttribute")]
impl std::ops::DerefMut
for crate::Unity::Jobs::LowLevel::Unsafe::JobProducerTypeAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobProducerTypeAttribute")]
impl crate::Unity::Jobs::LowLevel::Unsafe::JobProducerTypeAttribute {
    pub fn New(
        producerType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (producerType))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        producerType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (producerType))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobProducerTypeAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Jobs::LowLevel::Unsafe::JobProducerTypeAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
