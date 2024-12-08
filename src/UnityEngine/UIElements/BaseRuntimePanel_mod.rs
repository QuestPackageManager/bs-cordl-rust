#[cfg(feature = "UnityEngine+UIElements+BaseRuntimePanel")]
#[repr(C)]
#[derive(Debug)]
pub struct BaseRuntimePanel {
    __cordl_parent: crate::UnityEngine::UIElements::Panel,
    pub m_SelectableGameObject: *mut crate::UnityEngine::GameObject,
    pub m_RuntimePanelCreationIndex: i32,
    pub m_SortingPriority: f32,
    pub resolvedSortingIndex: i32,
    pub destroyed: *mut crate::System::Action,
    pub m_StandardWorldSpaceShader: *mut crate::UnityEngine::Shader,
    pub m_DrawToCameras: bool,
    pub targetTexture: *mut crate::UnityEngine::RenderTexture,
    pub panelToWorld: crate::UnityEngine::Matrix4x4,
    pub _targetDisplay_k__BackingField: i32,
    pub m_ScreenToPanelSpace: *mut crate::System::Func_2<
        crate::UnityEngine::Vector2,
        crate::UnityEngine::Vector2,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+BaseRuntimePanel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::BaseRuntimePanel =>
    "UnityEngine.UIElements"."BaseRuntimePanel"
);
#[cfg(feature = "UnityEngine+UIElements+BaseRuntimePanel")]
impl std::ops::Deref for crate::UnityEngine::UIElements::BaseRuntimePanel {
    type Target = crate::UnityEngine::UIElements::Panel;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseRuntimePanel")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::BaseRuntimePanel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseRuntimePanel")]
impl crate::UnityEngine::UIElements::BaseRuntimePanel {
    #[cfg(feature = "UnityEngine+UIElements+BaseRuntimePanel+__c")]
    pub type __c = crate::UnityEngine::UIElements::BaseRuntimePanel___c;
    pub fn AssignPanelToComponents(
        &mut self,
        panel: *mut crate::UnityEngine::UIElements::BaseRuntimePanel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AssignPanelToComponents", (panel))?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        ownerObject: *mut crate::UnityEngine::ScriptableObject,
        dispatcher: *mut crate::UnityEngine::UIElements::EventDispatcher,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ownerObject, dispatcher))?;
        Ok(__cordl_object)
    }
    pub fn PointerEntersPanel(
        &mut self,
        pointerId: i32,
        position: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PointerEntersPanel", (pointerId, position))?;
        Ok(__cordl_ret)
    }
    pub fn PointerLeavesPanel(
        &mut self,
        pointerId: i32,
        position: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PointerLeavesPanel", (pointerId, position))?;
        Ok(__cordl_ret)
    }
    pub fn Repaint(
        &mut self,
        e: *mut crate::UnityEngine::Event,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Repaint", (e))?;
        Ok(__cordl_ret)
    }
    pub fn ScreenToPanel_Vector2_0(
        &mut self,
        screen: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("ScreenToPanel", (screen))?;
        Ok(__cordl_ret)
    }
    pub fn ScreenToPanel_Vector2_ByRefMut_ByRefMut__cordl_bool1(
        &mut self,
        screenPosition: crate::UnityEngine::Vector2,
        screenDelta: crate::UnityEngine::Vector2,
        panelPosition: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        panelDelta: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        allowOutside: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "ScreenToPanel",
                (screenPosition, screenDelta, panelPosition, panelDelta, allowOutside),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        ownerObject: *mut crate::UnityEngine::ScriptableObject,
        dispatcher: *mut crate::UnityEngine::UIElements::EventDispatcher,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ownerObject, dispatcher))?;
        Ok(__cordl_ret)
    }
    pub fn add_destroyed(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_destroyed", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_drawToCameras(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_drawToCameras", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_screenRenderingHeight(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_screenRenderingHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_screenRenderingWidth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_screenRenderingWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_screenToPanelSpace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Func_2<
            crate::UnityEngine::Vector2,
            crate::UnityEngine::Vector2,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Func_2<
            crate::UnityEngine::Vector2,
            crate::UnityEngine::Vector2,
        > = __cordl_object.invoke("get_screenToPanelSpace", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_selectableGameObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("get_selectableGameObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_sortingPriority(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_sortingPriority", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_standardWorldSpaceShader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Shader> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Shader = __cordl_object
            .invoke("get_standardWorldSpaceShader", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_targetDisplay(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_targetDisplay", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_destroyed(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_destroyed", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_drawToCameras(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_drawToCameras", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_screenToPanelSpace(
        &mut self,
        value: *mut crate::System::Func_2<
            crate::UnityEngine::Vector2,
            crate::UnityEngine::Vector2,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_screenToPanelSpace", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_selectableGameObject(
        &mut self,
        value: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_selectableGameObject", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_sortingPriority(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sortingPriority", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_targetDisplay(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_targetDisplay", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseRuntimePanel")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::BaseRuntimePanel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}