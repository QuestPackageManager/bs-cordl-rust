#[cfg(feature = "System+Configuration+ConfigurationSection")]
#[repr(C)]
#[derive(Debug)]
pub struct ConfigurationSection {
    __cordl_parent: crate::System::Configuration::ConfigurationElement,
}
#[cfg(feature = "System+Configuration+ConfigurationSection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Configuration::ConfigurationSection =>
    "System.Configuration"."ConfigurationSection"
);
#[cfg(feature = "System+Configuration+ConfigurationSection")]
impl std::ops::Deref for crate::System::Configuration::ConfigurationSection {
    type Target = crate::System::Configuration::ConfigurationElement;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Configuration+ConfigurationSection")]
impl std::ops::DerefMut for crate::System::Configuration::ConfigurationSection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Configuration+ConfigurationSection")]
impl crate::System::Configuration::ConfigurationSection {
    pub fn DeserializeSection(
        &mut self,
        reader: *mut crate::System::Xml::XmlReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeserializeSection", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn IsModified(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsModified", ())?;
        Ok(__cordl_ret)
    }
    pub fn ResetModified(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetModified", ())?;
        Ok(__cordl_ret)
    }
    pub fn SerializeSection(
        &mut self,
        parentElement: *mut crate::System::Configuration::ConfigurationElement,
        name: *mut crate::System::String,
        saveMode: crate::System::Configuration::ConfigurationSaveMode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("SerializeSection", (parentElement, name, saveMode))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Configuration+ConfigurationSection")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Configuration::ConfigurationSection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
