#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+Expression")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression {
    __cordl_parent: crate::System::Object,
    pub _cordl_type: crate::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionType,
    pub multiplier: crate::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionMultiplier,
    pub dataType: crate::UnityEngine::UIElements::StyleSheets::Syntax::DataType,
    pub combinator: crate::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionCombinator,
    pub subExpressions: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
    >,
    pub keyword: *mut crate::System::String,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+Expression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleSheets::Syntax::Expression =>
    "UnityEngine.UIElements.StyleSheets.Syntax"."Expression"
);
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+Expression")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+Expression")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+Expression")]
impl crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression {
    pub fn New(
        _cordl_type: crate::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionType,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: crate::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+Expression")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}