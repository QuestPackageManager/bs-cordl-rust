#[cfg(feature = "UnityEngine+UI+Selectable")]
#[repr(C)]
#[derive(Debug)]
pub struct Selectable {
    __cordl_parent: crate::UnityEngine::EventSystems::UIBehaviour,
    pub m_EnableCalled: bool,
    pub m_Navigation: crate::UnityEngine::UI::Navigation,
    pub m_Transition: crate::UnityEngine::UI::Selectable_Transition,
    pub m_Colors: crate::UnityEngine::UI::ColorBlock,
    pub m_SpriteState: crate::UnityEngine::UI::SpriteState,
    pub m_AnimationTriggers: *mut crate::UnityEngine::UI::AnimationTriggers,
    pub m_Interactable: bool,
    pub m_TargetGraphic: *mut crate::UnityEngine::UI::Graphic,
    pub m_GroupsAllowInteraction: bool,
    pub m_CurrentIndex: i32,
    pub _isPointerInside_k__BackingField: bool,
    pub _isPointerDown_k__BackingField: bool,
    pub _hasSelection_k__BackingField: bool,
    pub m_CanvasGroupCache: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::CanvasGroup,
    >,
}
#[cfg(feature = "UnityEngine+UI+Selectable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::Selectable => "UnityEngine.UI"
    ."Selectable"
);
#[cfg(feature = "UnityEngine+UI+Selectable")]
impl std::ops::Deref for crate::UnityEngine::UI::Selectable {
    type Target = crate::UnityEngine::EventSystems::UIBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+Selectable")]
impl std::ops::DerefMut for crate::UnityEngine::UI::Selectable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+Selectable")]
impl crate::UnityEngine::UI::Selectable {
    #[cfg(feature = "UnityEngine+UI+Selectable+SelectionState")]
    pub type SelectionState = crate::UnityEngine::UI::Selectable_SelectionState;
    #[cfg(feature = "UnityEngine+UI+Selectable+Transition")]
    pub type Transition = crate::UnityEngine::UI::Selectable_Transition;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn DoSpriteSwap(
        &mut self,
        newSprite: *mut crate::UnityEngine::Sprite,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoSpriteSwap", (newSprite))?;
        Ok(__cordl_ret)
    }
    pub fn DoStateTransition(
        &mut self,
        state: crate::UnityEngine::UI::Selectable_SelectionState,
        instant: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoStateTransition", (state, instant))?;
        Ok(__cordl_ret)
    }
    pub fn EvaluateAndTransitionToSelectionState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EvaluateAndTransitionToSelectionState", ())?;
        Ok(__cordl_ret)
    }
    pub fn FindSelectable(
        &mut self,
        dir: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UI::Selectable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::Selectable = __cordl_object
            .invoke("FindSelectable", (dir))?;
        Ok(__cordl_ret)
    }
    pub fn FindSelectableOnDown(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UI::Selectable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::Selectable = __cordl_object
            .invoke("FindSelectableOnDown", ())?;
        Ok(__cordl_ret)
    }
    pub fn FindSelectableOnLeft(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UI::Selectable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::Selectable = __cordl_object
            .invoke("FindSelectableOnLeft", ())?;
        Ok(__cordl_ret)
    }
    pub fn FindSelectableOnRight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UI::Selectable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::Selectable = __cordl_object
            .invoke("FindSelectableOnRight", ())?;
        Ok(__cordl_ret)
    }
    pub fn FindSelectableOnUp(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UI::Selectable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::Selectable = __cordl_object
            .invoke("FindSelectableOnUp", ())?;
        Ok(__cordl_ret)
    }
    pub fn InstantClearState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstantClearState", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsHighlighted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsHighlighted", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsInteractable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsInteractable", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsPressed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsPressed", ())?;
        Ok(__cordl_ret)
    }
    pub fn Navigate(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::AxisEventData,
        sel: *mut crate::UnityEngine::UI::Selectable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Navigate", (eventData, sel))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnApplicationFocus(
        &mut self,
        hasFocus: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnApplicationFocus", (hasFocus))?;
        Ok(__cordl_ret)
    }
    pub fn OnCanvasGroupChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCanvasGroupChanged", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnDeselect(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::BaseEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDeselect", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn OnDidApplyAnimationProperties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDidApplyAnimationProperties", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnMove(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::AxisEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnMove", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerDown(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::PointerEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerDown", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerEnter(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::PointerEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerEnter", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerExit(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::PointerEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerExit", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerUp(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::PointerEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerUp", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn OnSelect(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::BaseEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSelect", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn OnSetProperty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSetProperty", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnTransformParentChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnTransformParentChanged", ())?;
        Ok(__cordl_ret)
    }
    pub fn ParentGroupAllowsInteraction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ParentGroupAllowsInteraction", ())?;
        Ok(__cordl_ret)
    }
    pub fn Select(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Select", ())?;
        Ok(__cordl_ret)
    }
    pub fn StartColorTween(
        &mut self,
        targetColor: crate::UnityEngine::Color,
        instant: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartColorTween", (targetColor, instant))?;
        Ok(__cordl_ret)
    }
    pub fn TriggerAnimation(
        &mut self,
        triggername: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TriggerAnimation", (triggername))?;
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
    pub fn get_animationTriggers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UI::AnimationTriggers> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::AnimationTriggers = __cordl_object
            .invoke("get_animationTriggers", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_animator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Animator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Animator = __cordl_object
            .invoke("get_animator", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_colors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UI::ColorBlock> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UI::ColorBlock = __cordl_object
            .invoke("get_colors", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_currentSelectionState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UI::Selectable_SelectionState,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UI::Selectable_SelectionState = __cordl_object
            .invoke("get_currentSelectionState", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_hasSelection(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasSelection", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_image(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UI::Image> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::Image = __cordl_object
            .invoke("get_image", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_interactable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_interactable", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isPointerDown(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isPointerDown", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isPointerInside(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isPointerInside", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_navigation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UI::Navigation> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UI::Navigation = __cordl_object
            .invoke("get_navigation", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_spriteState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UI::SpriteState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UI::SpriteState = __cordl_object
            .invoke("get_spriteState", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_targetGraphic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UI::Graphic> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::Graphic = __cordl_object
            .invoke("get_targetGraphic", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_transition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UI::Selectable_Transition> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UI::Selectable_Transition = __cordl_object
            .invoke("get_transition", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_animationTriggers(
        &mut self,
        value: *mut crate::UnityEngine::UI::AnimationTriggers,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_animationTriggers", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_colors(
        &mut self,
        value: crate::UnityEngine::UI::ColorBlock,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_colors", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_hasSelection(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_hasSelection", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_image(
        &mut self,
        value: *mut crate::UnityEngine::UI::Image,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_image", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_interactable(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_interactable", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_isPointerDown(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isPointerDown", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_isPointerInside(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isPointerInside", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_navigation(
        &mut self,
        value: crate::UnityEngine::UI::Navigation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_navigation", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_spriteState(
        &mut self,
        value: crate::UnityEngine::UI::SpriteState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_spriteState", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_targetGraphic(
        &mut self,
        value: *mut crate::UnityEngine::UI::Graphic,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_targetGraphic", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_transition(
        &mut self,
        value: crate::UnityEngine::UI::Selectable_Transition,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_transition", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UI+Selectable")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UI::Selectable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UI+Selectable+SelectionState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Selectable_SelectionState {
    Disabled = 4i32,
    Highlighted = 1i32,
    Normal = 0i32,
    Pressed = 2i32,
    Selected = 3i32,
}
#[cfg(feature = "UnityEngine+UI+Selectable+SelectionState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::Selectable_SelectionState =>
    "UnityEngine.UI"."Selectable/SelectionState"
);
#[cfg(feature = "UnityEngine+UI+Selectable+Transition")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Selectable_Transition {
    Animation = 3i32,
    ColorTint = 1i32,
    None = 0i32,
    SpriteSwap = 2i32,
}
#[cfg(feature = "UnityEngine+UI+Selectable+Transition")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::Selectable_Transition =>
    "UnityEngine.UI"."Selectable/Transition"
);
