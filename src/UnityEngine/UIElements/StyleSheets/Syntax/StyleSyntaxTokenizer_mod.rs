#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+StyleSyntaxTokenizer")]
#[repr(C)]
#[derive(Debug)]
pub struct StyleSyntaxTokenizer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn GlobCharacter(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
        c: char,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GlobCharacter", (s, index, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNextCharacter(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
        c: char,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNextCharacter", (s, index, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNextLetterOrDash(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNextLetterOrDash", (s, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNextNumber(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNextNumber", (s, index))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn Tokenize(
        &mut self,
        syntax: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Tokenize", (syntax))?;
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
        Ok(__cordl_ret.into())
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
