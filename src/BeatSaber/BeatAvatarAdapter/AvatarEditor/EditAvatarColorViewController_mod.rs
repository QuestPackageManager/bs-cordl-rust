#[cfg(
    feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+EditAvatarColorViewController"
)]
#[repr(C)]
#[derive(Debug)]
pub struct EditAvatarColorViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _hsvPanelController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::HSVPanelController,
    >,
    pub _previousColorPanelController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PreviousColorPanelController,
    >,
    pub _cancelButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub _applyButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub didChangeColorEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<crate::UnityEngine::Color>,
    >,
    pub didFinishEvent: quest_hook::libil2cpp::Gc<crate::System::Action_1<bool>>,
    pub _colorCallback: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<crate::UnityEngine::Color>,
    >,
    pub _initialColor: crate::UnityEngine::Color,
    pub _colorChanged: bool,
    pub _buttonBinder: quest_hook::libil2cpp::Gc<crate::HMUI::ButtonBinder>,
}
#[cfg(
    feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+EditAvatarColorViewController"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::BeatAvatarAdapter::AvatarEditor::EditAvatarColorViewController =>
    "BeatSaber.BeatAvatarAdapter.AvatarEditor"."EditAvatarColorViewController"
);
#[cfg(
    feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+EditAvatarColorViewController"
)]
impl std::ops::Deref
for crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::EditAvatarColorViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+EditAvatarColorViewController"
)]
impl std::ops::DerefMut
for crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::EditAvatarColorViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+EditAvatarColorViewController"
)]
impl crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::EditAvatarColorViewController {
    pub fn ChangeColor(
        &mut self,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ChangeColor", (color))?;
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
    pub fn HandleHSVPanelControllerColorDidChange(
        &mut self,
        color: crate::UnityEngine::Color,
        colorChangeUIEventType: crate::GlobalNamespace::ColorChangeUIEventType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleHSVPanelControllerColorDidChange",
                (color, colorChangeUIEventType),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandlePreviousColorPanelControllerColorWasSelected(
        &mut self,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePreviousColorPanelControllerColorWasSelected", (color))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColor(
        &mut self,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColor", (color))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColorCallback(
        &mut self,
        colorCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Color>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColorCallback", (colorCallback))?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
    pub fn add_didChangeColorEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Color>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didChangeColorEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<bool>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFinishEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_color(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_color", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didChangeColorEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Color>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didChangeColorEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<bool>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFinishEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+EditAvatarColorViewController"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::EditAvatarColorViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
