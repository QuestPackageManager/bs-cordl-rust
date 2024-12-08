#[cfg(feature = "UnityEngine+UIElements+ListViewReorderableDragAndDropController")]
#[repr(C)]
#[derive(Debug)]
pub struct ListViewReorderableDragAndDropController {
    __cordl_parent: crate::UnityEngine::UIElements::BaseReorderableDragAndDropController,
    pub m_ListView: *mut crate::UnityEngine::UIElements::BaseListView,
}
#[cfg(feature = "UnityEngine+UIElements+ListViewReorderableDragAndDropController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::ListViewReorderableDragAndDropController =>
    "UnityEngine.UIElements"."ListViewReorderableDragAndDropController"
);
#[cfg(feature = "UnityEngine+UIElements+ListViewReorderableDragAndDropController")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::ListViewReorderableDragAndDropController {
    type Target = crate::UnityEngine::UIElements::BaseReorderableDragAndDropController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ListViewReorderableDragAndDropController")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::ListViewReorderableDragAndDropController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ListViewReorderableDragAndDropController")]
impl crate::UnityEngine::UIElements::ListViewReorderableDragAndDropController {
    pub fn OnDrop(
        &mut self,
        args: *mut crate::UnityEngine::UIElements::IListDragAndDropArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDrop", (args))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        view: *mut crate::UnityEngine::UIElements::BaseListView,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (view))?;
        Ok(__cordl_ret)
    }
    pub fn HandleDragAndDrop(
        &mut self,
        args: *mut crate::UnityEngine::UIElements::IListDragAndDropArgs,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::DragVisualMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::DragVisualMode = __cordl_object
            .invoke("HandleDragAndDrop", (args))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        view: *mut crate::UnityEngine::UIElements::BaseListView,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (view))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+ListViewReorderableDragAndDropController")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::ListViewReorderableDragAndDropController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
