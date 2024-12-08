#[cfg(feature = "SimpleDialogPromptViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct SimpleDialogPromptViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _titleText: *mut crate::TMPro::TextMeshProUGUI,
    pub _messageText: *mut crate::TMPro::TextMeshProUGUI,
    pub _buttons: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::UI::Button,
    >,
    pub _buttonTexts: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::TMPro::TextMeshProUGUI,
    >,
    pub _didFinishAction: *mut crate::System::Action_1<i32>,
}
#[cfg(feature = "SimpleDialogPromptViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SimpleDialogPromptViewController => ""
    ."SimpleDialogPromptViewController"
);
#[cfg(feature = "SimpleDialogPromptViewController")]
impl std::ops::Deref for SimpleDialogPromptViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SimpleDialogPromptViewController")]
impl std::ops::DerefMut for SimpleDialogPromptViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SimpleDialogPromptViewController")]
impl SimpleDialogPromptViewController {
    #[cfg(feature = "SimpleDialogPromptViewController+__c__DisplayClass5_0")]
    pub type __c__DisplayClass5_0 = crate::GlobalNamespace::SimpleDialogPromptViewController___c__DisplayClass5_0;
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
        Ok(__cordl_ret)
    }
    pub fn Init_Action_1_0(
        &mut self,
        title: *mut crate::System::String,
        message: *mut crate::System::String,
        buttonText: *mut crate::System::String,
        didFinishAction: *mut crate::System::Action_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (title, message, buttonText, didFinishAction))?;
        Ok(__cordl_ret)
    }
    pub fn Init_String_Action_1_1(
        &mut self,
        title: *mut crate::System::String,
        message: *mut crate::System::String,
        firstButtonText: *mut crate::System::String,
        secondButtonText: *mut crate::System::String,
        didFinishAction: *mut crate::System::Action_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (title, message, firstButtonText, secondButtonText, didFinishAction),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Init_String_String_Action_1_2(
        &mut self,
        title: *mut crate::System::String,
        message: *mut crate::System::String,
        firstButtonText: *mut crate::System::String,
        secondButtonText: *mut crate::System::String,
        thirdButtonText: *mut crate::System::String,
        didFinishAction: *mut crate::System::Action_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (
                    title,
                    message,
                    firstButtonText,
                    secondButtonText,
                    thirdButtonText,
                    didFinishAction,
                ),
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
#[cfg(feature = "SimpleDialogPromptViewController")]
impl quest_hook::libil2cpp::ObjectType for SimpleDialogPromptViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
