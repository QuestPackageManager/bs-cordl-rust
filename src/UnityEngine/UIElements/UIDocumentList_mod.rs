#[cfg(feature = "UnityEngine+UIElements+UIDocumentList")]
#[repr(C)]
#[derive(Debug)]
pub struct UIDocumentList {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_AttachedUIDocuments: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::UIDocument,
        >,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+UIDocumentList")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIDocumentList =>
    "UnityEngine.UIElements"."UIDocumentList"
);
#[cfg(feature = "UnityEngine+UIElements+UIDocumentList")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIDocumentList {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIDocumentList")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIDocumentList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIDocumentList")]
impl crate::UnityEngine::UIElements::UIDocumentList {
    pub fn AddToListAndToVisualTree(
        &mut self,
        uiDocument: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIDocument,
        >,
        visualTree: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        firstInsertIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddToListAndToVisualTree",
                (uiDocument, visualTree, firstInsertIndex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RemoveFromListAndFromVisualTree(
        &mut self,
        uiDocument: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIDocument>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveFromListAndFromVisualTree", (uiDocument))?;
        Ok(__cordl_ret.into())
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
}
#[cfg(feature = "UnityEngine+UIElements+UIDocumentList")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIDocumentList {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
