#[cfg(feature = "UnityEngine+UIElements+TreeItem")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TreeItem {
    pub _id_k__BackingField: i32,
    pub _parentId_k__BackingField: i32,
    pub _childrenIds_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IEnumerable_1<i32>,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+TreeItem")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::TreeItem =>
    "UnityEngine.UIElements"."TreeItem"
);
#[cfg(feature = "UnityEngine+UIElements+TreeItem")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::TreeItem {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TreeItem")]
impl crate::UnityEngine::UIElements::TreeItem {
    pub fn _ctor(
        &mut self,
        id: i32,
        parentId: i32,
        childrenIds: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (id, parentId, childrenIds),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_childrenIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_childrenIds", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasChildren(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_hasChildren",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_id(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_id",
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
}
