#[cfg(feature = "System+Net+Configuration+SettingsSectionInternal")]
#[repr(C)]
#[derive(Debug)]
pub struct SettingsSectionInternal {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub HttpListenerUnescapeRequestUrl: bool,
    pub IPProtectionLevel: crate::System::Net::Sockets::IPProtectionLevel,
}
#[cfg(feature = "System+Net+Configuration+SettingsSectionInternal")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::Configuration::SettingsSectionInternal => "System.Net.Configuration"
    ."SettingsSectionInternal"
);
#[cfg(feature = "System+Net+Configuration+SettingsSectionInternal")]
impl std::ops::Deref for crate::System::Net::Configuration::SettingsSectionInternal {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Configuration+SettingsSectionInternal")]
impl std::ops::DerefMut for crate::System::Net::Configuration::SettingsSectionInternal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Configuration+SettingsSectionInternal")]
impl crate::System::Net::Configuration::SettingsSectionInternal {
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
    pub fn get_Ipv6Enabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Ipv6Enabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Section() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Net::Configuration::SettingsSectionInternal,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::Configuration::SettingsSectionInternal,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_Section", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Configuration+SettingsSectionInternal")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Configuration::SettingsSectionInternal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
