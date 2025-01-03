#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnHeaderColumnResizeHandle")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiColumnHeaderColumnResizeHandle {
    __cordl_parent: crate::UnityEngine::UIElements::VisualElement,
    pub _dragArea_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnHeaderColumnResizeHandle")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::Internal::MultiColumnHeaderColumnResizeHandle =>
    "UnityEngine.UIElements.Internal"."MultiColumnHeaderColumnResizeHandle"
);
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnHeaderColumnResizeHandle")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumnResizeHandle {
    type Target = crate::UnityEngine::UIElements::VisualElement;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnHeaderColumnResizeHandle")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumnResizeHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnHeaderColumnResizeHandle")]
impl crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumnResizeHandle {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_dragArea(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("get_dragArea", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnHeaderColumnResizeHandle")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumnResizeHandle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
