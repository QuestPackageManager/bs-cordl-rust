#[cfg(feature = "HealthWarningDisplayViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct HealthWarningDisplayViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _healthAndSafetyTextMesh: *mut crate::TMPro::TextMeshProUGUI,
    pub _healthAndSafetyFullLocalizationKey: *mut crate::System::String,
    pub _healthAndSafetyPSLocalizationKey: *mut crate::System::String,
    pub _showShortHealthAndSafety: bool,
}
#[cfg(feature = "HealthWarningDisplayViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for HealthWarningDisplayViewController => ""
    ."HealthWarningDisplayViewController"
);
#[cfg(feature = "HealthWarningDisplayViewController")]
impl std::ops::Deref for HealthWarningDisplayViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HealthWarningDisplayViewController")]
impl std::ops::DerefMut for HealthWarningDisplayViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HealthWarningDisplayViewController")]
impl HealthWarningDisplayViewController {
    pub fn DidActivate(
        &mut self,
        firstActivation: bool,
        addedToHierarchy: bool,
        screenSystemEnabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DidActivate",
                (firstActivation, addedToHierarchy, screenSystemEnabling),
            )?;
        Ok(__cordl_ret)
    }
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
#[cfg(feature = "HealthWarningDisplayViewController")]
impl quest_hook::libil2cpp::ObjectType for HealthWarningDisplayViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
