#[cfg(feature = "System+Diagnostics+DiagnosticsConfigurationHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct DiagnosticsConfigurationHandler {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Diagnostics+DiagnosticsConfigurationHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Diagnostics::DiagnosticsConfigurationHandler => "System.Diagnostics"
    ."DiagnosticsConfigurationHandler"
);
#[cfg(feature = "System+Diagnostics+DiagnosticsConfigurationHandler")]
impl std::ops::Deref for crate::System::Diagnostics::DiagnosticsConfigurationHandler {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+DiagnosticsConfigurationHandler")]
impl std::ops::DerefMut for crate::System::Diagnostics::DiagnosticsConfigurationHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+DiagnosticsConfigurationHandler")]
impl crate::System::Diagnostics::DiagnosticsConfigurationHandler {
    pub fn Create(
        &mut self,
        parent: *mut crate::System::Object,
        configContext: *mut crate::System::Object,
        section: *mut crate::System::Xml::XmlNode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Create", (parent, configContext, section))?;
        Ok(__cordl_ret)
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Diagnostics+DiagnosticsConfigurationHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Diagnostics::DiagnosticsConfigurationHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
