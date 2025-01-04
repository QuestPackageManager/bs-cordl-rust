#[cfg(feature = "HMUI+SelectableCell")]
#[repr(C)]
#[derive(Debug)]
pub struct SelectableCell {
    __cordl_parent: crate::HMUI::Interactable,
    pub _wasPressedSignal: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Signal>,
    pub selectionDidChangeEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_3<
            *mut crate::HMUI::SelectableCell,
            crate::HMUI::SelectableCell_TransitionType,
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    >,
    pub highlightDidChangeEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            *mut crate::HMUI::SelectableCell,
            crate::HMUI::SelectableCell_TransitionType,
        >,
    >,
    pub nonInteractableCellWasPressedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<*mut crate::HMUI::SelectableCell>,
    >,
    pub _highlighted_k__BackingField: bool,
    pub _selected_k__BackingField: bool,
}
#[cfg(feature = "HMUI+SelectableCell")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::SelectableCell => "HMUI"."SelectableCell"
);
#[cfg(feature = "HMUI+SelectableCell")]
impl std::ops::Deref for crate::HMUI::SelectableCell {
    type Target = crate::HMUI::Interactable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+SelectableCell")]
impl std::ops::DerefMut for crate::HMUI::SelectableCell {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+SelectableCell")]
impl crate::HMUI::SelectableCell {
    #[cfg(feature = "HMUI+SelectableCell+TransitionType")]
    pub type TransitionType = crate::HMUI::SelectableCell_TransitionType;
    pub fn ClearHighlight(
        &mut self,
        transitionType: crate::HMUI::SelectableCell_TransitionType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearHighlight", (transitionType))?;
        Ok(__cordl_ret.into())
    }
    pub fn HighlightDidChange(
        &mut self,
        transitionType: crate::HMUI::SelectableCell_TransitionType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HighlightDidChange", (transitionType))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalToggle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalToggle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerClick(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerClick", (eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerEnter(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerEnter", (eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerExit(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerExit", (eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnSubmit(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSubmit", (eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn SelectionDidChange(
        &mut self,
        transitionType: crate::HMUI::SelectableCell_TransitionType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SelectionDidChange", (transitionType))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetHighlight(
        &mut self,
        value: bool,
        transitionType: crate::HMUI::SelectableCell_TransitionType,
        ignoreCurrentValue: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetHighlight", (value, transitionType, ignoreCurrentValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSelected(
        &mut self,
        value: bool,
        transitionType: crate::HMUI::SelectableCell_TransitionType,
        changeOwner: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        ignoreCurrentValue: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetSelected",
                (value, transitionType, changeOwner, ignoreCurrentValue),
            )?;
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
    pub fn add_highlightDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::HMUI::SelectableCell,
                crate::HMUI::SelectableCell_TransitionType,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_highlightDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_nonInteractableCellWasPressedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::HMUI::SelectableCell>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_nonInteractableCellWasPressedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_selectionDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_3<
                *mut crate::HMUI::SelectableCell,
                crate::HMUI::SelectableCell_TransitionType,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_selectionDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_highlighted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_highlighted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selected(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_selected", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_highlightDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::HMUI::SelectableCell,
                crate::HMUI::SelectableCell_TransitionType,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_highlightDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_nonInteractableCellWasPressedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::HMUI::SelectableCell>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_nonInteractableCellWasPressedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_selectionDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_3<
                *mut crate::HMUI::SelectableCell,
                crate::HMUI::SelectableCell_TransitionType,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_selectionDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_highlighted(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_highlighted", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_selected(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_selected", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HMUI+SelectableCell")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::SelectableCell {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HMUI+SelectableCell")]
impl AsRef<crate::UnityEngine::EventSystems::IEventSystemHandler>
for crate::HMUI::SelectableCell {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IEventSystemHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+SelectableCell")]
impl AsMut<crate::UnityEngine::EventSystems::IEventSystemHandler>
for crate::HMUI::SelectableCell {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IEventSystemHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+SelectableCell")]
impl AsRef<crate::UnityEngine::EventSystems::IPointerClickHandler>
for crate::HMUI::SelectableCell {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IPointerClickHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+SelectableCell")]
impl AsMut<crate::UnityEngine::EventSystems::IPointerClickHandler>
for crate::HMUI::SelectableCell {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IPointerClickHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+SelectableCell")]
impl AsRef<crate::UnityEngine::EventSystems::IPointerEnterHandler>
for crate::HMUI::SelectableCell {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IPointerEnterHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+SelectableCell")]
impl AsMut<crate::UnityEngine::EventSystems::IPointerEnterHandler>
for crate::HMUI::SelectableCell {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IPointerEnterHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+SelectableCell")]
impl AsRef<crate::UnityEngine::EventSystems::IPointerExitHandler>
for crate::HMUI::SelectableCell {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IPointerExitHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+SelectableCell")]
impl AsMut<crate::UnityEngine::EventSystems::IPointerExitHandler>
for crate::HMUI::SelectableCell {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IPointerExitHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+SelectableCell")]
impl AsRef<crate::UnityEngine::EventSystems::ISubmitHandler>
for crate::HMUI::SelectableCell {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::ISubmitHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+SelectableCell")]
impl AsMut<crate::UnityEngine::EventSystems::ISubmitHandler>
for crate::HMUI::SelectableCell {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::ISubmitHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+SelectableCell+TransitionType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectableCell_TransitionType {
    #[default]
    Animated = 1i32,
    Instant = 0i32,
}
#[cfg(feature = "HMUI+SelectableCell+TransitionType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HMUI::SelectableCell_TransitionType => "HMUI"
    ."SelectableCell/TransitionType"
);
