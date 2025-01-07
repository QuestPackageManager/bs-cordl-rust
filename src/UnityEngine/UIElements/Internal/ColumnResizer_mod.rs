#[cfg(feature = "UnityEngine+UIElements+Internal+ColumnResizer")]
#[repr(C)]
#[derive(Debug)]
pub struct ColumnResizer {
    __cordl_parent: crate::UnityEngine::UIElements::PointerManipulator,
    pub m_Start: crate::UnityEngine::Vector2,
    pub m_Active: bool,
    pub m_Resizing: bool,
    pub m_Header: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader,
    >,
    pub m_Column: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Column>,
    pub m_PreviewElement: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub _columnLayout_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::ColumnLayout,
    >,
    pub _preview_k__BackingField: bool,
}
#[cfg(feature = "UnityEngine+UIElements+Internal+ColumnResizer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::Internal::ColumnResizer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.Internal";
    const CLASS_NAME: &'static str = "ColumnResizer";
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
#[cfg(feature = "UnityEngine+UIElements+Internal+ColumnResizer")]
impl std::ops::Deref for crate::UnityEngine::UIElements::Internal::ColumnResizer {
    type Target = crate::UnityEngine::UIElements::PointerManipulator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+ColumnResizer")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::Internal::ColumnResizer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+ColumnResizer")]
impl crate::UnityEngine::UIElements::Internal::ColumnResizer {
    pub fn BeginDragResize(
        &mut self,
        pos: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BeginDragResize", (pos))?;
        Ok(__cordl_ret.into())
    }
    pub fn DragResize(
        &mut self,
        pos: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DragResize", (pos))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndDragResize(
        &mut self,
        pos: f32,
        cancelled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndDragResize", (pos, cancelled))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        column: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Column>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (column))?;
        Ok(__cordl_object.into())
    }
    pub fn OnKeyDown(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::KeyDownEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnKeyDown", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerDown(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::PointerDownEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerDown", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerMove(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::PointerMoveEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerMove", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerUp(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::PointerUpEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerUp", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterCallbacksOnTarget(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterCallbacksOnTarget", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterCallbacksFromTarget(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterCallbacksFromTarget", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdatePreviewPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdatePreviewPosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        column: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Column>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (column))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_columnLayout(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::ColumnLayout>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ColumnLayout,
        > = __cordl_object.invoke("get_columnLayout", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_preview(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_preview", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_columnLayout(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::ColumnLayout>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_columnLayout", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_preview(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_preview", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+ColumnResizer")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Internal::ColumnResizer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
