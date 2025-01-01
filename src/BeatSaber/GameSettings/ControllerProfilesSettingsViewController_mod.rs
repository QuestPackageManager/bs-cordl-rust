#[cfg(feature = "BeatSaber+GameSettings+ControllerProfilesSettingsViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct ControllerProfilesSettingsViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _profilesDropdown: *mut crate::HMUI::SimpleTextDropdown,
    pub _handlingToggle: *mut crate::HMUI::ToggleWithCallbacks,
    pub _interactableCanvasGroups: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::CanvasGroup,
    >,
    pub _transformSettingsViewControllers: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::BeatSaber::GameSettings::ControllersTransformSettings,
    >,
    pub _buttonsContainer: *mut crate::UnityEngine::GameObject,
    pub _copyFromLeftButton: *mut crate::UnityEngine::UI::Button,
    pub _copyFromRightButton: *mut crate::UnityEngine::UI::Button,
    pub _copyFromProfileButton: *mut crate::UnityEngine::UI::Button,
    pub _profileModel: *mut crate::BeatSaber::GameSettings::ControllerProfilesModel,
    pub _toggleBinder: *mut crate::HMUI::ToggleBinder,
    pub onRequestSelectProfileToCopyFrom: *mut crate::System::Action_1<
        *mut quest_hook::libil2cpp::Il2CppArray<
            crate::System::ValueTuple_2<*mut quest_hook::libil2cpp::Il2CppString, i32>,
        >,
    >,
}
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfilesSettingsViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::GameSettings::ControllerProfilesSettingsViewController =>
    "BeatSaber.GameSettings"."ControllerProfilesSettingsViewController"
);
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfilesSettingsViewController")]
impl std::ops::Deref
for crate::BeatSaber::GameSettings::ControllerProfilesSettingsViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfilesSettingsViewController")]
impl std::ops::DerefMut
for crate::BeatSaber::GameSettings::ControllerProfilesSettingsViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfilesSettingsViewController")]
impl crate::BeatSaber::GameSettings::ControllerProfilesSettingsViewController {
    pub fn CopyFormLeftToRight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyFormLeftToRight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyFromRightToLeft(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyFromRightToLeft", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyProfileResponse(
        &mut self,
        wasCanceled: bool,
        selectedDropdownIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyProfileResponse", (wasCanceled, selectedDropdownIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyValuesFromOtherProfile(
        &mut self,
        otherProfileIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyValuesFromOtherProfile", (otherProfileIndex))?;
        Ok(__cordl_ret.into())
    }
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
        Ok(__cordl_ret.into())
    }
    pub fn DidDeactivate(
        &mut self,
        removedFromHierarchy: bool,
        screenSystemDisabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DidDeactivate", (removedFromHierarchy, screenSystemDisabling))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSelectedProfile(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::GameSettings::ControllerProfile>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::GameSettings::ControllerProfile,
        > = __cordl_object.invoke("GetSelectedProfile", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandlingToggleChanged(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlingToggleChanged", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ProfileSelected(
        &mut self,
        view: quest_hook::libil2cpp::Gc<crate::HMUI::DropdownWithTableView>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProfileSelected", (view, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn RefreshView(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshView", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ShowCopyFromProfile(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowCopyFromProfile", ())?;
        Ok(__cordl_ret.into())
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
    pub fn add_onRequestSelectProfileToCopyFrom(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut quest_hook::libil2cpp::Il2CppArray<
                    crate::System::ValueTuple_2<
                        *mut quest_hook::libil2cpp::Il2CppString,
                        i32,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onRequestSelectProfileToCopyFrom", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onRequestSelectProfileToCopyFrom(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut quest_hook::libil2cpp::Il2CppArray<
                    crate::System::ValueTuple_2<
                        *mut quest_hook::libil2cpp::Il2CppString,
                        i32,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onRequestSelectProfileToCopyFrom", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfilesSettingsViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::GameSettings::ControllerProfilesSettingsViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
