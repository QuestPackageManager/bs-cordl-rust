#[cfg(feature = "System+Net+Configuration+WebRequestModulesSection")]
#[repr(C)]
#[derive(Debug)]
pub struct WebRequestModulesSection {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::System::Configuration::ConfigurationSection,
    >,
}
#[cfg(feature = "System+Net+Configuration+WebRequestModulesSection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::Configuration::WebRequestModulesSection => "System.Net.Configuration"
    ."WebRequestModulesSection"
);
#[cfg(feature = "System+Net+Configuration+WebRequestModulesSection")]
impl std::ops::Deref for crate::System::Net::Configuration::WebRequestModulesSection {
    type Target = quest_hook::libil2cpp::Gc<
        crate::System::Configuration::ConfigurationSection,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Configuration+WebRequestModulesSection")]
impl std::ops::DerefMut for crate::System::Net::Configuration::WebRequestModulesSection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Configuration+WebRequestModulesSection")]
impl crate::System::Net::Configuration::WebRequestModulesSection {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Properties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Configuration::ConfigurationPropertyCollection,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Configuration::ConfigurationPropertyCollection,
        > = __cordl_object.invoke("get_Properties", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Configuration+WebRequestModulesSection")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Configuration::WebRequestModulesSection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
