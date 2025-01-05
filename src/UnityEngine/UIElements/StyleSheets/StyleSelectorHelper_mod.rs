#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSelectorHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct StyleSelectorHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSelectorHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleSheets::StyleSelectorHelper =>
    "UnityEngine.UIElements.StyleSheets"."StyleSelectorHelper"
);
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSelectorHelper")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::StyleSheets::StyleSelectorHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSelectorHelper")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::StyleSheets::StyleSelectorHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSelectorHelper")]
impl crate::UnityEngine::UIElements::StyleSheets::StyleSelectorHelper {
    pub fn FastLookup(
        table: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IDictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::StyleComplexSelector,
                >,
            >,
        >,
        matchedSelectors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::StyleSheets::SelectorMatchRecord,
            >,
        >,
        context: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleMatchingContext,
        >,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        record: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::StyleSheets::SelectorMatchRecord,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FastLookup", (table, matchedSelectors, context, input, record))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindMatches(
        context: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleMatchingContext,
        >,
        matchedSelectors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::StyleSheets::SelectorMatchRecord,
            >,
        >,
        parentSheetIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindMatches", (context, matchedSelectors, parentSheetIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchRightToLeft(
        element: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        complexSelector: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleComplexSelector,
        >,
        processResult: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                crate::UnityEngine::UIElements::StyleSheets::MatchResultInfo,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MatchRightToLeft", (element, complexSelector, processResult))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchesSelector(
        element: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        selector: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSelector,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleSheets::MatchResultInfo,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::StyleSheets::MatchResultInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MatchesSelector", (element, selector))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSelectorHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StyleSheets::StyleSelectorHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
