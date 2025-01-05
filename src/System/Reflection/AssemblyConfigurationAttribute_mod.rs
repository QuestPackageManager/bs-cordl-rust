#[cfg(feature = "System+Reflection+AssemblyConfigurationAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct AssemblyConfigurationAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _Configuration_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "System+Reflection+AssemblyConfigurationAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Reflection::AssemblyConfigurationAttribute => "System.Reflection"
    ."AssemblyConfigurationAttribute"
);
#[cfg(feature = "System+Reflection+AssemblyConfigurationAttribute")]
impl std::ops::Deref for crate::System::Reflection::AssemblyConfigurationAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+AssemblyConfigurationAttribute")]
impl std::ops::DerefMut for crate::System::Reflection::AssemblyConfigurationAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+AssemblyConfigurationAttribute")]
impl crate::System::Reflection::AssemblyConfigurationAttribute {
    pub fn New(
        configuration: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (configuration))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        configuration: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (configuration))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Reflection+AssemblyConfigurationAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Reflection::AssemblyConfigurationAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
