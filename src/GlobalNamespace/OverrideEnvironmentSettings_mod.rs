#[cfg(feature = "OverrideEnvironmentSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct OverrideEnvironmentSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub overrideEnvironments: bool,
    pub _data: *mut crate::System::Collections::Generic::Dictionary_2<
        crate::GlobalNamespace::EnvironmentType,
        *mut crate::GlobalNamespace::EnvironmentInfoSO,
    >,
}
#[cfg(feature = "OverrideEnvironmentSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OverrideEnvironmentSettings =>
    ""."OverrideEnvironmentSettings"
);
#[cfg(feature = "OverrideEnvironmentSettings")]
impl std::ops::Deref for crate::GlobalNamespace::OverrideEnvironmentSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OverrideEnvironmentSettings")]
impl std::ops::DerefMut for crate::GlobalNamespace::OverrideEnvironmentSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OverrideEnvironmentSettings")]
impl crate::GlobalNamespace::OverrideEnvironmentSettings {
    pub fn GetOverrideEnvironmentInfoForType(
        &mut self,
        environmentType: crate::GlobalNamespace::EnvironmentType,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::EnvironmentInfoSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::EnvironmentInfoSO = __cordl_object
            .invoke("GetOverrideEnvironmentInfoForType", (environmentType))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetEnvironmentInfoForType(
        &mut self,
        environmentType: crate::GlobalNamespace::EnvironmentType,
        environmentInfo: *mut crate::GlobalNamespace::EnvironmentInfoSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetEnvironmentInfoForType", (environmentType, environmentInfo))?;
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
}
#[cfg(feature = "OverrideEnvironmentSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OverrideEnvironmentSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
