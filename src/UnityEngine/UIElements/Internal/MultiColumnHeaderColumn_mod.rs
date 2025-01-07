#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnHeaderColumn")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiColumnHeaderColumn {
    __cordl_parent: crate::UnityEngine::UIElements::VisualElement,
    pub m_ContentContainer: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub m_Content: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub m_SortIndicatorContainer: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumnSortIndicator,
    >,
    pub m_ScheduledHeaderTemplateUpdate: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::IVisualElementScheduledItem,
    >,
    pub _clickable_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::Clickable,
    >,
    pub _mover_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::Internal::ColumnMover,
    >,
    pub _column_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::Column,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnHeaderColumn")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumn {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.Internal";
    const CLASS_NAME: &'static str = "MultiColumnHeaderColumn";
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
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnHeaderColumn")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumn {
    type Target = crate::UnityEngine::UIElements::VisualElement;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnHeaderColumn")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnHeaderColumn")]
impl crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumn {
    pub fn BindHeaderContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BindHeaderContent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateDefaultHeaderContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("CreateDefaultHeaderContent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DefaultBindHeaderContent(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DefaultBindHeaderContent", (ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyHeaderContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroyHeaderContent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitManipulators(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitManipulators", ())?;
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
    pub fn UnbindHeaderContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnbindHeaderContent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateDataFromColumn(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateDataFromColumn", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateGeometryFromColumn(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateGeometryFromColumn", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateHeaderTemplate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateHeaderTemplate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _InitManipulators_b__46_0(
        &mut self,
        mv: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Internal::ColumnMover,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<InitManipulators>b__46_0", (mv))?;
        Ok(__cordl_ret.into())
    }
    pub fn __ctor_b__45_0(
        &mut self,
        c: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Column>,
        role: crate::UnityEngine::UIElements::ColumnDataType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<.ctor>b__45_0", (c, role))?;
        Ok(__cordl_ret.into())
    }
    pub fn __ctor_b__45_1(
        &mut self,
        c: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Column>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<.ctor>b__45_1", (c))?;
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
    pub fn get_clickable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Clickable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Clickable,
        > = __cordl_object.invoke("get_clickable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_column(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Column>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Column,
        > = __cordl_object.invoke("get_column", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_content(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("get_content", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isContentBound(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isContentBound", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_mover(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Internal::ColumnMover>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Internal::ColumnMover,
        > = __cordl_object.invoke("get_mover", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_clickable(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Clickable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_clickable", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_column(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Column>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_column", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_content(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_content", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isContentBound(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isContentBound", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_mover(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Internal::ColumnMover,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_mover", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_sortOrderLabel(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sortOrderLabel", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnHeaderColumn")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumn {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
