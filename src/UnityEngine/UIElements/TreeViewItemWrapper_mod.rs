#[cfg(feature = "UnityEngine+UIElements+TreeViewItemWrapper")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TreeViewItemWrapper {
    pub item: crate::UnityEngine::UIElements::TreeItem,
    pub depth: i32,
}
#[cfg(feature = "UnityEngine+UIElements+TreeViewItemWrapper")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::TreeViewItemWrapper =>
    "UnityEngine.UIElements"."TreeViewItemWrapper"
);
#[cfg(feature = "UnityEngine+UIElements+TreeViewItemWrapper")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::TreeViewItemWrapper {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TreeViewItemWrapper")]
impl crate::UnityEngine::UIElements::TreeViewItemWrapper {
    pub fn _ctor(
        &mut self,
        item: crate::UnityEngine::UIElements::TreeItem,
        depth: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (item, depth),
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
}
