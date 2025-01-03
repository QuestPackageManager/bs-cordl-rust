#[cfg(feature = "UnityEngine+UIElements+ReusableCollectionItem")]
#[repr(C)]
#[derive(Debug)]
pub struct ReusableCollectionItem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _bindableElement_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub _animator_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::Experimental::ValueAnimation_1<
            crate::UnityEngine::UIElements::Experimental::StyleValues,
        >,
    >,
    pub _index_k__BackingField: i32,
    pub _id_k__BackingField: i32,
    pub _isDragGhost_k__BackingField: bool,
    pub onGeometryChanged: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            *mut crate::UnityEngine::UIElements::ReusableCollectionItem,
        >,
    >,
    pub m_GeometryChangedEventCallback: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::EventCallback_1<
            *mut crate::UnityEngine::UIElements::GeometryChangedEvent,
        >,
    >,
    pub onDestroy: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            *mut crate::UnityEngine::UIElements::ReusableCollectionItem,
        >,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+ReusableCollectionItem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::ReusableCollectionItem
    => "UnityEngine.UIElements"."ReusableCollectionItem"
);
#[cfg(feature = "UnityEngine+UIElements+ReusableCollectionItem")]
impl std::ops::Deref for crate::UnityEngine::UIElements::ReusableCollectionItem {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ReusableCollectionItem")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::ReusableCollectionItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ReusableCollectionItem")]
impl crate::UnityEngine::UIElements::ReusableCollectionItem {
    pub fn DestroyElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroyElement", ())?;
        Ok(__cordl_ret.into())
    }
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnGeometryChanged(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::GeometryChangedEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnGeometryChanged", (evt))?;
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
    pub fn SetDragGhost(
        &mut self,
        dragGhost: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDragGhost", (dragGhost))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSelected(
        &mut self,
        selected: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSelected", (selected))?;
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
    pub fn add_onDestroy(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::UnityEngine::UIElements::ReusableCollectionItem,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onDestroy", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_onGeometryChanged(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::UnityEngine::UIElements::ReusableCollectionItem,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onGeometryChanged", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_animator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Experimental::ValueAnimation_1<
                crate::UnityEngine::UIElements::Experimental::StyleValues,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Experimental::ValueAnimation_1<
                crate::UnityEngine::UIElements::Experimental::StyleValues,
            >,
        > = __cordl_object.invoke("get_animator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bindableElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("get_bindableElement", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_id(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_id", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_index(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_index", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isDragGhost(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isDragGhost", ())?;
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
    pub fn remove_onDestroy(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::UnityEngine::UIElements::ReusableCollectionItem,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onDestroy", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onGeometryChanged(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::UnityEngine::UIElements::ReusableCollectionItem,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onGeometryChanged", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_animator(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Experimental::ValueAnimation_1<
                crate::UnityEngine::UIElements::Experimental::StyleValues,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_animator", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_bindableElement(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_bindableElement", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_id(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_id", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_index(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_index", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isDragGhost(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isDragGhost", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+ReusableCollectionItem")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::ReusableCollectionItem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
