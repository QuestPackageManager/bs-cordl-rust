#[cfg(feature = "UnityEngine+UIElements+ReusableTreeViewItem")]
#[repr(C)]
#[derive(Debug)]
pub struct ReusableTreeViewItem {
    __cordl_parent: crate::UnityEngine::UIElements::ReusableCollectionItem,
    pub m_Toggle: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Toggle>,
    pub m_Container: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub m_IndentElement: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub m_BindableContainer: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub m_Checkmark: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub onPointerUp: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<*mut crate::UnityEngine::UIElements::PointerUpEvent>,
    >,
    pub onToggleValueChanged: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<*mut crate::UnityEngine::UIElements::ChangeEvent_1<bool>>,
    >,
    pub m_Depth: i32,
    pub m_IndentWidth: f32,
    pub m_PointerUpCallback: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::EventCallback_1<
            *mut crate::UnityEngine::UIElements::PointerUpEvent,
        >,
    >,
    pub m_ToggleValueChangedCallback: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::EventCallback_1<
            *mut crate::UnityEngine::UIElements::ChangeEvent_1<bool>,
        >,
    >,
    pub m_ToggleGeometryChangedCallback: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::EventCallback_1<
            *mut crate::UnityEngine::UIElements::GeometryChangedEvent,
        >,
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        item: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (item))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitExpandHierarchy(
        &mut self,
        root: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        item: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitExpandHierarchy", (root, item))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnPointerUp(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::PointerUpEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerUp", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnToggleGeometryChanged(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::GeometryChangedEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnToggleGeometryChanged", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnToggleValueChanged(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ChangeEvent_1<bool>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnToggleValueChanged", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn PreAttachElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PreAttachElement", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn UpdateIndentLayout(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateIndentLayout", ())?;
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
    pub fn add_onPointerUp(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::UnityEngine::UIElements::PointerUpEvent>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onPointerUp", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_onToggleValueChanged(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::UnityEngine::UIElements::ChangeEvent_1<bool>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onToggleValueChanged", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rootElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("get_rootElement", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onPointerUp(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::UnityEngine::UIElements::PointerUpEvent>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onPointerUp", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onToggleValueChanged(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::UnityEngine::UIElements::ChangeEvent_1<bool>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onToggleValueChanged", (value))?;
        Ok(__cordl_ret.into())
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
