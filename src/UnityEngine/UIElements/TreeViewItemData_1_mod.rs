#[cfg(feature = "UnityEngine+UIElements+TreeViewItemData_1")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TreeViewItemData_1<T: quest_hook::libil2cpp::Type> {
    pub _id_k__BackingField: i32,
    pub m_Data: T,
    pub m_Children: *mut crate::System::Collections::Generic::IList_1<
        crate::UnityEngine::UIElements::TreeViewItemData_1<T>,
    >,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "UnityEngine+UIElements+TreeViewItemData_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::TreeViewItemData_1 < T
    > => "UnityEngine.UIElements"."TreeViewItemData`1<T>" < T >
);
#[cfg(feature = "UnityEngine+UIElements+TreeViewItemData_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::TreeViewItemData_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TreeViewItemData_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::TreeViewItemData_1<T> {
    pub fn InsertChild(
        &mut self,
        child: crate::UnityEngine::UIElements::TreeViewItemData_1<T>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "InsertChild",
            (child, index),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_data(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_data",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ReplaceChild(
        &mut self,
        newChild: crate::UnityEngine::UIElements::TreeViewItemData_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ReplaceChild",
            (newChild),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_id(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_id",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn RemoveChild(
        &mut self,
        childId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "RemoveChild",
            (childId),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetChildIndex(&mut self, itemId: i32) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetChildIndex",
            (itemId),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_hasChildren(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_hasChildren",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_children(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            crate::UnityEngine::UIElements::TreeViewItemData_1<T>,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            crate::UnityEngine::UIElements::TreeViewItemData_1<T>,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_children", ())?;
        Ok(__cordl_ret)
    }
}
