#[cfg(feature = "OverrideEnvironmentSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct OverrideEnvironmentSettings {
    __cordl_parent: crate::System::Object,
    pub overrideEnvironments: bool,
    pub _data: *mut crate::System::Collections::Generic::Dictionary_2<
        EnvironmentType,
        *mut EnvironmentInfoSO,
    >,
}
#[cfg(feature = "OverrideEnvironmentSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OverrideEnvironmentSettings => ""
    ."OverrideEnvironmentSettings"
);
#[cfg(feature = "OverrideEnvironmentSettings")]
impl std::ops::Deref for OverrideEnvironmentSettings {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OverrideEnvironmentSettings")]
impl std::ops::DerefMut for OverrideEnvironmentSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OverrideEnvironmentSettings")]
impl OverrideEnvironmentSettings {
    pub fn SetEnvironmentInfoForType(
        &mut self,
        environmentType: EnvironmentType,
        environmentInfo: *mut EnvironmentInfoSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetEnvironmentInfoForType", (environmentType, environmentInfo))?;
        Ok(__cordl_ret)
    }
    pub fn GetOverrideEnvironmentInfoForType(
        &mut self,
        environmentType: EnvironmentType,
    ) -> quest_hook::libil2cpp::Result<*mut EnvironmentInfoSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut EnvironmentInfoSO = __cordl_object
            .invoke("GetOverrideEnvironmentInfoForType", (environmentType))?;
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
#[cfg(feature = "OverrideEnvironmentSettings")]
impl quest_hook::libil2cpp::ObjectType for OverrideEnvironmentSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
