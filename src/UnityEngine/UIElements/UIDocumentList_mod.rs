#[cfg(feature = "UnityEngine+UIElements+UIDocumentList")]
#[repr(C)]
#[derive(Debug)]
pub struct UIDocumentList {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_AttachedUIDocuments: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIDocument>,
        >,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+UIDocumentList")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UIDocumentList {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "UIDocumentList";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIDocumentList as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::UIDocument,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::VisualElement,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("AddToListAndToVisualTree")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIDocumentList as
                    quest_hook::libil2cpp::Type > ::class(), "AddToListAndToVisualTree",
                    3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (uiDocument, visualTree, firstInsertIndex))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIDocumentList as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIDocument>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RemoveFromListAndFromVisualTree")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIDocumentList as
                    quest_hook::libil2cpp::Type > ::class(),
                    "RemoveFromListAndFromVisualTree", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (uiDocument))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIDocumentList as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIDocumentList as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
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
