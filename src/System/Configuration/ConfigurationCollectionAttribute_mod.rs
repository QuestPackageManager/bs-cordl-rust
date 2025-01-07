#[cfg(feature = "System+Configuration+ConfigurationCollectionAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct ConfigurationCollectionAttribute {
    __cordl_parent: crate::System::Attribute,
}
#[cfg(feature = "System+Configuration+ConfigurationCollectionAttribute")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Configuration::ConfigurationCollectionAttribute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Configuration";
    const CLASS_NAME: &'static str = "ConfigurationCollectionAttribute";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
    pub fn New(
        itemType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (itemType))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        itemType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (itemType))?;
        Ok(__cordl_ret.into())
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
