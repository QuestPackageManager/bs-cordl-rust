#[cfg(feature = "System+Net+Configuration+SettingsSectionInternal")]
#[repr(C)]
#[derive(Debug)]
pub struct SettingsSectionInternal {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub HttpListenerUnescapeRequestUrl: bool,
    pub IPProtectionLevel: crate::System::Net::Sockets::IPProtectionLevel,
}
#[cfg(feature = "System+Net+Configuration+SettingsSectionInternal")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::Configuration::SettingsSectionInternal {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net.Configuration";
    const CLASS_NAME: &'static str = "SettingsSectionInternal";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::Configuration::SettingsSectionInternal as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::Configuration::SettingsSectionInternal as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Ipv6Enabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::Configuration::SettingsSectionInternal as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_Ipv6Enabled")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::Configuration::SettingsSectionInternal as
                    quest_hook::libil2cpp::Type > ::class(), "get_Ipv6Enabled", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Section() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Net::Configuration::SettingsSectionInternal,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::Configuration::SettingsSectionInternal as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Net::Configuration::SettingsSectionInternal,
                >,
                0usize,
            >("get_Section")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::Configuration::SettingsSectionInternal as
                    quest_hook::libil2cpp::Type > ::class(), "get_Section", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::Configuration::SettingsSectionInternal,
        > = unsafe { method.invoke_unchecked((), ())? };
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
