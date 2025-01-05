#[cfg(feature = "SoloModeSelectionViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct SoloModeSelectionViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _freePlayModeButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub _oneSaberModeButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub _noArrowsModeButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub _dismissButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub didFinishEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::SoloModeSelectionViewController,
            >,
            crate::GlobalNamespace::SoloModeSelectionViewController_MenuType,
        >,
    >,
}
#[cfg(feature = "SoloModeSelectionViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SoloModeSelectionViewController
    => ""."SoloModeSelectionViewController"
);
#[cfg(feature = "SoloModeSelectionViewController")]
impl std::ops::Deref for crate::GlobalNamespace::SoloModeSelectionViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SoloModeSelectionViewController")]
impl std::ops::DerefMut for crate::GlobalNamespace::SoloModeSelectionViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SoloModeSelectionViewController")]
impl crate::GlobalNamespace::SoloModeSelectionViewController {
    #[cfg(feature = "SoloModeSelectionViewController+MenuType")]
    pub type MenuType = crate::GlobalNamespace::SoloModeSelectionViewController_MenuType;
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
    pub fn HandleMenuButton(
        &mut self,
        subMenuType: crate::GlobalNamespace::SoloModeSelectionViewController_MenuType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMenuButton", (subMenuType))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _DidActivate_b__8_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__8_0", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _DidActivate_b__8_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__8_1", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _DidActivate_b__8_2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__8_2", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _DidActivate_b__8_3(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__8_3", ())?;
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
    pub fn add_didFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::SoloModeSelectionViewController,
                >,
                crate::GlobalNamespace::SoloModeSelectionViewController_MenuType,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFinishEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::SoloModeSelectionViewController,
                >,
                crate::GlobalNamespace::SoloModeSelectionViewController_MenuType,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFinishEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SoloModeSelectionViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SoloModeSelectionViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SoloModeSelectionViewController+MenuType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SoloModeSelectionViewController_MenuType {
    #[default]
    Back = 3i32,
    FreePlayMode = 0i32,
    NoArrowsMode = 1i32,
    OneSaberMode = 2i32,
}
#[cfg(feature = "SoloModeSelectionViewController+MenuType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SoloModeSelectionViewController_MenuType => ""
    ."SoloModeSelectionViewController/MenuType"
);
