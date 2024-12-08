#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+StyleSyntaxTokenizer")]
#[repr(C)]
#[derive(Debug)]
pub struct StyleSyntaxTokenizer {
    __cordl_parent: crate::System::Object,
    pub m_Tokens: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxToken,
    >,
    pub m_CurrentTokenIndex: i32,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+StyleSyntaxTokenizer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxTokenizer =>
    "UnityEngine.UIElements.StyleSheets.Syntax"."StyleSyntaxTokenizer"
);
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+StyleSyntaxTokenizer")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxTokenizer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+StyleSyntaxTokenizer")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxTokenizer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+StyleSyntaxTokenizer")]
impl crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxTokenizer {
    pub fn get_current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxToken,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxToken = __cordl_object
            .invoke("get_current", ())?;
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
    pub fn PeekNext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxToken,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxToken = __cordl_object
            .invoke("PeekNext", ())?;
        Ok(__cordl_ret)
    }
    pub fn Tokenize(
        &mut self,
        syntax: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Tokenize", (syntax))?;
        Ok(__cordl_ret)
    }
    pub fn MoveNext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxToken,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxToken = __cordl_object
            .invoke("MoveNext", ())?;
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
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+StyleSyntaxTokenizer")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxTokenizer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
