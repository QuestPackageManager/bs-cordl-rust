#[cfg(feature = "cordl_class_UnityEngine+UIElements+ICollectionDragAndDropController")]
#[repr(C)]
#[derive(Debug)]
pub struct ICollectionDragAndDropController {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+ICollectionDragAndDropController")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::ICollectionDragAndDropController {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "ICollectionDragAndDropController";
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
#[cfg(feature = "UnityEngine+UIElements+ICollectionDragAndDropController")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::ICollectionDragAndDropController {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ICollectionDragAndDropController")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::ICollectionDragAndDropController {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+ICollectionDragAndDropController")]
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
