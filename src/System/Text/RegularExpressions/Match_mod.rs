#[cfg(feature = "System+Text+RegularExpressions+Match")]
#[repr(C)]
#[derive(Debug)]
pub struct Match {
    __cordl_parent: crate::System::Text::RegularExpressions::Group,
    pub _groupcoll: *mut crate::System::Text::RegularExpressions::GroupCollection,
    pub _regex: *mut crate::System::Text::RegularExpressions::Regex,
    pub _textbeg: i32,
    pub _textpos: i32,
    pub _textend: i32,
    pub _textstart: i32,
    pub _matches: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
    pub _matchcount: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub _balancing: bool,
}
#[cfg(feature = "System+Text+RegularExpressions+Match")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Text::RegularExpressions::Match =>
    "System.Text.RegularExpressions"."Match"
);
#[cfg(feature = "System+Text+RegularExpressions+Match")]
impl std::ops::Deref for crate::System::Text::RegularExpressions::Match {
    type Target = crate::System::Text::RegularExpressions::Group;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+Match")]
impl std::ops::DerefMut for crate::System::Text::RegularExpressions::Match {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+Match")]
impl crate::System::Text::RegularExpressions::Match {
    pub fn AddMatch(
        &mut self,
        cap: i32,
        start: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddMatch", (cap, start, len))?;
        Ok(__cordl_ret)
    }
    pub fn BalanceMatch(
        &mut self,
        cap: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BalanceMatch", (cap))?;
        Ok(__cordl_ret)
    }
    pub fn GroupToStringImpl(
        &mut self,
        groupnum: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<char>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ReadOnlySpan_1<char> = __cordl_object
            .invoke("GroupToStringImpl", (groupnum))?;
        Ok(__cordl_ret)
    }
    pub fn IsMatched(&mut self, cap: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsMatched", (cap))?;
        Ok(__cordl_ret)
    }
    pub fn LastGroupToStringImpl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<char>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ReadOnlySpan_1<char> = __cordl_object
            .invoke("LastGroupToStringImpl", ())?;
        Ok(__cordl_ret)
    }
    pub fn MatchIndex(&mut self, cap: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("MatchIndex", (cap))?;
        Ok(__cordl_ret)
    }
    pub fn MatchLength(&mut self, cap: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("MatchLength", (cap))?;
        Ok(__cordl_ret)
    }
    pub fn New_1() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_Regex_i32_String_i32_i32_i32_0(
        regex: *mut crate::System::Text::RegularExpressions::Regex,
        capcount: i32,
        text: *mut crate::System::String,
        begpos: i32,
        len: i32,
        startpos: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (regex, capcount, text, begpos, len, startpos))?;
        Ok(__cordl_object)
    }
    pub fn NextMatch(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Text::RegularExpressions::Match,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::RegularExpressions::Match = __cordl_object
            .invoke("NextMatch", ())?;
        Ok(__cordl_ret)
    }
    pub fn RemoveMatch(
        &mut self,
        cap: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveMatch", (cap))?;
        Ok(__cordl_ret)
    }
    pub fn Reset(
        &mut self,
        regex: *mut crate::System::Text::RegularExpressions::Regex,
        text: *mut crate::System::String,
        textbeg: i32,
        textend: i32,
        textstart: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", (regex, text, textbeg, textend, textstart))?;
        Ok(__cordl_ret)
    }
    pub fn Tidy(
        &mut self,
        textpos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Tidy", (textpos))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Regex_i32_String_i32_i32_i32_0(
        &mut self,
        regex: *mut crate::System::Text::RegularExpressions::Regex,
        capcount: i32,
        text: *mut crate::System::String,
        begpos: i32,
        len: i32,
        startpos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (regex, capcount, text, begpos, len, startpos))?;
        Ok(__cordl_ret)
    }
    pub fn get_Groups(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Text::RegularExpressions::GroupCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::RegularExpressions::GroupCollection = __cordl_object
            .invoke("get_Groups", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Text+RegularExpressions+Match")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Text::RegularExpressions::Match {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
