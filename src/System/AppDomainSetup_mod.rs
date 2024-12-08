#[cfg(feature = "System+AppDomainSetup")]
#[repr(C)]
#[derive(Debug)]
pub struct AppDomainSetup {
    __cordl_parent: crate::System::Object,
    pub application_base: *mut crate::System::String,
    pub application_name: *mut crate::System::String,
    pub cache_path: *mut crate::System::String,
    pub configuration_file: *mut crate::System::String,
    pub dynamic_base: *mut crate::System::String,
    pub license_file: *mut crate::System::String,
    pub private_bin_path: *mut crate::System::String,
    pub private_bin_path_probe: *mut crate::System::String,
    pub shadow_copy_directories: *mut crate::System::String,
    pub shadow_copy_files: *mut crate::System::String,
    pub publisher_policy: bool,
    pub path_changed: bool,
    pub loader_optimization: i32,
    pub disallow_binding_redirects: bool,
    pub disallow_code_downloads: bool,
    pub _activationArguments: *mut crate::System::Object,
    pub domain_initializer: *mut crate::System::Object,
    pub application_trust: *mut crate::System::Object,
    pub domain_initializer_args: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub disallow_appbase_probe: bool,
    pub configuration_bytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub serialized_non_primitives: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub manager_assembly: *mut crate::System::String,
    pub manager_type: *mut crate::System::String,
    pub partial_visible_assemblies: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub _TargetFrameworkName_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "System+AppDomainSetup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::AppDomainSetup => "System"
    ."AppDomainSetup"
);
#[cfg(feature = "System+AppDomainSetup")]
impl std::ops::Deref for crate::System::AppDomainSetup {
    type Target = crate::System::Object;
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
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
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
