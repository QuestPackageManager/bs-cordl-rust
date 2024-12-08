#[cfg(feature = "UnityEngine+UIElements+StyleSheets+BaseStyleMatcher")]
#[repr(C)]
#[derive(Debug)]
pub struct BaseStyleMatcher {
    __cordl_parent: crate::System::Object,
    pub m_ContextStack: *mut crate::System::Collections::Generic::Stack_1<
        crate::UnityEngine::UIElements::StyleSheets::BaseStyleMatcher_MatchContext,
    >,
    pub m_CurrentContext: crate::UnityEngine::UIElements::StyleSheets::BaseStyleMatcher_MatchContext,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+BaseStyleMatcher")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleSheets::BaseStyleMatcher =>
    "UnityEngine.UIElements.StyleSheets"."BaseStyleMatcher"
);
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+BaseStyleMatcher")]
impl std::ops::Deref for crate::UnityEngine::UIElements::StyleSheets::BaseStyleMatcher {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+BaseStyleMatcher")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::StyleSheets::BaseStyleMatcher {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+BaseStyleMatcher")]
impl crate::UnityEngine::UIElements::StyleSheets::BaseStyleMatcher {
    #[cfg(feature = "UnityEngine+UIElements+StyleSheets+BaseStyleMatcher+MatchContext")]
    pub type MatchContext = crate::UnityEngine::UIElements::StyleSheets::BaseStyleMatcher_MatchContext;
    pub fn MatchInteger(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MatchInteger", ())?;
        Ok(__cordl_ret)
    }
    pub fn MatchPercentage(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MatchPercentage", ())?;
        Ok(__cordl_ret)
    }
    pub fn MoveNext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveNext", ())?;
        Ok(__cordl_ret)
    }
    pub fn RestoreContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RestoreContext", ())?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_matchedVariableCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_matchedVariableCount", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_currentIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_currentIndex", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isCurrentComma(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isCurrentComma", ())?;
        Ok(__cordl_ret)
    }
    pub fn SaveContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SaveContext", ())?;
        Ok(__cordl_ret)
    }
    pub fn MatchJuxtaposition(
        &mut self,
        exp: *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MatchJuxtaposition", (exp))?;
        Ok(__cordl_ret)
    }
    pub fn MatchOr(
        &mut self,
        exp: *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MatchOr", (exp))?;
        Ok(__cordl_ret)
    }
    pub fn MatchNumber(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MatchNumber", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_hasCurrent(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasCurrent", ())?;
        Ok(__cordl_ret)
    }
    pub fn MatchExpressionWithMultiplier(
        &mut self,
        exp: *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("MatchExpressionWithMultiplier", (exp))?;
        Ok(__cordl_ret)
    }
    pub fn MatchResource(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MatchResource", ())?;
        Ok(__cordl_ret)
    }
    pub fn MatchManyByOrder(
        &mut self,
        exp: *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
        matchOrder: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("MatchManyByOrder", (exp, matchOrder))?;
        Ok(__cordl_ret)
    }
    pub fn DropContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DropContext", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_matchedVariableCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_matchedVariableCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn MatchTime(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MatchTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn MatchCombinator(
        &mut self,
        exp: *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MatchCombinator", (exp))?;
        Ok(__cordl_ret)
    }
    pub fn MatchDataType(
        &mut self,
        exp: *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MatchDataType", (exp))?;
        Ok(__cordl_ret)
    }
    pub fn MatchExpression(
        &mut self,
        exp: *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MatchExpression", (exp))?;
        Ok(__cordl_ret)
    }
    pub fn set_currentIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_currentIndex", (value))?;
        Ok(__cordl_ret)
    }
    pub fn MatchCustomIdent(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MatchCustomIdent", ())?;
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
    pub fn MatchOrOr(
        &mut self,
        exp: *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MatchOrOr", (exp))?;
        Ok(__cordl_ret)
    }
    pub fn MatchAndAnd(
        &mut self,
        exp: *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MatchAndAnd", (exp))?;
        Ok(__cordl_ret)
    }
    pub fn MatchColor(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MatchColor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_valueCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_valueCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn MatchGroup(
        &mut self,
        exp: *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MatchGroup", (exp))?;
        Ok(__cordl_ret)
    }
    pub fn MatchKeyword(
        &mut self,
        keyword: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MatchKeyword", (keyword))?;
        Ok(__cordl_ret)
    }
    pub fn MatchUrl(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MatchUrl", ())?;
        Ok(__cordl_ret)
    }
    pub fn MatchAngle(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MatchAngle", ())?;
        Ok(__cordl_ret)
    }
    pub fn MatchMany(
        &mut self,
        exp: *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("MatchMany", (exp))?;
        Ok(__cordl_ret)
    }
    pub fn Match(
        &mut self,
        exp: *mut crate::UnityEngine::UIElements::StyleSheets::Syntax::Expression,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Match", (exp))?;
        Ok(__cordl_ret)
    }
    pub fn get_isCurrentVariable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isCurrentVariable", ())?;
        Ok(__cordl_ret)
    }
    pub fn MatchLength(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MatchLength", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+BaseStyleMatcher")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StyleSheets::BaseStyleMatcher {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+BaseStyleMatcher+MatchContext")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BaseStyleMatcher_MatchContext {
    pub valueIndex: i32,
    pub matchedVariableCount: i32,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+BaseStyleMatcher+MatchContext")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleSheets::BaseStyleMatcher_MatchContext =>
    "UnityEngine.UIElements.StyleSheets"."BaseStyleMatcher/MatchContext"
);
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+BaseStyleMatcher+MatchContext")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StyleSheets::BaseStyleMatcher_MatchContext {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+BaseStyleMatcher+MatchContext")]
impl crate::UnityEngine::UIElements::StyleSheets::BaseStyleMatcher_MatchContext {}
