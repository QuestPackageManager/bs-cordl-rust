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
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "VisualTreeStyleUpdaterTraversal";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::VisualElement,
                    >,
                    crate::UnityEngine::UIElements::VersionChangeType,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("AddChangedElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal as
                    quest_hook::libil2cpp::Type > ::class(), "AddChangedElement", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ve, versionChangeType))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::VisualElement,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::UIElements::ComputedStyle,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("CancelAnimationsWithNoTransitionProperty")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal as
                    quest_hook::libil2cpp::Type > ::class(),
                    "CancelAnimationsWithNoTransitionProperty", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (element, newStyle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Clear")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal as
                    quest_hook::libil2cpp::Type > ::class(), "Clear", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ForceUpdateTransitions(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::VisualElement,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ForceUpdateTransitions")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal as
                    quest_hook::libil2cpp::Type > ::class(), "ForceUpdateTransitions",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (element))?
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
    pub fn OnProcessMatchResult(
        current: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        info: crate::UnityEngine::UIElements::StyleSheets::MatchResultInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::VisualElement,
                    >,
                    crate::UnityEngine::UIElements::StyleSheets::MatchResultInfo,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("OnProcessMatchResult")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal as
                    quest_hook::libil2cpp::Type > ::class(), "OnProcessMatchResult",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (current, info))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PrepareTraversal(
        &mut self,
        pixelsPerPoint: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("PrepareTraversal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal as
                    quest_hook::libil2cpp::Type > ::class(), "PrepareTraversal", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pixelsPerPoint))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::VisualElement,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            crate::UnityEngine::UIElements::StyleSheets::SelectorMatchRecord,
                        >,
                    >,
                ),
                crate::UnityEngine::UIElements::ComputedStyle,
                2usize,
            >("ProcessMatchedRules")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal as
                    quest_hook::libil2cpp::Type > ::class(), "ProcessMatchedRules",
                    2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::ComputedStyle = unsafe {
            method.invoke_unchecked(self, (element, matchingSelectors))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessMatchedVariables(
        &mut self,
        sheet: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleSheet>,
        rule: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleRule>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::StyleSheet,
                    >,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleRule>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ProcessMatchedVariables")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal as
                    quest_hook::libil2cpp::Type > ::class(), "ProcessMatchedVariables",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (sheet, rule))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::VisualElement,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::UIElements::ComputedStyle,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::UIElements::ComputedStyle,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("ProcessTransitions")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal as
                    quest_hook::libil2cpp::Type > ::class(), "ProcessTransitions", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (element, oldStyle, newStyle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PropagateToChildren(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::VisualElement,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("PropagateToChildren")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal as
                    quest_hook::libil2cpp::Type > ::class(), "PropagateToChildren",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ve))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PropagateToParents(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::VisualElement,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("PropagateToParents")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal as
                    quest_hook::libil2cpp::Type > ::class(), "PropagateToParents", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ve))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShouldSkipElement(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::VisualElement,
                >),
                bool,
                1usize,
            >("ShouldSkipElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal as
                    quest_hook::libil2cpp::Type > ::class(), "ShouldSkipElement", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (element))? };
        Ok(__cordl_ret.into())
    }
    pub fn TraverseRecursive(
        &mut self,
        element: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        depth: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::VisualElement,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("TraverseRecursive")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal as
                    quest_hook::libil2cpp::Type > ::class(), "TraverseRecursive", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (element, depth))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_currentPixelsPerPoint(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_currentPixelsPerPoint")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal as
                    quest_hook::libil2cpp::Type > ::class(), "get_currentPixelsPerPoint",
                    0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_currentPixelsPerPoint(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_currentPixelsPerPoint")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal as
                    quest_hook::libil2cpp::Type > ::class(), "set_currentPixelsPerPoint",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
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
