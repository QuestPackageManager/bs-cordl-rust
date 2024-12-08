#[cfg(feature = "UnityEngine+UIElements+ReusableTreeViewItem")]
#[repr(C)]
#[derive(Debug)]
pub struct ReusableTreeViewItem {
    __cordl_parent: crate::UnityEngine::UIElements::ReusableCollectionItem,
    pub m_Toggle: *mut crate::UnityEngine::UIElements::Toggle,
    pub m_Container: *mut crate::UnityEngine::UIElements::VisualElement,
    pub m_IndentElement: *mut crate::UnityEngine::UIElements::VisualElement,
    pub m_BindableContainer: *mut crate::UnityEngine::UIElements::VisualElement,
    pub m_Checkmark: *mut crate::UnityEngine::UIElements::VisualElement,
    pub onPointerUp: *mut crate::System::Action_1<
        *mut crate::UnityEngine::UIElements::PointerUpEvent,
    >,
    pub onToggleValueChanged: *mut crate::System::Action_1<
        *mut crate::UnityEngine::UIElements::ChangeEvent_1<bool>,
    >,
    pub m_Depth: i32,
    pub m_IndentWidth: f32,
    pub m_PointerUpCallback: *mut crate::UnityEngine::UIElements::EventCallback_1<
        *mut crate::UnityEngine::UIElements::PointerUpEvent,
    >,
    pub m_ToggleValueChangedCallback: *mut crate::UnityEngine::UIElements::EventCallback_1<
        *mut crate::UnityEngine::UIElements::ChangeEvent_1<bool>,
    >,
    pub m_ToggleGeometryChangedCallback: *mut crate::UnityEngine::UIElements::EventCallback_1<
        *mut crate::UnityEngine::UIElements::GeometryChangedEvent,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+ReusableTreeViewItem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::ReusableTreeViewItem =>
    "UnityEngine.UIElements"."ReusableTreeViewItem"
);
#[cfg(feature = "UnityEngine+UIElements+ReusableTreeViewItem")]
impl std::ops::Deref for crate::UnityEngine::UIElements::ReusableTreeViewItem {
    type Target = crate::UnityEngine::UIElements::ReusableCollectionItem;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ReusableTreeViewItem")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::ReusableTreeViewItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ReusableTreeViewItem")]
impl crate::UnityEngine::UIElements::ReusableTreeViewItem {
    pub fn DetachElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DetachElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn Indent(
        &mut self,
        depth: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Indent", (depth))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        item: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (item))?;
        Ok(__cordl_ret)
    }
    pub fn InitExpandHierarchy(
        &mut self,
        root: *mut crate::UnityEngine::UIElements::VisualElement,
        item: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitExpandHierarchy", (root, item))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn OnToggleGeometryChanged(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::GeometryChangedEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnToggleGeometryChanged", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn OnToggleValueChanged(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::ChangeEvent_1<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnToggleValueChanged", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn PreAttachElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PreAttachElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetExpandedWithoutNotify(
        &mut self,
        expanded: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetExpandedWithoutNotify", (expanded))?;
        Ok(__cordl_ret)
    }
    pub fn SetToggleVisibility(
        &mut self,
        visible: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetToggleVisibility", (visible))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateIndentLayout(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateIndentLayout", ())?;
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
    pub fn add_onPointerUp(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::UnityEngine::UIElements::PointerUpEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onPointerUp", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_onToggleValueChanged(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::UnityEngine::UIElements::ChangeEvent_1<bool>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onToggleValueChanged", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_rootElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::VisualElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::VisualElement = __cordl_object
            .invoke("get_rootElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_onPointerUp(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::UnityEngine::UIElements::PointerUpEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onPointerUp", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_onToggleValueChanged(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::UnityEngine::UIElements::ChangeEvent_1<bool>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onToggleValueChanged", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+ReusableTreeViewItem")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::ReusableTreeViewItem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
