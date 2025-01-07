#[cfg(feature = "UnityEngine+UIElements+StyleMatchingContext")]
#[repr(C)]
#[derive(Debug)]
pub struct StyleMatchingContext {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_StyleSheetStack: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleSheet>,
        >,
    >,
    pub variableContext: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::StyleVariableContext,
    >,
    pub currentElement: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub processResult: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
            crate::UnityEngine::UIElements::StyleSheets::MatchResultInfo,
        >,
    >,
    pub ancestorFilter: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::AncestorFilter,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+StyleMatchingContext")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::StyleMatchingContext {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "StyleMatchingContext";
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
#[cfg(feature = "UnityEngine+UIElements+StyleMatchingContext")]
impl std::ops::Deref for crate::UnityEngine::UIElements::StyleMatchingContext {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        sheet: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleSheet>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddStyleSheet", (sheet))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStyleSheetAt(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleSheet>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheet,
        > = __cordl_object.invoke("GetStyleSheetAt", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        processResult: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                crate::UnityEngine::UIElements::StyleSheets::MatchResultInfo,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (processResult))?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        processResult: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                crate::UnityEngine::UIElements::StyleSheets::MatchResultInfo,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (processResult))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_styleSheetCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_styleSheetCount", ())?;
        Ok(__cordl_ret.into())
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
