#[cfg(feature = "UnityEngine+UIElements+VisualTreeStyleUpdaterTraversal")]
#[repr(C)]
#[derive(Debug)]
pub struct VisualTreeStyleUpdaterTraversal {
    __cordl_parent: crate::UnityEngine::UIElements::StyleSheets::HierarchyTraversal,
    pub m_ProcessVarContext: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::StyleVariableContext,
    >,
    pub m_UpdateList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        >,
    >,
    pub m_ParentList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        >,
    >,
    pub m_TempMatchResults: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::StyleSheets::SelectorMatchRecord,
        >,
    >,
    pub _currentPixelsPerPoint_k__BackingField: f32,
    pub m_StyleMatchingContext: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::StyleMatchingContext,
    >,
    pub m_StylePropertyReader: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader,
    >,
    pub m_AnimatedProperties: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        >,
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
    pub fn AddChangedElement(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        versionChangeType: crate::UnityEngine::UIElements::VersionChangeType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddChangedElement", (ve, versionChangeType))?;
        Ok(__cordl_ret.into())
    }
    pub fn CancelAnimationsWithNoTransitionProperty(
        &mut self,
        element: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        newStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancelAnimationsWithNoTransitionProperty", (element, newStyle))?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ForceUpdateTransitions(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ForceUpdateTransitions", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnProcessMatchResult(
        current: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        info: crate::UnityEngine::UIElements::StyleSheets::MatchResultInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnProcessMatchResult", (current, info))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn ProcessMatchedRules(
        &mut self,
        element: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        matchingSelectors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::StyleSheets::SelectorMatchRecord,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::ComputedStyle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::ComputedStyle = __cordl_object
            .invoke("ProcessMatchedRules", (element, matchingSelectors))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessMatchedVariables(
        &mut self,
        sheet: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleSheet>,
        rule: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleRule>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessMatchedVariables", (sheet, rule))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessTransitions(
        &mut self,
        element: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn PropagateToChildren(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PropagateToChildren", (ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn PropagateToParents(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PropagateToParents", (ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldSkipElement(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ShouldSkipElement", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn TraverseRecursive(
        &mut self,
        element: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        depth: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TraverseRecursive", (element, depth))?;
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
    pub fn get_currentPixelsPerPoint(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_currentPixelsPerPoint", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
