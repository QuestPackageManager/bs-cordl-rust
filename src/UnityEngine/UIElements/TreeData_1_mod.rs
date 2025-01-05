#[cfg(feature = "UnityEngine+UIElements+TreeData_1")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TreeData_1<T: quest_hook::libil2cpp::Type> {
    pub m_RootItemIds: quest_hook::libil2cpp::Gc<i32>,
    pub m_Tree: quest_hook::libil2cpp::Gc<
        i32,
        crate::UnityEngine::UIElements::TreeViewItemData_1<T>,
    >,
    pub m_ParentIds: quest_hook::libil2cpp::Gc<i32, i32>,
    pub m_ChildrenIds: quest_hook::libil2cpp::Gc<i32, quest_hook::libil2cpp::Gc<i32>>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "UnityEngine+UIElements+TreeData_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::TreeData_1 < T > =>
    "UnityEngine.UIElements"."TreeData`1<T>" < T >
);
#[cfg(feature = "UnityEngine+UIElements+TreeData_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::TreeData_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TreeData_1")]
impl<T: quest_hook::libil2cpp::Type> crate::UnityEngine::UIElements::TreeData_1<T> {
    pub fn AddItemToParent(
        &mut self,
        item: crate::UnityEngine::UIElements::TreeViewItemData_1<T>,
        parentId: i32,
        childIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddItemToParent",
            (item, parentId, childIndex),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildTree(
        &mut self,
        items: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::TreeViewItemData_1<T>,
        >,
        isRoot: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "BuildTree",
            (items, isRoot),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDataForId(
        &mut self,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::TreeViewItemData_1<T>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::UIElements::TreeViewItemData_1<T> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetDataForId",
            (id),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParentId(&mut self, id: i32) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetParentId",
            (id),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn HasAncestor(
        &mut self,
        childId: i32,
        ancestorId: i32,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "HasAncestor",
            (childId, ancestorId),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Move(
        &mut self,
        id: i32,
        newParentId: i32,
        childIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Move",
            (id, newParentId, childIndex),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn RefreshTree(
        &mut self,
        rootItems: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::TreeViewItemData_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "RefreshTree",
            (rootItems),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveFromParent(
        &mut self,
        id: i32,
        parentId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "RemoveFromParent",
            (id, parentId),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateParentTree(
        &mut self,
        current: crate::UnityEngine::UIElements::TreeViewItemData_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "UpdateParentTree",
            (current),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        rootItems: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::TreeViewItemData_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (rootItems),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rootItemIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<i32>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<i32> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_rootItemIds",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
