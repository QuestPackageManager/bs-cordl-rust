#[cfg(feature = "UnityEngine+UIElements+ReusableMultiColumnTreeViewItem")]
#[repr(C)]
#[derive(Debug)]
pub struct ReusableMultiColumnTreeViewItem {
    __cordl_parent: crate::UnityEngine::UIElements::ReusableTreeViewItem,
}
#[cfg(feature = "UnityEngine+UIElements+ReusableMultiColumnTreeViewItem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::ReusableMultiColumnTreeViewItem =>
    "UnityEngine.UIElements"."ReusableMultiColumnTreeViewItem"
);
#[cfg(feature = "UnityEngine+UIElements+ReusableMultiColumnTreeViewItem")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::ReusableMultiColumnTreeViewItem {
    type Target = crate::UnityEngine::UIElements::ReusableTreeViewItem;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ReusableMultiColumnTreeViewItem")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::ReusableMultiColumnTreeViewItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ReusableMultiColumnTreeViewItem")]
impl crate::UnityEngine::UIElements::ReusableMultiColumnTreeViewItem {
    pub fn Init_Columns1(
        &mut self,
        container: *mut crate::UnityEngine::UIElements::VisualElement,
        columns: *mut crate::UnityEngine::UIElements::Columns,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (container, columns))?;
        Ok(__cordl_ret)
    }
    pub fn Init_VisualElement0(
        &mut self,
        item: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (item))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rootElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::VisualElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::VisualElement = __cordl_object
            .invoke("get_rootElement", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+ReusableMultiColumnTreeViewItem")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::ReusableMultiColumnTreeViewItem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}