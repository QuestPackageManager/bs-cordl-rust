#[cfg(feature = "UnityEngine+TextCore+Text+UnicodeLineBreakingRules")]
#[repr(C)]
#[derive(Debug)]
pub struct UnicodeLineBreakingRules {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_UnicodeLineBreakingRules: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::TextAsset,
    >,
    pub m_LeadingCharacters: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
    pub m_FollowingCharacters: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
    pub m_UseModernHangulLineBreakingRules: bool,
    pub m_LeadingCharactersLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<u32>,
    >,
    pub m_FollowingCharactersLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<u32>,
    >,
}
#[cfg(feature = "UnityEngine+TextCore+Text+UnicodeLineBreakingRules")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::TextCore::Text::UnicodeLineBreakingRules =>
    "UnityEngine.TextCore.Text"."UnicodeLineBreakingRules"
);
#[cfg(feature = "UnityEngine+TextCore+Text+UnicodeLineBreakingRules")]
impl std::ops::Deref for crate::UnityEngine::TextCore::Text::UnicodeLineBreakingRules {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+UnicodeLineBreakingRules")]
impl std::ops::DerefMut
for crate::UnityEngine::TextCore::Text::UnicodeLineBreakingRules {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+UnicodeLineBreakingRules")]
impl crate::UnityEngine::TextCore::Text::UnicodeLineBreakingRules {
    pub fn GetCharacters(
        file: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::HashSet_1<u32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<u32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCharacters", (file))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadLineBreakingRules_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadLineBreakingRules", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadLineBreakingRules_TextAsset_TextAsset1(
        &mut self,
        leadingRules: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
        followingRules: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadLineBreakingRules", (leadingRules, followingRules))?;
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
    pub fn get_followingCharacters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset> = __cordl_object
            .invoke("get_followingCharacters", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_followingCharactersLookup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::HashSet_1<u32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<u32>,
        > = __cordl_object.invoke("get_followingCharactersLookup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_leadingCharacters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset> = __cordl_object
            .invoke("get_leadingCharacters", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_leadingCharactersLookup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::HashSet_1<u32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<u32>,
        > = __cordl_object.invoke("get_leadingCharactersLookup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_useModernHangulLineBreakingRules(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_useModernHangulLineBreakingRules", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+UnicodeLineBreakingRules")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TextCore::Text::UnicodeLineBreakingRules {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
