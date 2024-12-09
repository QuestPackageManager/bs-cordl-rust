#[cfg(feature = "UnityEngine+UIElements+ColumnLayout")]
#[repr(C)]
#[derive(Debug)]
pub struct ColumnLayout {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_StretchableColumns: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::Column,
    >,
    pub m_FixedColumns: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::Column,
    >,
    pub m_RelativeWidthColumns: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::Column,
    >,
    pub m_MixedWidthColumns: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::Column,
    >,
    pub m_Columns: *mut crate::UnityEngine::UIElements::Columns,
    pub m_ColumnsWidth: f32,
    pub m_ColumnsWidthDirty: bool,
    pub m_MaxColumnsWidth: f32,
    pub m_MinColumnsWidth: f32,
    pub m_IsDirty: bool,
    pub m_PreviousWidth: f32,
    pub m_LayoutWidth: f32,
    pub m_DragResizeInPreviewMode: bool,
    pub m_DragResizing: bool,
    pub m_DragStartPos: f32,
    pub m_DragLastPos: f32,
    pub m_DragInitialColumnWidth: f32,
    pub m_DragStretchableColumns: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::Column,
    >,
    pub m_DragRelativeColumns: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::Column,
    >,
    pub m_DragFixedColumns: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::Column,
    >,
    pub m_PreviewDesiredWidths: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::UnityEngine::UIElements::Column,
        f32,
    >,
    pub layoutRequested: *mut crate::System::Action,
}
#[cfg(feature = "UnityEngine+UIElements+ColumnLayout")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::ColumnLayout =>
    "UnityEngine.UIElements"."ColumnLayout"
);
#[cfg(feature = "UnityEngine+UIElements+ColumnLayout")]
impl std::ops::Deref for crate::UnityEngine::UIElements::ColumnLayout {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ColumnLayout")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::ColumnLayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ColumnLayout")]
impl crate::UnityEngine::UIElements::ColumnLayout {
    #[cfg(feature = "UnityEngine+UIElements+ColumnLayout+__c__DisplayClass53_0")]
    pub type __c__DisplayClass53_0 = crate::UnityEngine::UIElements::ColumnLayout___c__DisplayClass53_0;
    #[cfg(feature = "UnityEngine+UIElements+ColumnLayout+__c__DisplayClass54_0")]
    pub type __c__DisplayClass54_0 = crate::UnityEngine::UIElements::ColumnLayout___c__DisplayClass54_0;
    pub fn BeginDragResize(
        &mut self,
        column: *mut crate::UnityEngine::UIElements::Column,
        pos: f32,
        previewMode: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BeginDragResize", (column, pos, previewMode))?;
        Ok(__cordl_ret)
    }
    pub fn ClearCache(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearCache", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dirty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dirty", ())?;
        Ok(__cordl_ret)
    }
    pub fn DistributeExcess(
        &mut self,
        stretchableColumns: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::Column,
        >,
        fixedColumns: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::Column,
        >,
        relativeWidthColumns: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::Column,
        >,
        delta: quest_hook::libil2cpp::ByRefMut<f32>,
        resizeToFit: bool,
        dragResize: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DistributeExcess",
                (
                    stretchableColumns,
                    fixedColumns,
                    relativeWidthColumns,
                    delta,
                    resizeToFit,
                    dragResize,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn DistributeOverflow(
        &mut self,
        stretchableColumns: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::Column,
        >,
        fixedColumns: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::Column,
        >,
        relativeWidthColumns: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::Column,
        >,
        delta: quest_hook::libil2cpp::ByRefMut<f32>,
        resizeToFit: bool,
        dragResize: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DistributeOverflow",
                (
                    stretchableColumns,
                    fixedColumns,
                    relativeWidthColumns,
                    delta,
                    resizeToFit,
                    dragResize,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn DoLayout(
        &mut self,
        width: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoLayout", (width))?;
        Ok(__cordl_ret)
    }
    pub fn DragResize(
        &mut self,
        column: *mut crate::UnityEngine::UIElements::Column,
        pos: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DragResize", (column, pos))?;
        Ok(__cordl_ret)
    }
    pub fn EndDragResize(
        &mut self,
        column: *mut crate::UnityEngine::UIElements::Column,
        cancelled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndDragResize", (column, cancelled))?;
        Ok(__cordl_ret)
    }
    pub fn GetDesiredPosition(
        &mut self,
        column: *mut crate::UnityEngine::UIElements::Column,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetDesiredPosition", (column))?;
        Ok(__cordl_ret)
    }
    pub fn GetDesiredWidth(
        &mut self,
        c: *mut crate::UnityEngine::UIElements::Column,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetDesiredWidth", (c))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        columns: *mut crate::UnityEngine::UIElements::Columns,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (columns))?;
        Ok(__cordl_object)
    }
    pub fn OnColumnAdded(
        &mut self,
        column: *mut crate::UnityEngine::UIElements::Column,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnColumnAdded", (column, index))?;
        Ok(__cordl_ret)
    }
    pub fn OnColumnChanged(
        &mut self,
        column: *mut crate::UnityEngine::UIElements::Column,
        _cordl_type: crate::UnityEngine::UIElements::ColumnDataType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnColumnChanged", (column, _cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn OnColumnRemoved(
        &mut self,
        column: *mut crate::UnityEngine::UIElements::Column,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnColumnRemoved", (column))?;
        Ok(__cordl_ret)
    }
    pub fn OnColumnReordered(
        &mut self,
        column: *mut crate::UnityEngine::UIElements::Column,
        from: i32,
        to: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnColumnReordered", (column, from, to))?;
        Ok(__cordl_ret)
    }
    pub fn OnColumnResized(
        &mut self,
        column: *mut crate::UnityEngine::UIElements::Column,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnColumnResized", (column))?;
        Ok(__cordl_ret)
    }
    pub fn RecomputeToDesiredWidth_Column1(
        &mut self,
        column: *mut crate::UnityEngine::UIElements::Column,
        distributedDelta: f32,
        setDesiredWidthOnly: bool,
        distributeOverflow: bool,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke(
                "RecomputeToDesiredWidth",
                (column, distributedDelta, setDesiredWidthOnly, distributeOverflow),
            )?;
        Ok(__cordl_ret)
    }
    pub fn RecomputeToDesiredWidth_List_1_0(
        &mut self,
        columns: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::Column,
        >,
        distributedDelta: f32,
        setDesiredWidthOnly: bool,
        distributeOverflow: bool,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke(
                "RecomputeToDesiredWidth",
                (columns, distributedDelta, setDesiredWidthOnly, distributeOverflow),
            )?;
        Ok(__cordl_ret)
    }
    pub fn RecomputeToMaxWidth(
        &mut self,
        columns: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::Column,
        >,
        distributedDelta: f32,
        setDesiredWidthOnly: bool,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke(
                "RecomputeToMaxWidth",
                (columns, distributedDelta, setDesiredWidthOnly),
            )?;
        Ok(__cordl_ret)
    }
    pub fn RecomputeToMaxWidthProportionally(
        &mut self,
        columns: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::Column,
        >,
        distributedDelta: f32,
        setDesiredWidthOnly: bool,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke(
                "RecomputeToMaxWidthProportionally",
                (columns, distributedDelta, setDesiredWidthOnly),
            )?;
        Ok(__cordl_ret)
    }
    pub fn RecomputeToMinWidth(
        &mut self,
        columns: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::Column,
        >,
        distributedDelta: f32,
        setDesiredWidthOnly: bool,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke(
                "RecomputeToMinWidth",
                (columns, distributedDelta, setDesiredWidthOnly),
            )?;
        Ok(__cordl_ret)
    }
    pub fn RecomputeToMinWidthProportionally(
        &mut self,
        columns: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::Column,
        >,
        distributedDelta: f32,
        setDesiredWidthOnly: bool,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke(
                "RecomputeToMinWidthProportionally",
                (columns, distributedDelta, setDesiredWidthOnly),
            )?;
        Ok(__cordl_ret)
    }
    pub fn RequiresLayoutUpdate(
        &mut self,
        _cordl_type: crate::UnityEngine::UIElements::ColumnDataType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("RequiresLayoutUpdate", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn ResizeColumn(
        &mut self,
        column: *mut crate::UnityEngine::UIElements::Column,
        width: f32,
        setDesiredWidthOnly: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResizeColumn", (column, width, setDesiredWidthOnly))?;
        Ok(__cordl_ret)
    }
    pub fn ResizeToFit(
        &mut self,
        width: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResizeToFit", (width))?;
        Ok(__cordl_ret)
    }
    pub fn StretchResizeColumns(
        &mut self,
        stretchableColumns: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::Column,
        >,
        fixedColumns: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::Column,
        >,
        relativeWidthColumns: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::Column,
        >,
        delta: quest_hook::libil2cpp::ByRefMut<f32>,
        resizeToFit: bool,
        dragResize: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "StretchResizeColumns",
                (
                    stretchableColumns,
                    fixedColumns,
                    relativeWidthColumns,
                    delta,
                    resizeToFit,
                    dragResize,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn UpdateCache(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateCache", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateMinAndMaxColumnsWidth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateMinAndMaxColumnsWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn _DoLayout_b__49_0(
        &mut self,
        c1: *mut crate::UnityEngine::UIElements::Column,
        c2: *mut crate::UnityEngine::UIElements::Column,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("<DoLayout>b__49_0", (c1, c2))?;
        Ok(__cordl_ret)
    }
    pub fn _DoLayout_b__49_1(
        &mut self,
        c1: *mut crate::UnityEngine::UIElements::Column,
        c2: *mut crate::UnityEngine::UIElements::Column,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("<DoLayout>b__49_1", (c1, c2))?;
        Ok(__cordl_ret)
    }
    pub fn _RecomputeToMaxWidthProportionally_b__53_0(
        &mut self,
        c1: *mut crate::UnityEngine::UIElements::Column,
        c2: *mut crate::UnityEngine::UIElements::Column,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("<RecomputeToMaxWidthProportionally>b__53_0", (c1, c2))?;
        Ok(__cordl_ret)
    }
    pub fn _RecomputeToMinWidthProportionally_b__54_0(
        &mut self,
        c1: *mut crate::UnityEngine::UIElements::Column,
        c2: *mut crate::UnityEngine::UIElements::Column,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("<RecomputeToMinWidthProportionally>b__54_0", (c1, c2))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        columns: *mut crate::UnityEngine::UIElements::Columns,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (columns))?;
        Ok(__cordl_ret)
    }
    pub fn add_layoutRequested(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_layoutRequested", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_columns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UIElements::Columns> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::Columns = __cordl_object
            .invoke("get_columns", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_columnsWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_columnsWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_hasRelativeWidthColumns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_hasRelativeWidthColumns", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_hasStretchableColumns(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasStretchableColumns", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_layoutWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_layoutWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_maxColumnsWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_maxColumnsWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_minColumnsWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_minColumnsWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_layoutRequested(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_layoutRequested", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+ColumnLayout")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::ColumnLayout {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
