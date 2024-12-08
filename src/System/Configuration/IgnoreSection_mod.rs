#[cfg(feature = "System+Configuration+IgnoreSection")]
#[repr(C)]
#[derive(Debug)]
pub struct IgnoreSection {
    __cordl_parent: crate::System::Configuration::ConfigurationSection,
}
#[cfg(feature = "System+Configuration+IgnoreSection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Configuration::IgnoreSection =>
    "System.Configuration"."IgnoreSection"
);
#[cfg(feature = "System+Configuration+IgnoreSection")]
impl std::ops::Deref for crate::System::Configuration::IgnoreSection {
    type Target = crate::System::Configuration::ConfigurationSection;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Configuration+IgnoreSection")]
impl std::ops::DerefMut for crate::System::Configuration::IgnoreSection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Configuration+IgnoreSection")]
impl crate::System::Configuration::IgnoreSection {
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
    pub fn DeserializeSection(
        &mut self,
        xmlReader: *mut crate::System::Xml::XmlReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeserializeSection", (xmlReader))?;
        Ok(__cordl_ret)
    }
    pub fn get_Properties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Configuration::ConfigurationPropertyCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Configuration::ConfigurationPropertyCollection = __cordl_object
            .invoke("get_Properties", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsModified(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsModified", ())?;
        Ok(__cordl_ret)
    }
    pub fn Reset(
        &mut self,
        parentSection: *mut crate::System::Configuration::ConfigurationElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", (parentSection))?;
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
        parentSection: *mut crate::System::Configuration::ConfigurationElement,
        name: *mut crate::System::String,
        saveMode: crate::System::Configuration::ConfigurationSaveMode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("SerializeSection", (parentSection, name, saveMode))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Configuration+IgnoreSection")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Configuration::IgnoreSection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
