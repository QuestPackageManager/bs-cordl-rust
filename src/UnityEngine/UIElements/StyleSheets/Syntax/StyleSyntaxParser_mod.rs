#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+StyleSyntaxParser")]
#[repr(C)]
#[derive(Debug)]
pub struct StyleSyntaxParser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_ProcessExpressionList: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
    >,
    pub m_ExpressionStack: *mut crate::System::Collections::Generic::Stack_1<
        *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
    >,
    pub m_CombinatorStack: *mut crate::System::Collections::Generic::Stack_1<
        crate::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionCombinator,
    >,
    pub m_ParsedExpressionCache: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+StyleSyntaxParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxParser =>
    "UnityEngine.UIElements.StyleSheets.Syntax"."StyleSyntaxParser"
);
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+StyleSyntaxParser")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxParser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+StyleSyntaxParser")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+StyleSyntaxParser")]
impl crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxParser {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Parse(
        &mut self,
        syntax: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression = __cordl_object
            .invoke("Parse", (syntax))?;
        Ok(__cordl_ret)
    }
    pub fn ParseCombinatorType(
        &mut self,
        tokenizer: *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxTokenizer,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionCombinator,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionCombinator = __cordl_object
            .invoke("ParseCombinatorType", (tokenizer))?;
        Ok(__cordl_ret)
    }
    pub fn ParseDataType(
        &mut self,
        tokenizer: *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxTokenizer,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression = __cordl_object
            .invoke("ParseDataType", (tokenizer))?;
        Ok(__cordl_ret)
    }
    pub fn ParseExpression(
        &mut self,
        tokenizer: *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxTokenizer,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression = __cordl_object
            .invoke("ParseExpression", (tokenizer))?;
        Ok(__cordl_ret)
    }
    pub fn ParseGroup(
        &mut self,
        tokenizer: *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxTokenizer,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression = __cordl_object
            .invoke("ParseGroup", (tokenizer))?;
        Ok(__cordl_ret)
    }
    pub fn ParseMultiplier(
        &mut self,
        tokenizer: *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxTokenizer,
        multiplier: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionMultiplier,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseMultiplier", (tokenizer, multiplier))?;
        Ok(__cordl_ret)
    }
    pub fn ParseNonTerminalValue(
        &mut self,
        syntax: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression = __cordl_object
            .invoke("ParseNonTerminalValue", (syntax))?;
        Ok(__cordl_ret)
    }
    pub fn ParseProperty(
        &mut self,
        tokenizer: *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxTokenizer,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression = __cordl_object
            .invoke("ParseProperty", (tokenizer))?;
        Ok(__cordl_ret)
    }
    pub fn ParseRanges(
        &mut self,
        tokenizer: *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxTokenizer,
        min: quest_hook::libil2cpp::ByRefMut<i32>,
        max: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseRanges", (tokenizer, min, max))?;
        Ok(__cordl_ret)
    }
    pub fn ParseTerm(
        &mut self,
        tokenizer: *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxTokenizer,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression = __cordl_object
            .invoke("ParseTerm", (tokenizer))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessCombinatorStack(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessCombinatorStack", ())?;
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
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+StyleSyntaxParser")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
