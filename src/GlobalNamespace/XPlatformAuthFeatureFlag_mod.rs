#[cfg(feature = "XPlatformAuthFeatureFlag")]
#[repr(C)]
#[derive(Debug)]
pub struct XPlatformAuthFeatureFlag {
    __cordl_parent: crate::System::Object,
    pub _UseXPlatformAuth_k__BackingField: bool,
}
#[cfg(feature = "XPlatformAuthFeatureFlag")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for XPlatformAuthFeatureFlag => ""
    ."XPlatformAuthFeatureFlag"
);
#[cfg(feature = "XPlatformAuthFeatureFlag")]
impl std::ops::Deref for XPlatformAuthFeatureFlag {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "XPlatformAuthFeatureFlag")]
impl std::ops::DerefMut for XPlatformAuthFeatureFlag {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "XPlatformAuthFeatureFlag")]
impl XPlatformAuthFeatureFlag {
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
    pub fn get_UseXPlatformAuth(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_UseXPlatformAuth", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_UseXPlatformAuth(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_UseXPlatformAuth", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "XPlatformAuthFeatureFlag")]
impl quest_hook::libil2cpp::ObjectType for XPlatformAuthFeatureFlag {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
