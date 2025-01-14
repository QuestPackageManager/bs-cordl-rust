#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+Expression")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _cordl_type: crate::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionType,
    pub multiplier: crate::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionMultiplier,
    pub dataType: crate::UnityEngine::UIElements::StyleSheets::Syntax::DataType,
    pub combinator: crate::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionCombinator,
    pub subExpressions: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
            >,
        >,
    >,
    pub keyword: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+Expression")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.StyleSheets.Syntax";
    const CLASS_NAME: &'static str = "Expression";
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
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+Expression")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: crate::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionType),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (_cordl_type))
        };
        Ok(__cordl_ret.into())
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
