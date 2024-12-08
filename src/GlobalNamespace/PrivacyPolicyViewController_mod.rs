#[cfg(feature = "PrivacyPolicyViewController+ButtonType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivacyPolicyViewController_ButtonType {
    _cordl_Ok = 0i32,
}
#[cfg(feature = "PrivacyPolicyViewController+ButtonType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PrivacyPolicyViewController_ButtonType => ""
    ."PrivacyPolicyViewController/ButtonType"
);
#[cfg(feature = "PrivacyPolicyViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct PrivacyPolicyViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _okButton: *mut crate::UnityEngine::UI::Button,
    pub _iAcceptButton: *mut crate::UnityEngine::UI::Button,
    pub _textPageScrollView: *mut crate::HMUI::TextPageScrollView,
    pub _privacyPolicyLocalizedTextAsset: *mut crate::GlobalNamespace::LocalizedTextAsset,
    pub _updateNoticeLocalizationKey: *mut crate::System::String,
    pub didFinishEvent: *mut crate::System::Action_1<
        crate::GlobalNamespace::PrivacyPolicyViewController_ButtonType,
    >,
    pub _showUpdate: bool,
    pub _showIAcceptPrompt: bool,
}
#[cfg(feature = "PrivacyPolicyViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PrivacyPolicyViewController =>
    ""."PrivacyPolicyViewController"
);
#[cfg(feature = "PrivacyPolicyViewController")]
impl std::ops::Deref for crate::GlobalNamespace::PrivacyPolicyViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PrivacyPolicyViewController")]
impl std::ops::DerefMut for crate::GlobalNamespace::PrivacyPolicyViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PrivacyPolicyViewController")]
impl crate::GlobalNamespace::PrivacyPolicyViewController {
    #[cfg(feature = "PrivacyPolicyViewController+ButtonType")]
    pub type ButtonType = crate::GlobalNamespace::PrivacyPolicyViewController_ButtonType;
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
    pub fn Init(
        &mut self,
        showUpdate: bool,
        showIAcceptPrompt: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (showUpdate, showIAcceptPrompt))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _DidActivate_b__11_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__11_0", ())?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__11_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__11_1", ())?;
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
        value: *mut crate::System::Action_1<
            crate::GlobalNamespace::PrivacyPolicyViewController_ButtonType,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            crate::GlobalNamespace::PrivacyPolicyViewController_ButtonType,
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
#[cfg(feature = "PrivacyPolicyViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PrivacyPolicyViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
