#[cfg(feature = "CreateServerViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct CreateServerViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _createServerButton: *mut crate::UnityEngine::UI::Button,
    pub _cancelCreateServerButton: *mut crate::UnityEngine::UI::Button,
    pub _createServerFormController: *mut crate::GlobalNamespace::CreateServerFormController,
    pub didFinishEvent: *mut crate::System::Action_2<
        bool,
        crate::GlobalNamespace::CreateServerFormData,
    >,
    pub _multiplayerModeSettings: *mut crate::GlobalNamespace::MultiplayerModeSettings,
}
#[cfg(feature = "CreateServerViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::CreateServerViewController =>
    ""."CreateServerViewController"
);
#[cfg(feature = "CreateServerViewController")]
impl std::ops::Deref for crate::GlobalNamespace::CreateServerViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CreateServerViewController")]
impl std::ops::DerefMut for crate::GlobalNamespace::CreateServerViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CreateServerViewController")]
impl crate::GlobalNamespace::CreateServerViewController {
    pub fn ApplyAndGetData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::CreateServerFormData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::CreateServerFormData = __cordl_object
            .invoke("ApplyAndGetData", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Setup(
        &mut self,
        multiplayerModeSettings: *mut crate::GlobalNamespace::MultiplayerModeSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Setup", (multiplayerModeSettings))?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__10_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__10_0", ())?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__10_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__10_1", ())?;
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
    pub fn add_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            bool,
            crate::GlobalNamespace::CreateServerFormData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_multiplayerModeSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::MultiplayerModeSettings,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::MultiplayerModeSettings = __cordl_object
            .invoke("get_multiplayerModeSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            bool,
            crate::GlobalNamespace::CreateServerFormData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "CreateServerViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::CreateServerViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
