#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiColumnCollectionHeader {
    __cordl_parent: crate::UnityEngine::UIElements::VisualElement,
    pub m_SortingEnabled: bool,
    pub m_SortedColumns: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::UIElements::SortColumnDescription,
            >,
        >,
    >,
    pub m_SortDescriptions: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::SortColumnDescriptions,
    >,
    pub m_OldSortedColumnStates: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_SortedColumnState,
        >,
    >,
    pub m_SortingUpdatesTemporarilyDisabled: bool,
    pub m_ViewState: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_ViewState,
    >,
    pub m_ApplyingViewState: bool,
    pub m_DoLayoutScheduled: bool,
    pub _columnDataMap_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Column>,
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_ColumnData,
            >,
        >,
    >,
    pub _columnLayout_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::ColumnLayout,
    >,
    pub _columnContainer_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub _resizeHandleContainer_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub _columns_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::Columns,
    >,
    pub columnResized: quest_hook::libil2cpp::Gc<crate::System::Action_2<i32, f32>>,
    pub columnSortingChanged: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub contextMenuPopulateEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::UIElements::ContextualMenuPopulateEvent,
            >,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Column>,
        >,
    >,
    pub viewDataRestored: quest_hook::libil2cpp::Gc<crate::System::Action>,
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.Internal";
    const CLASS_NAME: &'static str = "MultiColumnCollectionHeader";
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
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader {
    type Target = crate::UnityEngine::UIElements::VisualElement;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader")]
impl crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader {
    #[cfg(
        feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ColumnData"
    )]
    pub type ColumnData = crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_ColumnData;
    #[cfg(
        feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+SortedColumnState"
    )]
    pub type SortedColumnState = crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_SortedColumnState;
    #[cfg(
        feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ViewState"
    )]
    pub type ViewState = crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_ViewState;
    pub fn ApplyColumnSorting(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ApplyColumnSorting")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ApplyColumnSorting", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Dispose", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn DoLayout(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("DoLayout")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DoLayout", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        columns: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Columns>,
        sortDescriptions: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::SortColumnDescriptions,
        >,
        sortedColumns: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::SortColumnDescription,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (columns, sortDescriptions, sortedColumns))?;
        Ok(__cordl_object.into())
    }
    pub fn OnColumnAdded_Column1(
        &mut self,
        column: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Column>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Column>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnColumnAdded")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnColumnAdded", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (column))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnColumnAdded_i32_0(
        &mut self,
        column: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Column>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Column>, i32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("OnColumnAdded")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnColumnAdded", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (column, index))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnColumnChanged(
        &mut self,
        column: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Column>,
        _cordl_type: crate::UnityEngine::UIElements::ColumnDataType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Column>,
                    crate::UnityEngine::UIElements::ColumnDataType,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("OnColumnChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnColumnChanged", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (column, _cordl_type))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnColumnClicked(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnColumnClicked")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnColumnClicked", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (evt))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnColumnControlGeometryChanged(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::GeometryChangedEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::GeometryChangedEvent,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnColumnControlGeometryChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnColumnControlGeometryChanged", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (evt))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnColumnRemoved(
        &mut self,
        column: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Column>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Column>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnColumnRemoved")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnColumnRemoved", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (column))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnColumnReordered(
        &mut self,
        column: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Column>,
        from: i32,
        to: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Column>,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("OnColumnReordered")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnColumnReordered", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (column, from, to))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnColumnResized(
        &mut self,
        column: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Column>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Column>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnColumnResized")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnColumnResized", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (column))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnContextualMenuManipulator(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ContextualMenuPopulateEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::ContextualMenuPopulateEvent,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnContextualMenuManipulator")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnContextualMenuManipulator", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (evt))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnGeometryChanged(
        &mut self,
        e: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::GeometryChangedEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::GeometryChangedEvent,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnGeometryChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnGeometryChanged", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (e))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnMoveManipulatorActivated(
        &mut self,
        mover: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Internal::ColumnMover,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::Internal::ColumnMover,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnMoveManipulatorActivated")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnMoveManipulatorActivated", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (mover))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnViewDataReady(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnViewDataReady")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnViewDataReady", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn RaiseColumnResized(
        &mut self,
        columnIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RaiseColumnResized")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RaiseColumnResized", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (columnIndex))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RaiseColumnSortingChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("RaiseColumnSortingChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RaiseColumnSortingChanged", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResizeToFit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ResizeToFit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ResizeToFit", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn SaveViewState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("SaveViewState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SaveViewState", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleDoLayout(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ScheduleDoLayout")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ScheduleDoLayout", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScrollHorizontally(
        &mut self,
        horizontalOffset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ScrollHorizontally")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ScrollHorizontally", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (horizontalOffset))
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateColumnControls(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("UpdateColumnControls")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UpdateColumnControls", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateSortColumnDescriptionsOnClick(
        &mut self,
        column: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Column>,
        modifiers: crate::UnityEngine::EventModifiers,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Column>,
                    crate::UnityEngine::EventModifiers,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("UpdateSortColumnDescriptionsOnClick")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UpdateSortColumnDescriptionsOnClick", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (column, modifiers))
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateSortedColumns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("UpdateSortedColumns")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UpdateSortedColumns", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateSortingStatus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("UpdateSortingStatus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UpdateSortingStatus", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn _OnContextualMenuManipulator_b__65_0(
        &mut self,
        a: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DropdownMenuAction>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::DropdownMenuAction,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("<OnContextualMenuManipulator>b__65_0")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "<OnContextualMenuManipulator>b__65_0", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        columns: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Columns>,
        sortDescriptions: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::SortColumnDescriptions,
        >,
        sortedColumns: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::SortColumnDescription,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Columns>,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::SortColumnDescriptions,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::SortColumnDescription,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (columns, sortDescriptions, sortedColumns))
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_columnResized(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_2<i32, f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action_2<i32, f32>>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_columnResized")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_columnResized", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_columnSortingChanged(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_columnSortingChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_columnSortingChanged", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_contextMenuPopulateEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::ContextualMenuPopulateEvent,
                >,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Column>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_2<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::ContextualMenuPopulateEvent,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Column>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_contextMenuPopulateEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_contextMenuPopulateEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_viewDataRestored(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_viewDataRestored")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_viewDataRestored", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_columnContainer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                0usize,
            >("get_columnContainer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_columnContainer", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_columnDataMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Column>,
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_ColumnData,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::Dictionary_2<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::Column,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_ColumnData,
                        >,
                    >,
                >,
                0usize,
            >("get_columnDataMap")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_columnDataMap", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Column>,
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_ColumnData,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_columnLayout(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::ColumnLayout>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::ColumnLayout>,
                0usize,
            >("get_columnLayout")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_columnLayout", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ColumnLayout,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_columns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Columns>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Columns>,
                0usize,
            >("get_columns")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_columns", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Columns,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_isApplyingViewState(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isApplyingViewState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_isApplyingViewState", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_resizeHandleContainer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                0usize,
            >("get_resizeHandleContainer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_resizeHandleContainer", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_sortDescriptions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::SortColumnDescriptions>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::SortColumnDescriptions,
                >,
                0usize,
            >("get_sortDescriptions")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_sortDescriptions", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::SortColumnDescriptions,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_sortedColumns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::SortColumnDescription,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerable_1<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::SortColumnDescription,
                        >,
                    >,
                >,
                0usize,
            >("get_sortedColumns")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_sortedColumns", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::SortColumnDescription,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_sortingEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_sortingEnabled")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_sortingEnabled", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn remove_columnResized(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_2<i32, f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action_2<i32, f32>>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_columnResized")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "remove_columnResized", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_columnSortingChanged(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_columnSortingChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "remove_columnSortingChanged", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_contextMenuPopulateEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::ContextualMenuPopulateEvent,
                >,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Column>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_2<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::ContextualMenuPopulateEvent,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Column>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_contextMenuPopulateEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "remove_contextMenuPopulateEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_viewDataRestored(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_viewDataRestored")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "remove_viewDataRestored", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_sortDescriptions(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::SortColumnDescriptions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::SortColumnDescriptions,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_sortDescriptions")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_sortDescriptions", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_sortingEnabled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_sortingEnabled")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_sortingEnabled", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader")]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader")]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ColumnData"
)]
#[repr(C)]
#[derive(Debug)]
pub struct MultiColumnCollectionHeader_ColumnData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _control_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumn,
    >,
    pub _resizeHandle_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumnResizeHandle,
    >,
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ColumnData"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_ColumnData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.Internal";
    const CLASS_NAME: &'static str = "MultiColumnCollectionHeader/ColumnData";
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
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ColumnData"
)]
impl std::ops::Deref
for crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_ColumnData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ColumnData"
)]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_ColumnData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ColumnData"
)]
impl crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_ColumnData {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_control(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumn,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumn,
                >,
                0usize,
            >("get_control")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_control", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumn,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_resizeHandle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumnResizeHandle,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumnResizeHandle,
                >,
                0usize,
            >("get_resizeHandle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_resizeHandle", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumnResizeHandle,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn set_control(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumn,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumn,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_control")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_control", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_resizeHandle(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumnResizeHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumnResizeHandle,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_resizeHandle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_resizeHandle", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ColumnData"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_ColumnData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+SortedColumnState"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MultiColumnCollectionHeader_SortedColumnState {
    pub columnDesc: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::SortColumnDescription,
    >,
    pub direction: crate::UnityEngine::UIElements::SortDirection,
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+SortedColumnState"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_SortedColumnState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.Internal";
    const CLASS_NAME: &'static str = "MultiColumnCollectionHeader/SortedColumnState";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+SortedColumnState"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_SortedColumnState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+SortedColumnState"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_SortedColumnState {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+SortedColumnState"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_SortedColumnState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+SortedColumnState"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_SortedColumnState {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+SortedColumnState"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_SortedColumnState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+SortedColumnState"
)]
impl crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_SortedColumnState {
    pub fn _ctor(
        &mut self,
        desc: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::SortColumnDescription,
        >,
        dir: crate::UnityEngine::UIElements::SortDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::SortColumnDescription,
                    >,
                    crate::UnityEngine::UIElements::SortDirection,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (desc, dir))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ViewState")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiColumnCollectionHeader_ViewState {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_HasPersistedData: bool,
    pub m_SortDescriptions: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::UIElements::SortColumnDescription,
            >,
        >,
    >,
    pub m_OrderedColumnStates: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::Internal::ViewState_MultiColumnCollectionHeader_ColumnState,
        >,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ViewState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_ViewState {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.Internal";
    const CLASS_NAME: &'static str = "MultiColumnCollectionHeader/ViewState";
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
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ViewState")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_ViewState {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ViewState")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_ViewState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ViewState")]
impl crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_ViewState {
    #[cfg(
        feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ViewState+ColumnState"
    )]
    pub type ColumnState = crate::UnityEngine::UIElements::Internal::ViewState_MultiColumnCollectionHeader_ColumnState;
    pub fn Apply(
        &mut self,
        header: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Apply")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Apply", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (header))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Save(
        &mut self,
        header: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Save")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Save", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (header))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ViewState")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_ViewState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ViewState+ColumnState"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ViewState_MultiColumnCollectionHeader_ColumnState {
    pub index: i32,
    pub name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub actualWidth: f32,
    pub width: crate::UnityEngine::UIElements::Length,
    pub visible: bool,
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ViewState+ColumnState"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::Internal::ViewState_MultiColumnCollectionHeader_ColumnState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.Internal";
    const CLASS_NAME: &'static str = "MultiColumnCollectionHeader/ViewState/ColumnState";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ViewState+ColumnState"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::Internal::ViewState_MultiColumnCollectionHeader_ColumnState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ViewState+ColumnState"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::Internal::ViewState_MultiColumnCollectionHeader_ColumnState {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ViewState+ColumnState"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::Internal::ViewState_MultiColumnCollectionHeader_ColumnState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ViewState+ColumnState"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::Internal::ViewState_MultiColumnCollectionHeader_ColumnState {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ViewState+ColumnState"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::Internal::ViewState_MultiColumnCollectionHeader_ColumnState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ViewState+ColumnState"
)]
impl crate::UnityEngine::UIElements::Internal::ViewState_MultiColumnCollectionHeader_ColumnState {}
