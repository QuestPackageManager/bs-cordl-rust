#[cfg(feature = "System+AppDomainSetup")]
#[repr(C)]
#[derive(Debug)]
pub struct AppDomainSetup {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub application_base: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub application_name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub cache_path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub configuration_file: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub dynamic_base: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub license_file: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub private_bin_path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub private_bin_path_probe: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub shadow_copy_directories: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub shadow_copy_files: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub publisher_policy: bool,
    pub path_changed: bool,
    pub loader_optimization: i32,
    pub disallow_binding_redirects: bool,
    pub disallow_code_downloads: bool,
    pub _activationArguments: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppObject,
    >,
    pub domain_initializer: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppObject,
    >,
    pub application_trust: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppObject,
    >,
    pub domain_initializer_args: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub disallow_appbase_probe: bool,
    pub configuration_bytes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
    pub serialized_non_primitives: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
    pub manager_assembly: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub manager_type: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub partial_visible_assemblies: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub _TargetFrameworkName_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "System+AppDomainSetup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::AppDomainSetup => "System"
    ."AppDomainSetup"
);
#[cfg(feature = "System+AppDomainSetup")]
impl std::ops::Deref for crate::System::AppDomainSetup {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+AppDomainSetup")]
impl std::ops::DerefMut for crate::System::AppDomainSetup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+AppDomainSetup")]
impl crate::System::AppDomainSetup {
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
}
#[cfg(feature = "System+AppDomainSetup")]
impl quest_hook::libil2cpp::ObjectType for crate::System::AppDomainSetup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
