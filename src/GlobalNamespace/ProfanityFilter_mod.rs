#[cfg(feature = "ProfanityFilter")]
#[repr(C)]
#[derive(Debug)]
pub struct ProfanityFilter {
    __cordl_parent: crate::System::Object,
    pub _trie: *mut crate::GlobalNamespace::ProfanityFilter_TrieNode,
}
#[cfg(feature = "ProfanityFilter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ProfanityFilter => ""."ProfanityFilter"
);
#[cfg(feature = "ProfanityFilter")]
impl std::ops::Deref for ProfanityFilter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ProfanityFilter")]
impl std::ops::DerefMut for ProfanityFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ProfanityFilter")]
impl ProfanityFilter {
    #[cfg(feature = "ProfanityFilter+TrieNode")]
    pub type TrieNode = crate::GlobalNamespace::ProfanityFilter_TrieNode;
    #[cfg(feature = "ProfanityFilter+_GetLookalikeLetters_d__4")]
    pub type _GetLookalikeLetters_d__4 = crate::GlobalNamespace::ProfanityFilter__GetLookalikeLetters_d__4;
    pub fn _ctor(
        &mut self,
        wordList: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (wordList))?;
        Ok(__cordl_ret)
    }
    pub fn IsProfane(
        &mut self,
        word: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsProfane", (word))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        wordList: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (wordList))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "ProfanityFilter")]
impl quest_hook::libil2cpp::ObjectType for ProfanityFilter {
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
    __cordl_parent: crate::System::Object,
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
    type Target = crate::System::Object;
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
        word: *mut crate::System::String,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddWord", (word, index))?;
        Ok(__cordl_ret)
    }
    pub fn IsMatch(
        &mut self,
        word: *mut crate::System::String,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsMatch", (word, index))?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
