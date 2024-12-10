#[cfg(feature = "UnityEngine+UIElements+DragAndDropArgs")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DragAndDropArgs {
    pub _target_k__BackingField: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _insertAtIndex_k__BackingField: i32,
    pub _parentId_k__BackingField: i32,
    pub _childIndex_k__BackingField: i32,
    pub _dragAndDropPosition_k__BackingField: crate::UnityEngine::UIElements::DragAndDropPosition,
    pub _dragAndDropData_k__BackingField: *mut crate::UnityEngine::UIElements::DragAndDropData,
}
#[cfg(feature = "UnityEngine+UIElements+DragAndDropArgs")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::DragAndDropArgs =>
    "UnityEngine.UIElements"."DragAndDropArgs"
);
#[cfg(feature = "UnityEngine+UIElements+DragAndDropArgs")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::DragAndDropArgs {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DragAndDropArgs")]
impl crate::UnityEngine::UIElements::DragAndDropArgs {
    pub fn get_childIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_childIndex",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_dragAndDropData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DragAndDropData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::DragAndDropData,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_dragAndDropData",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_dragAndDropPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::DragAndDropPosition,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::DragAndDropPosition = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_dragAndDropPosition",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_insertAtIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_insertAtIndex",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_parentId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_parentId",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_childIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_childIndex",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_dragAndDropData(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DragAndDropData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_dragAndDropData",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_dragAndDropPosition(
        &mut self,
        value: crate::UnityEngine::UIElements::DragAndDropPosition,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_dragAndDropPosition",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_insertAtIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_insertAtIndex",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_parentId(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_parentId",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_target(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_target",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+DragAndDropArgs")]
impl AsRef<crate::UnityEngine::UIElements::IListDragAndDropArgs>
for crate::UnityEngine::UIElements::DragAndDropArgs {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::IListDragAndDropArgs {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+DragAndDropArgs")]
impl AsMut<crate::UnityEngine::UIElements::IListDragAndDropArgs>
for crate::UnityEngine::UIElements::DragAndDropArgs {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UIElements::IListDragAndDropArgs {
        todo!()
    }
}
