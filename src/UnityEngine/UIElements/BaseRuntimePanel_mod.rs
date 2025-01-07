#[cfg(feature = "UnityEngine+UIElements+BaseRuntimePanel")]
#[repr(C)]
#[derive(Debug)]
pub struct BaseRuntimePanel {
    __cordl_parent: crate::UnityEngine::UIElements::Panel,
    pub m_SelectableGameObject: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::GameObject,
    >,
    pub m_RuntimePanelCreationIndex: i32,
    pub m_SortingPriority: f32,
    pub resolvedSortingIndex: i32,
    pub destroyed: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub m_StandardWorldSpaceShader: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Shader,
    >,
    pub m_DrawToCameras: bool,
    pub targetTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    pub panelToWorld: crate::UnityEngine::Matrix4x4,
    pub _targetDisplay_k__BackingField: i32,
    pub m_ScreenToPanelSpace: quest_hook::libil2cpp::Gc<
        crate::System::Func_2<crate::UnityEngine::Vector2, crate::UnityEngine::Vector2>,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+BaseRuntimePanel")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::BaseRuntimePanel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "BaseRuntimePanel";
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
    pub fn AssignPanelToComponents(
        &mut self,
        panel: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::BaseRuntimePanel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AssignPanelToComponents", (panel))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn New(
        ownerObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
        dispatcher: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventDispatcher,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ownerObject, dispatcher))?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn Repaint(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Repaint", (e))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        ownerObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
        dispatcher: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventDispatcher,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ownerObject, dispatcher))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_destroyed(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_destroyed", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn getScreenRenderingHeight(display: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("getScreenRenderingHeight", (display))?;
        Ok(__cordl_ret.into())
    }
    pub fn getScreenRenderingWidth(display: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("getScreenRenderingWidth", (display))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_drawToCameras(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_drawToCameras", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_screenRenderingHeight(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_screenRenderingHeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_screenRenderingWidth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_screenRenderingWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_screenToPanelSpace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                crate::UnityEngine::Vector2,
                crate::UnityEngine::Vector2,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                crate::UnityEngine::Vector2,
                crate::UnityEngine::Vector2,
            >,
        > = __cordl_object.invoke("get_screenToPanelSpace", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectableGameObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = __cordl_object
            .invoke("get_selectableGameObject", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sortingPriority(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_sortingPriority", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_standardWorldSpaceShader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader> = __cordl_object
            .invoke("get_standardWorldSpaceShader", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_targetDisplay(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_targetDisplay", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_destroyed(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_destroyed", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn set_screenToPanelSpace(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                crate::UnityEngine::Vector2,
                crate::UnityEngine::Vector2,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_screenToPanelSpace", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_selectableGameObject(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_selectableGameObject", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
