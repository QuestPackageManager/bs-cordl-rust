#[cfg(feature = "UnityEngine+UIElements+GenericDropdownMenu")]
#[repr(C)]
#[derive(Debug)]
pub struct GenericDropdownMenu {
    __cordl_parent: crate::System::Object,
    pub m_Items: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::GenericDropdownMenu_MenuItem,
    >,
    pub m_MenuContainer: *mut crate::UnityEngine::UIElements::VisualElement,
    pub m_OuterContainer: *mut crate::UnityEngine::UIElements::VisualElement,
    pub m_ScrollView: *mut crate::UnityEngine::UIElements::ScrollView,
    pub m_PanelRootVisualContainer: *mut crate::UnityEngine::UIElements::VisualElement,
    pub m_TargetElement: *mut crate::UnityEngine::UIElements::VisualElement,
    pub m_DesiredRect: crate::UnityEngine::Rect,
    pub m_NavigationManipulator: *mut crate::UnityEngine::UIElements::KeyboardNavigationManipulator,
    pub _isSingleSelectionDropdown_k__BackingField: bool,
    pub _closeOnParentResize_k__BackingField: bool,
    pub m_MousePosition: crate::UnityEngine::Vector2,
}
#[cfg(feature = "UnityEngine+UIElements+GenericDropdownMenu")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::GenericDropdownMenu =>
    "UnityEngine.UIElements"."GenericDropdownMenu"
);
#[cfg(feature = "UnityEngine+UIElements+GenericDropdownMenu")]
impl std::ops::Deref for crate::UnityEngine::UIElements::GenericDropdownMenu {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+GenericDropdownMenu")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::GenericDropdownMenu {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+GenericDropdownMenu")]
impl crate::UnityEngine::UIElements::GenericDropdownMenu {
    #[cfg(feature = "UnityEngine+UIElements+GenericDropdownMenu+MenuItem")]
    pub type MenuItem = crate::UnityEngine::UIElements::GenericDropdownMenu_MenuItem;
    #[cfg(feature = "UnityEngine+UIElements+GenericDropdownMenu+__c__DisplayClass39_0")]
    pub type __c__DisplayClass39_0 = crate::UnityEngine::UIElements::GenericDropdownMenu___c__DisplayClass39_0;
    pub fn AddItem_Action0(
        &mut self,
        itemName: *mut crate::System::String,
        isChecked: bool,
        action: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddItem", (itemName, isChecked, action))?;
        Ok(__cordl_ret)
    }
    pub fn AddItem_Action_1_Object1(
        &mut self,
        itemName: *mut crate::System::String,
        isChecked: bool,
        action: *mut crate::System::Action_1<*mut crate::System::Object>,
        data: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddItem", (itemName, isChecked, action, data))?;
        Ok(__cordl_ret)
    }
    pub fn AddItem__cordl_bool_Object2(
        &mut self,
        itemName: *mut crate::System::String,
        isChecked: bool,
        isEnabled: bool,
        data: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::GenericDropdownMenu_MenuItem,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::GenericDropdownMenu_MenuItem = __cordl_object
            .invoke("AddItem", (itemName, isChecked, isEnabled, data))?;
        Ok(__cordl_ret)
    }
    pub fn AddSeparator(
        &mut self,
        path: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSeparator", (path))?;
        Ok(__cordl_ret)
    }
    pub fn Apply_EventBase0(
        &mut self,
        op: crate::UnityEngine::UIElements::KeyboardNavigationOperation,
        sourceEvent: *mut crate::UnityEngine::UIElements::EventBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Apply", (op, sourceEvent))?;
        Ok(__cordl_ret)
    }
    pub fn Apply_KeyboardNavigationOperation1(
        &mut self,
        op: crate::UnityEngine::UIElements::KeyboardNavigationOperation,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Apply", (op))?;
        Ok(__cordl_ret)
    }
    pub fn ChangeSelectedIndex(
        &mut self,
        newIndex: i32,
        previousIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ChangeSelectedIndex", (newIndex, previousIndex))?;
        Ok(__cordl_ret)
    }
    pub fn DropDown(
        &mut self,
        position: crate::UnityEngine::Rect,
        targetElement: *mut crate::UnityEngine::UIElements::VisualElement,
        anchored: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DropDown", (position, targetElement, anchored))?;
        Ok(__cordl_ret)
    }
    pub fn EnsureVisibilityInParent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnsureVisibilityInParent", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSelectedIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetSelectedIndex", ())?;
        Ok(__cordl_ret)
    }
    pub fn Hide(
        &mut self,
        giveFocusBack: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Hide", (giveFocusBack))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnAttachToPanel(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::AttachToPanelEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnAttachToPanel", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn OnContainerGeometryChanged(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::GeometryChangedEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnContainerGeometryChanged", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn OnDetachFromPanel(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::DetachFromPanelEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDetachFromPanel", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn OnFocusOut(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::FocusOutEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnFocusOut", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn OnParentResized(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::GeometryChangedEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnParentResized", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerDown(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::PointerDownEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerDown", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerMove(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::PointerMoveEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerMove", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerUp(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::PointerUpEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerUp", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn OnTargetElementDetachFromPanel(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::DetachFromPanelEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnTargetElementDetachFromPanel", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateSelection(
        &mut self,
        target: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateSelection", (target))?;
        Ok(__cordl_ret)
    }
    pub fn _Apply_g__UpdateSelectionDown_39_0(
        &mut self,
        newIndex: i32,
        _cordl_fixed_empty_name_whitespace: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::GenericDropdownMenu___c__DisplayClass39_0,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "<Apply>g__UpdateSelectionDown|39_0",
                (newIndex, _cordl_fixed_empty_name_whitespace),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _Apply_g__UpdateSelectionUp_39_1(
        &mut self,
        newIndex: i32,
        _cordl_fixed_empty_name_whitespace: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::GenericDropdownMenu___c__DisplayClass39_0,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "<Apply>g__UpdateSelectionUp|39_1",
                (newIndex, _cordl_fixed_empty_name_whitespace),
            )?;
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
    pub fn get_closeOnParentResize(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_closeOnParentResize", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_contentContainer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::VisualElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::VisualElement = __cordl_object
            .invoke("get_contentContainer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isSingleSelectionDropdown(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_isSingleSelectionDropdown", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_closeOnParentResize(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_closeOnParentResize", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_isSingleSelectionDropdown(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isSingleSelectionDropdown", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+GenericDropdownMenu")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::GenericDropdownMenu {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+GenericDropdownMenu+MenuItem")]
#[repr(C)]
#[derive(Debug)]
pub struct GenericDropdownMenu_MenuItem {
    __cordl_parent: crate::System::Object,
    pub name: *mut crate::System::String,
    pub element: *mut crate::UnityEngine::UIElements::VisualElement,
    pub action: *mut crate::System::Action,
    pub actionUserData: *mut crate::System::Action_1<*mut crate::System::Object>,
}
#[cfg(feature = "UnityEngine+UIElements+GenericDropdownMenu+MenuItem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::GenericDropdownMenu_MenuItem => "UnityEngine.UIElements"
    ."GenericDropdownMenu/MenuItem"
);
#[cfg(feature = "UnityEngine+UIElements+GenericDropdownMenu+MenuItem")]
impl std::ops::Deref for crate::UnityEngine::UIElements::GenericDropdownMenu_MenuItem {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+GenericDropdownMenu+MenuItem")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::GenericDropdownMenu_MenuItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+GenericDropdownMenu+MenuItem")]
impl crate::UnityEngine::UIElements::GenericDropdownMenu_MenuItem {
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
#[cfg(feature = "UnityEngine+UIElements+GenericDropdownMenu+MenuItem")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::GenericDropdownMenu_MenuItem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}