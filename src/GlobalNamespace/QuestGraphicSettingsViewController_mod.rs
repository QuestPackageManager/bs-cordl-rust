#[cfg(feature = "QuestGraphicSettingsViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct QuestGraphicSettingsViewController {
    __cordl_parent: crate::GlobalNamespace::GraphicSettingsViewController,
    pub _mirror: *mut crate::GlobalNamespace::PresetsSettingsController,
    pub _120HzMode: *mut crate::UnityEngine::UI::Toggle,
    pub _stinsonOnlyEntries: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::GameObject,
    >,
}
#[cfg(feature = "QuestGraphicSettingsViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::QuestGraphicSettingsViewController => ""
    ."QuestGraphicSettingsViewController"
);
#[cfg(feature = "QuestGraphicSettingsViewController")]
impl std::ops::Deref for crate::GlobalNamespace::QuestGraphicSettingsViewController {
    type Target = crate::GlobalNamespace::GraphicSettingsViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "QuestGraphicSettingsViewController")]
impl std::ops::DerefMut for crate::GlobalNamespace::QuestGraphicSettingsViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "QuestGraphicSettingsViewController")]
impl crate::GlobalNamespace::QuestGraphicSettingsViewController {
    pub const kBalancedPresetKey: &'static str = "Balanced";
    pub const kFrameratePresetKey: &'static str = "Framerate";
    pub const kNoMirrorSuffix: &'static str = "_NoMirror";
    #[cfg(feature = "QuestGraphicSettingsViewController+_InitializeSettingsAsync_d__8")]
    pub type _InitializeSettingsAsync_d__8 = crate::GlobalNamespace::QuestGraphicSettingsViewController__InitializeSettingsAsync_d__8;
    #[cfg(feature = "QuestGraphicSettingsViewController+_UpdatePreset_d__12")]
    pub type _UpdatePreset_d__12 = crate::GlobalNamespace::QuestGraphicSettingsViewController__UpdatePreset_d__12;
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
    pub fn GetPresetKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetPresetKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Handle120HzToggleValueChangedAsync(
        &mut self,
        newState: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Handle120HzToggleValueChangedAsync", (newState))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMirrorChanged(
        &mut self,
        newValue: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMirrorChanged", (newValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeSettingsAsync(
        &mut self,
        firstActivation: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeSettingsAsync", (firstActivation))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UpdatePreset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdatePreset", ())?;
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
}
#[cfg(feature = "QuestGraphicSettingsViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::QuestGraphicSettingsViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
