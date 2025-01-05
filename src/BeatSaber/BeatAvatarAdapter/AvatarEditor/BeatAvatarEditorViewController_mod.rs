#[cfg(
    feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+BeatAvatarEditorViewController"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BeatAvatarEditorViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _skinColorValuePicker: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NamedColorListController,
    >,
    pub _headTopValuePicker: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NamedIntListController,
    >,
    pub _eyesValuePicker: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NamedIntListController,
    >,
    pub _handsValuePicker: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NamedIntListController,
    >,
    pub _clothesValuePicker: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NamedIntListController,
    >,
    pub _headTopPrimaryColorButtonController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ColorPickerButtonController,
    >,
    pub _headTopSecondaryColorButtonController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ColorPickerButtonController,
    >,
    pub _handsColorButtonController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ColorPickerButtonController,
    >,
    pub _clothesColorButtonControllerPrimary: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ColorPickerButtonController,
    >,
    pub _clothesColorButtonControllerSecondary: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ColorPickerButtonController,
    >,
    pub _clothesColorButtonControllerDetail: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ColorPickerButtonController,
    >,
    pub _randomizeAllButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub _undoButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub _redoButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub _applyButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub _cancelButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub _applyButtonText: quest_hook::libil2cpp::Gc<crate::HMUI::CurvedTextMeshPro>,
    pub _eyesPreviewImage: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
    pub _avatarPartsModel: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::BeatAvatarSDK::AvatarPartsModel,
    >,
    pub _avatarDataModel: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::BeatAvatarSDK::AvatarDataModel,
    >,
    pub didRequestColorChangeEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_4<
            quest_hook::libil2cpp::Gc<
                crate::System::Action_1<crate::UnityEngine::Color>,
            >,
            crate::UnityEngine::Color,
            crate::BeatSaber::BeatAvatarSDK::AvatarPart,
            i32,
        >,
    >,
    pub randomizeAllButtonWasPressedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action,
    >,
    pub didChangedAvatarPartEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<crate::BeatSaber::BeatAvatarSDK::AvatarPart>,
    >,
    pub cancelButtonWasPressedEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub okButtonWasPressedEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub _avatarEditHistory: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::AvatarEditHistory,
    >,
    pub _buttonBinder: quest_hook::libil2cpp::Gc<crate::HMUI::ButtonBinder>,
    pub _intPickerBinder: quest_hook::libil2cpp::Gc<
        crate::HMUI::ValueChangedBinder_1<i32>,
    >,
    pub _lastEditedPart: crate::BeatSaber::BeatAvatarSDK::AvatarPart,
}
#[cfg(
    feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+BeatAvatarEditorViewController"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::BeatAvatarAdapter::AvatarEditor::BeatAvatarEditorViewController =>
    "BeatSaber.BeatAvatarAdapter.AvatarEditor"."BeatAvatarEditorViewController"
);
#[cfg(
    feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+BeatAvatarEditorViewController"
)]
impl std::ops::Deref
for crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::BeatAvatarEditorViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+BeatAvatarEditorViewController"
)]
impl std::ops::DerefMut
for crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::BeatAvatarEditorViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+BeatAvatarEditorViewController"
)]
impl crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::BeatAvatarEditorViewController {
    pub const kCreateApplyButtonLocalizationKey: &'static str = "BUTTON_CREATE_AVATAR";
    pub const kEditApplyButtonLocalizationKey: &'static str = "BUTTON_APPLY";
    pub fn CreateColorValuePairsForAvatarPartCollection(
        &mut self,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::BeatSaber::BeatAvatarSDK::SkinColorSO,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::NamedColorListController_ColorValuePair,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::NamedColorListController_ColorValuePair,
            >,
        > = __cordl_object
            .invoke("CreateColorValuePairsForAvatarPartCollection", (colors))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateTextValuePairsForAvatarPartCollection<T>(
        partCollection: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::NamedIntListController_TextValuePair,
            >,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::NamedIntListController_TextValuePair,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateTextValuePairsForAvatarPartCollection", (partCollection))?;
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
    pub fn EyesValuePickerHasChanged(
        &mut self,
        eyesId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EyesValuePickerHasChanged", (eyesId))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleApplyButtonWasPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleApplyButtonWasPressed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleCancelButtonWasPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleCancelButtonWasPressed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleRandomizeAllButtonWasPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleRandomizeAllButtonWasPressed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleRandomizeColorsButtonWasPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleRandomizeColorsButtonWasPressed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleRandomizeModelsButtonWasPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleRandomizeModelsButtonWasPressed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleRedoButtonWasPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleRedoButtonWasPressed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSkinColorDidChanged(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSkinColorDidChanged", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleUndoButtonWasPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleUndoButtonWasPressed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitHistory(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitHistory", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OneTimeInitialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OneTimeInitialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RefreshUi(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshUi", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReportAllChangedAndUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReportAllChangedAndUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveColorChange(
        &mut self,
        avatarEditPart: crate::BeatSaber::BeatAvatarSDK::AvatarPart,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SaveColorChange", (avatarEditPart))?;
        Ok(__cordl_ret.into())
    }
    pub fn Setup(
        &mut self,
        showAsCreateView: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Setup", (showAsCreateView))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetupColorButton(
        &mut self,
        button: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
        colorSetter: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Color>,
        >,
        currentColorGetter: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<crate::UnityEngine::Color>,
        >,
        avatarEditPart: crate::BeatSaber::BeatAvatarSDK::AvatarPart,
        uvSegment: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetupColorButton",
                (button, colorSetter, currentColorGetter, avatarEditPart, uvSegment),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetupValuePicker<T>(
        &mut self,
        partCollection: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<T>,
        >,
        valuePicker: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NamedIntListController,
        >,
        setIdAction: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        avatarEditPart: crate::BeatSaber::BeatAvatarSDK::AvatarPart,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetupValuePicker",
                (partCollection, valuePicker, setIdAction, avatarEditPart),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateButtons(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateButtons", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _OneTimeInitialize_b__45_0(
        &mut self,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<OneTimeInitialize>b__45_0", (color))?;
        Ok(__cordl_ret.into())
    }
    pub fn _OneTimeInitialize_b__45_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("<OneTimeInitialize>b__45_1", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _OneTimeInitialize_b__45_10(
        &mut self,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<OneTimeInitialize>b__45_10", (color))?;
        Ok(__cordl_ret.into())
    }
    pub fn _OneTimeInitialize_b__45_11(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("<OneTimeInitialize>b__45_11", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _OneTimeInitialize_b__45_12(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<OneTimeInitialize>b__45_12", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn _OneTimeInitialize_b__45_13(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<OneTimeInitialize>b__45_13", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn _OneTimeInitialize_b__45_14(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<OneTimeInitialize>b__45_14", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn _OneTimeInitialize_b__45_2(
        &mut self,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<OneTimeInitialize>b__45_2", (color))?;
        Ok(__cordl_ret.into())
    }
    pub fn _OneTimeInitialize_b__45_3(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("<OneTimeInitialize>b__45_3", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _OneTimeInitialize_b__45_4(
        &mut self,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<OneTimeInitialize>b__45_4", (color))?;
        Ok(__cordl_ret.into())
    }
    pub fn _OneTimeInitialize_b__45_5(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("<OneTimeInitialize>b__45_5", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _OneTimeInitialize_b__45_6(
        &mut self,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<OneTimeInitialize>b__45_6", (color))?;
        Ok(__cordl_ret.into())
    }
    pub fn _OneTimeInitialize_b__45_7(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("<OneTimeInitialize>b__45_7", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _OneTimeInitialize_b__45_8(
        &mut self,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<OneTimeInitialize>b__45_8", (color))?;
        Ok(__cordl_ret.into())
    }
    pub fn _OneTimeInitialize_b__45_9(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("<OneTimeInitialize>b__45_9", ())?;
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
    pub fn add_cancelButtonWasPressedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_cancelButtonWasPressedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didChangedAvatarPartEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::BeatSaber::BeatAvatarSDK::AvatarPart>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didChangedAvatarPartEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didRequestColorChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_4<
                quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<crate::UnityEngine::Color>,
                >,
                crate::UnityEngine::Color,
                crate::BeatSaber::BeatAvatarSDK::AvatarPart,
                i32,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didRequestColorChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_okButtonWasPressedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_okButtonWasPressedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_randomizeAllButtonWasPressedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_randomizeAllButtonWasPressedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_cancelButtonWasPressedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_cancelButtonWasPressedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didChangedAvatarPartEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::BeatSaber::BeatAvatarSDK::AvatarPart>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didChangedAvatarPartEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didRequestColorChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_4<
                quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<crate::UnityEngine::Color>,
                >,
                crate::UnityEngine::Color,
                crate::BeatSaber::BeatAvatarSDK::AvatarPart,
                i32,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didRequestColorChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_okButtonWasPressedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_okButtonWasPressedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_randomizeAllButtonWasPressedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_randomizeAllButtonWasPressedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+BeatAvatarEditorViewController"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::BeatAvatarEditorViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
