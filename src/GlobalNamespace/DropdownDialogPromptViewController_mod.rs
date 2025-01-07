#[cfg(feature = "DropdownDialogPromptViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct DropdownDialogPromptViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _titleText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _messageText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _dropdownLabel: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _dropdown: quest_hook::libil2cpp::Gc<crate::HMUI::SimpleTextDropdown>,
    pub _buttons: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::DropdownDialogPromptViewController_ButtonAndLabel,
            >,
        >,
    >,
    pub _didFinishAction: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<crate::System::ValueTuple_2<i32, i32>>,
    >,
    pub _dropdownValues: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::System::ValueTuple_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                i32,
            >,
        >,
    >,
}
#[cfg(feature = "DropdownDialogPromptViewController")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::DropdownDialogPromptViewController {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "DropdownDialogPromptViewController";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "DropdownDialogPromptViewController")]
impl std::ops::Deref for crate::GlobalNamespace::DropdownDialogPromptViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DropdownDialogPromptViewController")]
impl std::ops::DerefMut for crate::GlobalNamespace::DropdownDialogPromptViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DropdownDialogPromptViewController")]
impl crate::GlobalNamespace::DropdownDialogPromptViewController {
    #[cfg(feature = "DropdownDialogPromptViewController+ButtonAndLabel")]
    pub type ButtonAndLabel = crate::GlobalNamespace::DropdownDialogPromptViewController_ButtonAndLabel;
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
    pub fn Init(
        &mut self,
        title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dropdownLabel: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dropdownValues: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::ValueTuple_2<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    i32,
                >,
            >,
        >,
        didFinishAction: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::System::ValueTuple_2<i32, i32>>,
        >,
        buttonTexts: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
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
                    dropdownLabel,
                    dropdownValues,
                    didFinishAction,
                    buttonTexts,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnButtonClick(
        &mut self,
        buttonNum: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnButtonClick", (buttonNum))?;
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
    pub fn add__didFinishAction(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::System::ValueTuple_2<i32, i32>>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add__didFinishAction", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedValue(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_selectedValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove__didFinishAction(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::System::ValueTuple_2<i32, i32>>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove__didFinishAction", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "DropdownDialogPromptViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DropdownDialogPromptViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "DropdownDialogPromptViewController+ButtonAndLabel")]
#[repr(C)]
#[derive(Debug)]
pub struct DropdownDialogPromptViewController_ButtonAndLabel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _component: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub _label: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
}
#[cfg(feature = "DropdownDialogPromptViewController+ButtonAndLabel")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::DropdownDialogPromptViewController_ButtonAndLabel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ButtonAndLabel";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "DropdownDialogPromptViewController+ButtonAndLabel")]
impl std::ops::Deref
for crate::GlobalNamespace::DropdownDialogPromptViewController_ButtonAndLabel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DropdownDialogPromptViewController+ButtonAndLabel")]
impl std::ops::DerefMut
for crate::GlobalNamespace::DropdownDialogPromptViewController_ButtonAndLabel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DropdownDialogPromptViewController+ButtonAndLabel")]
impl crate::GlobalNamespace::DropdownDialogPromptViewController_ButtonAndLabel {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn get_component(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button> = __cordl_object
            .invoke("get_component", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_label(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI> = __cordl_object
            .invoke("get_label", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "DropdownDialogPromptViewController+ButtonAndLabel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DropdownDialogPromptViewController_ButtonAndLabel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
