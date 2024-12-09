#[cfg(feature = "System+Text+RegularExpressions+MatchSparse")]
#[repr(C)]
#[derive(Debug)]
pub struct MatchSparse {
    __cordl_parent: crate::System::Text::RegularExpressions::Match,
    pub _caps: *mut crate::System::Collections::Hashtable,
}
#[cfg(feature = "System+Text+RegularExpressions+MatchSparse")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Text::RegularExpressions::MatchSparse =>
    "System.Text.RegularExpressions"."MatchSparse"
);
#[cfg(feature = "System+Text+RegularExpressions+MatchSparse")]
impl std::ops::Deref for crate::System::Text::RegularExpressions::MatchSparse {
    type Target = crate::System::Text::RegularExpressions::Match;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+MatchSparse")]
impl std::ops::DerefMut for crate::System::Text::RegularExpressions::MatchSparse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+MatchSparse")]
impl crate::System::Text::RegularExpressions::MatchSparse {
    pub fn New(
        regex: *mut crate::System::Text::RegularExpressions::Regex,
        caps: *mut crate::System::Collections::Hashtable,
        capcount: i32,
        text: *mut quest_hook::libil2cpp::Il2CppString,
        begpos: i32,
        len: i32,
        startpos: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (regex, caps, capcount, text, begpos, len, startpos))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        regex: *mut crate::System::Text::RegularExpressions::Regex,
        caps: *mut crate::System::Collections::Hashtable,
        capcount: i32,
        text: *mut quest_hook::libil2cpp::Il2CppString,
        begpos: i32,
        len: i32,
        startpos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (regex, caps, capcount, text, begpos, len, startpos))?;
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
#[cfg(feature = "System+Text+RegularExpressions+MatchSparse")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Text::RegularExpressions::MatchSparse {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
