#[cfg(feature = "UnityEngine+UIElements+VisualTreeStyleUpdaterTraversal")]
#[repr(C)]
#[derive(Debug)]
pub struct VisualTreeStyleUpdaterTraversal {
    __cordl_parent: crate::UnityEngine::UIElements::StyleSheets::HierarchyTraversal,
    pub m_ProcessVarContext: *mut crate::UnityEngine::UIElements::StyleVariableContext,
    pub m_UpdateList: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::UnityEngine::UIElements::VisualElement,
    >,
    pub m_ParentList: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::UnityEngine::UIElements::VisualElement,
    >,
    pub m_TempMatchResults: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::UIElements::StyleSheets::SelectorMatchRecord,
    >,
    pub _currentPixelsPerPoint_k__BackingField: f32,
    pub m_StyleMatchingContext: *mut crate::UnityEngine::UIElements::StyleMatchingContext,
    pub m_StylePropertyReader: *mut crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader,
    pub m_AnimatedProperties: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeStyleUpdaterTraversal")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal =>
    "UnityEngine.UIElements"."VisualTreeStyleUpdaterTraversal"
);
#[cfg(feature = "UnityEngine+UIElements+VisualTreeStyleUpdaterTraversal")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal {
    type Target = crate::UnityEngine::UIElements::StyleSheets::HierarchyTraversal;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeStyleUpdaterTraversal")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeStyleUpdaterTraversal")]
impl crate::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal {
    #[cfg(feature = "UnityEngine+UIElements+VisualTreeStyleUpdaterTraversal+__c")]
    pub type __c = crate::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal___c;
    pub fn AddChangedElement(
        &mut self,
        ve: *mut crate::UnityEngine::UIElements::VisualElement,
        versionChangeType: crate::UnityEngine::UIElements::VersionChangeType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddChangedElement", (ve, versionChangeType))?;
        Ok(__cordl_ret)
    }
    pub fn CancelAnimationsWithNoTransitionProperty(
        &mut self,
        element: *mut crate::UnityEngine::UIElements::VisualElement,
        newStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancelAnimationsWithNoTransitionProperty", (element, newStyle))?;
        Ok(__cordl_ret)
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret)
    }
    pub fn ForceUpdateTransitions(
        &mut self,
        element: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ForceUpdateTransitions", (element))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn PrepareTraversal(
        &mut self,
        pixelsPerPoint: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PrepareTraversal", (pixelsPerPoint))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessMatchedRules(
        &mut self,
        element: *mut crate::UnityEngine::UIElements::VisualElement,
        matchingSelectors: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::StyleSheets::SelectorMatchRecord,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::ComputedStyle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::ComputedStyle = __cordl_object
            .invoke("ProcessMatchedRules", (element, matchingSelectors))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessMatchedVariables(
        &mut self,
        sheet: *mut crate::UnityEngine::UIElements::StyleSheet,
        rule: *mut crate::UnityEngine::UIElements::StyleRule,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessMatchedVariables", (sheet, rule))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessTransitions(
        &mut self,
        element: *mut crate::UnityEngine::UIElements::VisualElement,
        oldStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
        newStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessTransitions", (element, oldStyle, newStyle))?;
        Ok(__cordl_ret)
    }
    pub fn PropagateToChildren(
        &mut self,
        ve: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PropagateToChildren", (ve))?;
        Ok(__cordl_ret)
    }
    pub fn PropagateToParents(
        &mut self,
        ve: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PropagateToParents", (ve))?;
        Ok(__cordl_ret)
    }
    pub fn ShouldSkipElement(
        &mut self,
        element: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ShouldSkipElement", (element))?;
        Ok(__cordl_ret)
    }
    pub fn TraverseRecursive(
        &mut self,
        element: *mut crate::UnityEngine::UIElements::VisualElement,
        depth: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TraverseRecursive", (element, depth))?;
        Ok(__cordl_ret)
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
    pub fn get_currentPixelsPerPoint(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_currentPixelsPerPoint", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_currentPixelsPerPoint(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_currentPixelsPerPoint", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeStyleUpdaterTraversal")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
