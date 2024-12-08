#[cfg(feature = "UnityEngine+UIElements+StyleMatchingContext")]
#[repr(C)]
#[derive(Debug)]
pub struct StyleMatchingContext {
    __cordl_parent: crate::System::Object,
    pub m_StyleSheetStack: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::StyleSheet,
    >,
    pub variableContext: *mut crate::UnityEngine::UIElements::StyleVariableContext,
    pub currentElement: *mut crate::UnityEngine::UIElements::VisualElement,
    pub processResult: *mut crate::System::Action_2<
        *mut crate::UnityEngine::UIElements::VisualElement,
        crate::UnityEngine::UIElements::StyleSheets::MatchResultInfo,
    >,
    pub ancestorFilter: *mut crate::UnityEngine::UIElements::AncestorFilter,
}
#[cfg(feature = "UnityEngine+UIElements+StyleMatchingContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::StyleMatchingContext =>
    "UnityEngine.UIElements"."StyleMatchingContext"
);
#[cfg(feature = "UnityEngine+UIElements+StyleMatchingContext")]
impl std::ops::Deref for crate::UnityEngine::UIElements::StyleMatchingContext {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleMatchingContext")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::StyleMatchingContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleMatchingContext")]
impl crate::UnityEngine::UIElements::StyleMatchingContext {
    pub fn AddStyleSheet(
        &mut self,
        sheet: *mut crate::UnityEngine::UIElements::StyleSheet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddStyleSheet", (sheet))?;
        Ok(__cordl_ret)
    }
    pub fn GetStyleSheetAt(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UIElements::StyleSheet> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::StyleSheet = __cordl_object
            .invoke("GetStyleSheetAt", (index))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        processResult: *mut crate::System::Action_2<
            *mut crate::UnityEngine::UIElements::VisualElement,
            crate::UnityEngine::UIElements::StyleSheets::MatchResultInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (processResult))?;
        Ok(__cordl_object)
    }
    pub fn RemoveStyleSheetRange(
        &mut self,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveStyleSheetRange", (index, count))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        processResult: *mut crate::System::Action_2<
            *mut crate::UnityEngine::UIElements::VisualElement,
            crate::UnityEngine::UIElements::StyleSheets::MatchResultInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (processResult))?;
        Ok(__cordl_ret)
    }
    pub fn get_styleSheetCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_styleSheetCount", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleMatchingContext")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StyleMatchingContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
