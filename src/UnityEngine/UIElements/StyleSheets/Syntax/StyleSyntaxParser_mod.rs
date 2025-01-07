#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+StyleSyntaxParser")]
#[repr(C)]
#[derive(Debug)]
pub struct StyleSyntaxParser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_ProcessExpressionList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
            >,
        >,
    >,
    pub m_ExpressionStack: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Stack_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
            >,
        >,
    >,
    pub m_CombinatorStack: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Stack_1<
            crate::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionCombinator,
        >,
    >,
    pub m_ParsedExpressionCache: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
            >,
        >,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+StyleSyntaxParser")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxParser {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.StyleSheets.Syntax";
    const CLASS_NAME: &'static str = "StyleSyntaxParser";
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
    pub fn EatSpace(
        tokenizer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxTokenizer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EatSpace", (tokenizer))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsCombinator(
        token: crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxToken,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsCombinator", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsExpressionEnd(
        token: crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxToken,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsExpressionEnd", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsMultiplier(
        token: crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxToken,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsMultiplier", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Parse(
        &mut self,
        syntax: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
        > = __cordl_object.invoke("Parse", (syntax))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseCombinatorType(
        &mut self,
        tokenizer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxTokenizer,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionCombinator,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionCombinator = __cordl_object
            .invoke("ParseCombinatorType", (tokenizer))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseDataType(
        &mut self,
        tokenizer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxTokenizer,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
        > = __cordl_object.invoke("ParseDataType", (tokenizer))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseExpression(
        &mut self,
        tokenizer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxTokenizer,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
        > = __cordl_object.invoke("ParseExpression", (tokenizer))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseGroup(
        &mut self,
        tokenizer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxTokenizer,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
        > = __cordl_object.invoke("ParseGroup", (tokenizer))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseMultiplier(
        &mut self,
        tokenizer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxTokenizer,
        >,
        multiplier: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionMultiplier,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseMultiplier", (tokenizer, multiplier))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseNonTerminalValue(
        &mut self,
        syntax: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
        > = __cordl_object.invoke("ParseNonTerminalValue", (syntax))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseProperty(
        &mut self,
        tokenizer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxTokenizer,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
        > = __cordl_object.invoke("ParseProperty", (tokenizer))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseRanges(
        &mut self,
        tokenizer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxTokenizer,
        >,
        min: quest_hook::libil2cpp::ByRefMut<i32>,
        max: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseRanges", (tokenizer, min, max))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseTerm(
        &mut self,
        tokenizer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxTokenizer,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
        > = __cordl_object.invoke("ParseTerm", (tokenizer))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCombinatorStack(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessCombinatorStack", ())?;
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
