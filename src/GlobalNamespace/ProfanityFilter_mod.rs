#[cfg(feature = "ProfanityFilter")]
#[repr(C)]
#[derive(Debug)]
pub struct ProfanityFilter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _trie: *mut crate::GlobalNamespace::ProfanityFilter_TrieNode,
}
#[cfg(feature = "ProfanityFilter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ProfanityFilter => ""
    ."ProfanityFilter"
);
#[cfg(feature = "ProfanityFilter")]
impl std::ops::Deref for crate::GlobalNamespace::ProfanityFilter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ProfanityFilter")]
impl std::ops::DerefMut for crate::GlobalNamespace::ProfanityFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ProfanityFilter")]
impl crate::GlobalNamespace::ProfanityFilter {
    #[cfg(feature = "ProfanityFilter+TrieNode")]
    pub type TrieNode = crate::GlobalNamespace::ProfanityFilter_TrieNode;
    pub fn GetLeetEquivalent(c: char) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLeetEquivalent", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLookalikeLetters(
        c: char,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<char>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<char>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLookalikeLetters", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsProfane(
        &mut self,
        word: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsProfane", (word))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        wordList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (wordList))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        wordList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (wordList))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ProfanityFilter")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ProfanityFilter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ProfanityFilter+TrieNode")]
#[repr(C)]
#[derive(Debug)]
pub struct ProfanityFilter_TrieNode {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _children: *mut crate::System::Collections::Generic::Dictionary_2<
        char,
        *mut crate::GlobalNamespace::ProfanityFilter_TrieNode,
    >,
    pub _shortestWord: i32,
}
#[cfg(feature = "ProfanityFilter+TrieNode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ProfanityFilter_TrieNode => ""
    ."ProfanityFilter/TrieNode"
);
#[cfg(feature = "ProfanityFilter+TrieNode")]
impl std::ops::Deref for crate::GlobalNamespace::ProfanityFilter_TrieNode {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ProfanityFilter+TrieNode")]
impl std::ops::DerefMut for crate::GlobalNamespace::ProfanityFilter_TrieNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ProfanityFilter+TrieNode")]
impl crate::GlobalNamespace::ProfanityFilter_TrieNode {
    pub fn AddWord(
        &mut self,
        word: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddWord", (word, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsMatch(
        &mut self,
        word: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsMatch", (word, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "ProfanityFilter+TrieNode")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ProfanityFilter_TrieNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
