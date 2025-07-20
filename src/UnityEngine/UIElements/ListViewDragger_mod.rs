#[cfg(feature = "UnityEngine+UIElements+ListViewDragger")]
#[repr(C)]
#[derive(Debug)]
pub struct ListViewDragger {
    __cordl_parent: crate::UnityEngine::UIElements::DragEventsProcessor,
    pub m_LastDragPosition: crate::UnityEngine::UIElements::ListViewDragger_DragPosition,
    pub m_DragHoverBar: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub m_DragHoverItemMarker: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub m_DragHoverSiblingMarker: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub m_LeftIndentation: f32,
    pub m_SiblingBottom: f32,
    pub _dragAndDropController_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::ICollectionDragAndDropController,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+ListViewDragger")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::ListViewDragger {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "ListViewDragger";
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
#[cfg(feature = "UnityEngine+UIElements+ListViewDragger")]
impl std::ops::Deref for crate::UnityEngine::UIElements::ListViewDragger {
    type Target = crate::UnityEngine::UIElements::DragEventsProcessor;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ListViewDragger")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::ListViewDragger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ListViewDragger")]
impl crate::UnityEngine::UIElements::ListViewDragger {
    #[cfg(feature = "UnityEngine+UIElements+ListViewDragger+DragPosition")]
    pub type DragPosition = crate::UnityEngine::UIElements::ListViewDragger_DragPosition;
    pub fn ApplyDragAndDropUI(
        &mut self,
        dragPosition: crate::UnityEngine::UIElements::ListViewDragger_DragPosition,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::ListViewDragger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::UIElements::ListViewDragger_DragPosition),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ApplyDragAndDropUI")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::ListViewDragger as
                    quest_hook::libil2cpp::Type > ::class(), "ApplyDragAndDropUI", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (dragPosition))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CanStartDrag(
        &mut self,
        pointerPosition: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::ListViewDragger as quest_hook::libil2cpp::Type>::class()
            .find_method::<(crate::UnityEngine::Vector3), bool, 1usize>("CanStartDrag")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::ListViewDragger as
                    quest_hook::libil2cpp::Type > ::class(), "CanStartDrag", 1usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (pointerPosition))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearDragAndDropUI(
        &mut self,
        dragCancelled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::ListViewDragger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ClearDragAndDropUI")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::ListViewDragger as
                    quest_hook::libil2cpp::Type > ::class(), "ClearDragAndDropUI", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (dragCancelled))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetHoverBarTopPosition(
        &mut self,
        item: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ReusableCollectionItem,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::ListViewDragger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::ReusableCollectionItem,
                >),
                f32,
                1usize,
            >("GetHoverBarTopPosition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::ListViewDragger as
                    quest_hook::libil2cpp::Type > ::class(), "GetHoverBarTopPosition",
                    1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, (item))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPreviousAndNextItemsIgnoringDraggedItems(
        &mut self,
        insertAtIndex: i32,
        previousItemId: quest_hook::libil2cpp::ByRefMut<i32>,
        nextItemId: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::ListViewDragger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("GetPreviousAndNextItemsIgnoringDraggedItems")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::ListViewDragger as
                    quest_hook::libil2cpp::Type > ::class(),
                    "GetPreviousAndNextItemsIgnoringDraggedItems", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (insertAtIndex, previousItemId, nextItemId))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRecycledItem(
        &mut self,
        pointerPosition: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::ReusableCollectionItem>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::ListViewDragger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector3),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::ReusableCollectionItem,
                >,
                1usize,
            >("GetRecycledItem")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::ListViewDragger as
                    quest_hook::libil2cpp::Type > ::class(), "GetRecycledItem", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ReusableCollectionItem,
        > = unsafe { method.invoke_unchecked(self, (pointerPosition))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetVisualMode(
        &mut self,
        pointerPosition: crate::UnityEngine::Vector3,
        dragPosition: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ListViewDragger_DragPosition,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::DragVisualMode> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::ListViewDragger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Vector3,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::UIElements::ListViewDragger_DragPosition,
                    >,
                ),
                crate::UnityEngine::UIElements::DragVisualMode,
                2usize,
            >("GetVisualMode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::ListViewDragger as
                    quest_hook::libil2cpp::Type > ::class(), "GetVisualMode", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::DragVisualMode = unsafe {
            method.invoke_unchecked(self, (pointerPosition, dragPosition))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleAutoExpansion(
        &mut self,
        pointerPosition: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::ListViewDragger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector2),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleAutoExpansion")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::ListViewDragger as
                    quest_hook::libil2cpp::Type > ::class(), "HandleAutoExpansion",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pointerPosition))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleDragAndScroll(
        &mut self,
        pointerPosition: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::ListViewDragger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector2),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleDragAndScroll")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::ListViewDragger as
                    quest_hook::libil2cpp::Type > ::class(), "HandleDragAndScroll",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pointerPosition))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleSiblingInsertionAtAvailableDepthsAndChangeTargetIfNeeded(
        &mut self,
        dragPosition: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ListViewDragger_DragPosition,
        >,
        pointerPosition: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::ListViewDragger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::UIElements::ListViewDragger_DragPosition,
                    >,
                    crate::UnityEngine::Vector2,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("HandleSiblingInsertionAtAvailableDepthsAndChangeTargetIfNeeded")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::ListViewDragger as
                    quest_hook::libil2cpp::Type > ::class(),
                    "HandleSiblingInsertionAtAvailableDepthsAndChangeTargetIfNeeded",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (dragPosition, pointerPosition))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleTreePosition(
        &mut self,
        pointerPosition: crate::UnityEngine::Vector2,
        dragPosition: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ListViewDragger_DragPosition,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::ListViewDragger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Vector2,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::UIElements::ListViewDragger_DragPosition,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("HandleTreePosition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::ListViewDragger as
                    quest_hook::libil2cpp::Type > ::class(), "HandleTreePosition", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pointerPosition, dragPosition))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MakeDragAndDropArgs(
        &mut self,
        dragPosition: crate::UnityEngine::UIElements::ListViewDragger_DragPosition,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::DragAndDropArgs> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::ListViewDragger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::UIElements::ListViewDragger_DragPosition),
                crate::UnityEngine::UIElements::DragAndDropArgs,
                1usize,
            >("MakeDragAndDropArgs")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::ListViewDragger as
                    quest_hook::libil2cpp::Type > ::class(), "MakeDragAndDropArgs",
                    1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::DragAndDropArgs = unsafe {
            method.invoke_unchecked(self, (dragPosition))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        listView: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::BaseVerticalCollectionView,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (listView))?;
        Ok(__cordl_object.into())
    }
    pub fn OnDrop(
        &mut self,
        pointerPosition: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::ListViewDragger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector3),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnDrop")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::ListViewDragger as
                    quest_hook::libil2cpp::Type > ::class(), "OnDrop", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pointerPosition))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PlaceHoverBarAt(
        &mut self,
        top: f32,
        indentationPadding: f32,
        siblingBottom: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::ListViewDragger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32, f32, f32),
                quest_hook::libil2cpp::Void,
                3usize,
            >("PlaceHoverBarAt")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::ListViewDragger as
                    quest_hook::libil2cpp::Type > ::class(), "PlaceHoverBarAt", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (top, indentationPadding, siblingBottom))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PlaceHoverBarAtElement(
        &mut self,
        item: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ReusableCollectionItem,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::ListViewDragger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::ReusableCollectionItem,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("PlaceHoverBarAtElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::ListViewDragger as
                    quest_hook::libil2cpp::Type > ::class(), "PlaceHoverBarAtElement",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (item))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartDrag(
        &mut self,
        pointerPosition: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StartDragArgs> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::ListViewDragger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector3),
                crate::UnityEngine::UIElements::StartDragArgs,
                1usize,
            >("StartDrag")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::ListViewDragger as
                    quest_hook::libil2cpp::Type > ::class(), "StartDrag", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StartDragArgs = unsafe {
            method.invoke_unchecked(self, (pointerPosition))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetDragPosition(
        &mut self,
        pointerPosition: crate::UnityEngine::Vector2,
        dragPosition: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ListViewDragger_DragPosition,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::ListViewDragger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Vector2,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::UIElements::ListViewDragger_DragPosition,
                    >,
                ),
                bool,
                2usize,
            >("TryGetDragPosition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::ListViewDragger as
                    quest_hook::libil2cpp::Type > ::class(), "TryGetDragPosition", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (pointerPosition, dragPosition))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateDrag(
        &mut self,
        pointerPosition: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::ListViewDragger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector3),
                quest_hook::libil2cpp::Void,
                1usize,
            >("UpdateDrag")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::ListViewDragger as
                    quest_hook::libil2cpp::Type > ::class(), "UpdateDrag", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pointerPosition))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ApplyDragAndDropUI_g__GeometryChangedCallback_27_0(
        &mut self,
        e: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::GeometryChangedEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::ListViewDragger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::GeometryChangedEvent,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("<ApplyDragAndDropUI>g__GeometryChangedCallback|27_0")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::ListViewDragger as
                    quest_hook::libil2cpp::Type > ::class(),
                    "<ApplyDragAndDropUI>g__GeometryChangedCallback|27_0", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (e))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        listView: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::BaseVerticalCollectionView,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::ListViewDragger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::BaseVerticalCollectionView,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::ListViewDragger as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (listView))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_dragAndDropController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ICollectionDragAndDropController,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::ListViewDragger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::ICollectionDragAndDropController,
                >,
                0usize,
            >("get_dragAndDropController")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::ListViewDragger as
                    quest_hook::libil2cpp::Type > ::class(), "get_dragAndDropController",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ICollectionDragAndDropController,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_targetScrollView(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::ScrollView>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::ListViewDragger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::ScrollView>,
                0usize,
            >("get_targetScrollView")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::ListViewDragger as
                    quest_hook::libil2cpp::Type > ::class(), "get_targetScrollView",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ScrollView,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_targetView(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::BaseVerticalCollectionView,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::ListViewDragger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::BaseVerticalCollectionView,
                >,
                0usize,
            >("get_targetView")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::ListViewDragger as
                    quest_hook::libil2cpp::Type > ::class(), "get_targetView", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::BaseVerticalCollectionView,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_dragAndDropController(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ICollectionDragAndDropController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::ListViewDragger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::ICollectionDragAndDropController,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_dragAndDropController")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::ListViewDragger as
                    quest_hook::libil2cpp::Type > ::class(), "set_dragAndDropController",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+ListViewDragger")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::ListViewDragger {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+ListViewDragger+DragPosition")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ListViewDragger_DragPosition {
    pub insertAtIndex: i32,
    pub parentId: i32,
    pub childIndex: i32,
    pub recycledItem: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::ReusableCollectionItem,
    >,
    pub dropPosition: crate::UnityEngine::UIElements::DragAndDropPosition,
}
#[cfg(feature = "UnityEngine+UIElements+ListViewDragger+DragPosition")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::ListViewDragger_DragPosition {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "ListViewDragger/DragPosition";
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
#[cfg(feature = "UnityEngine+UIElements+ListViewDragger+DragPosition")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::ListViewDragger_DragPosition {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UIElements+ListViewDragger+DragPosition")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::ListViewDragger_DragPosition {
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
#[cfg(feature = "UnityEngine+UIElements+ListViewDragger+DragPosition")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::ListViewDragger_DragPosition {
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
#[cfg(feature = "UnityEngine+UIElements+ListViewDragger+DragPosition")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::ListViewDragger_DragPosition {
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
#[cfg(feature = "UnityEngine+UIElements+ListViewDragger+DragPosition")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::ListViewDragger_DragPosition {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ListViewDragger+DragPosition")]
impl crate::UnityEngine::UIElements::ListViewDragger_DragPosition {
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::ListViewDragger_DragPosition as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                bool,
                1usize,
            >("Equals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::ListViewDragger_DragPosition as
                    quest_hook::libil2cpp::Type > ::class(), "Equals", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (obj))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_ListViewDragger_DragPosition0(
        &mut self,
        other: crate::UnityEngine::UIElements::ListViewDragger_DragPosition,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::ListViewDragger_DragPosition as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::UIElements::ListViewDragger_DragPosition),
                bool,
                1usize,
            >("Equals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::ListViewDragger_DragPosition as
                    quest_hook::libil2cpp::Type > ::class(), "Equals", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::ListViewDragger_DragPosition as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("GetHashCode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::ListViewDragger_DragPosition as
                    quest_hook::libil2cpp::Type > ::class(), "GetHashCode", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+ListViewDragger+DragPosition")]
impl AsRef<
    crate::System::IEquatable_1<
        crate::UnityEngine::UIElements::ListViewDragger_DragPosition,
    >,
> for crate::UnityEngine::UIElements::ListViewDragger_DragPosition {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::UnityEngine::UIElements::ListViewDragger_DragPosition,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+ListViewDragger+DragPosition")]
impl AsMut<
    crate::System::IEquatable_1<
        crate::UnityEngine::UIElements::ListViewDragger_DragPosition,
    >,
> for crate::UnityEngine::UIElements::ListViewDragger_DragPosition {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::UIElements::ListViewDragger_DragPosition,
    > {
        todo!()
    }
}
