#[cfg(feature = "UnityEngine+UIElements+ICollectionDragAndDropController")]
#[repr(C)]
#[derive(Debug)]
pub struct ICollectionDragAndDropController {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+ICollectionDragAndDropController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::ICollectionDragAndDropController =>
    "UnityEngine.UIElements"."ICollectionDragAndDropController"
);
#[cfg(feature = "UnityEngine+UIElements+ICollectionDragAndDropController")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::ICollectionDragAndDropController {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ICollectionDragAndDropController")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::ICollectionDragAndDropController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ICollectionDragAndDropController")]
impl crate::UnityEngine::UIElements::ICollectionDragAndDropController {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ICollectionDragAndDropController")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::ICollectionDragAndDropController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+ICollectionDragAndDropController")]
impl AsRef<
    crate::UnityEngine::UIElements::IDragAndDropController_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IListDragAndDropArgs>,
    >,
> for crate::UnityEngine::UIElements::ICollectionDragAndDropController {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::UIElements::IDragAndDropController_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IListDragAndDropArgs>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ICollectionDragAndDropController")]
impl AsMut<
    crate::UnityEngine::UIElements::IDragAndDropController_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IListDragAndDropArgs>,
    >,
> for crate::UnityEngine::UIElements::ICollectionDragAndDropController {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::UIElements::IDragAndDropController_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IListDragAndDropArgs>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ICollectionDragAndDropController")]
impl AsRef<crate::UnityEngine::UIElements::IReorderable>
for crate::UnityEngine::UIElements::ICollectionDragAndDropController {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::IReorderable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ICollectionDragAndDropController")]
impl AsMut<crate::UnityEngine::UIElements::IReorderable>
for crate::UnityEngine::UIElements::ICollectionDragAndDropController {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UIElements::IReorderable {
        unsafe { std::mem::transmute(self) }
    }
}
