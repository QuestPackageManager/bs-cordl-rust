#[cfg(feature = "System+Configuration+ConfigurationCollectionAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct ConfigurationCollectionAttribute {
    __cordl_parent: crate::System::Attribute,
}
#[cfg(feature = "System+Configuration+ConfigurationCollectionAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Configuration::ConfigurationCollectionAttribute => "System.Configuration"
    ."ConfigurationCollectionAttribute"
);
#[cfg(feature = "System+Configuration+ConfigurationCollectionAttribute")]
impl std::ops::Deref for crate::System::Configuration::ConfigurationCollectionAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Configuration+ConfigurationCollectionAttribute")]
impl std::ops::DerefMut
for crate::System::Configuration::ConfigurationCollectionAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Configuration+ConfigurationCollectionAttribute")]
impl crate::System::Configuration::ConfigurationCollectionAttribute {
    pub fn _ctor(
        &mut self,
        itemType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (itemType))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        itemType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (itemType))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Configuration+ConfigurationCollectionAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Configuration::ConfigurationCollectionAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
