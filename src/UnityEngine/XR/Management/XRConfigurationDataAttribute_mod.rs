#[cfg(feature = "UnityEngine+XR+Management+XRConfigurationDataAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct XRConfigurationDataAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _displayName_k__BackingField: *mut crate::System::String,
    pub _buildSettingsKey_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "UnityEngine+XR+Management+XRConfigurationDataAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::XR::Management::XRConfigurationDataAttribute =>
    "UnityEngine.XR.Management"."XRConfigurationDataAttribute"
);
#[cfg(feature = "UnityEngine+XR+Management+XRConfigurationDataAttribute")]
impl std::ops::Deref
for crate::UnityEngine::XR::Management::XRConfigurationDataAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+Management+XRConfigurationDataAttribute")]
impl std::ops::DerefMut
for crate::UnityEngine::XR::Management::XRConfigurationDataAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+Management+XRConfigurationDataAttribute")]
impl crate::UnityEngine::XR::Management::XRConfigurationDataAttribute {
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_String_String1(
        displayName: *mut crate::System::String,
        buildSettingsKey: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (displayName, buildSettingsKey))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_String1(
        &mut self,
        displayName: *mut crate::System::String,
        buildSettingsKey: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (displayName, buildSettingsKey))?;
        Ok(__cordl_ret)
    }
    pub fn get_buildSettingsKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_buildSettingsKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_displayName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_displayName", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_buildSettingsKey(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_buildSettingsKey", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_displayName(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_displayName", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+XR+Management+XRConfigurationDataAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::XR::Management::XRConfigurationDataAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
