#[cfg(feature = "System+Text+RegularExpressions+RegexReplacement")]
#[repr(C)]
#[derive(Debug)]
pub struct RegexReplacement {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _strings: *mut crate::System::Collections::Generic::List_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub _rules: *mut crate::System::Collections::Generic::List_1<i32>,
    pub _Pattern_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "System+Text+RegularExpressions+RegexReplacement")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Text::RegularExpressions::RegexReplacement =>
    "System.Text.RegularExpressions"."RegexReplacement"
);
#[cfg(feature = "System+Text+RegularExpressions+RegexReplacement")]
impl std::ops::Deref for crate::System::Text::RegularExpressions::RegexReplacement {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexReplacement")]
impl std::ops::DerefMut for crate::System::Text::RegularExpressions::RegexReplacement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexReplacement")]
impl crate::System::Text::RegularExpressions::RegexReplacement {
    pub const LastGroup: i32 = -3i32;
    pub const LeftPortion: i32 = -1i32;
    pub const RightPortion: i32 = -2i32;
    pub const Specials: i32 = 4i32;
    pub const WholeString: i32 = -4i32;
    pub fn New(
        rep: *mut quest_hook::libil2cpp::Il2CppString,
        concat: *mut crate::System::Text::RegularExpressions::RegexNode,
        _caps: *mut crate::System::Collections::Hashtable,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (rep, concat, _caps))?;
        Ok(__cordl_object)
    }
    pub fn Replace(
        &mut self,
        regex: *mut crate::System::Text::RegularExpressions::Regex,
        input: *mut quest_hook::libil2cpp::Il2CppString,
        count: i32,
        startat: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("Replace", (regex, input, count, startat))?;
        Ok(__cordl_ret)
    }
    pub fn ReplacementImpl(
        &mut self,
        sb: *mut crate::System::Text::StringBuilder,
        _cordl_match: *mut crate::System::Text::RegularExpressions::Match,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReplacementImpl", (sb, _cordl_match))?;
        Ok(__cordl_ret)
    }
    pub fn ReplacementImplRTL(
        &mut self,
        al: *mut crate::System::Collections::Generic::List_1<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        _cordl_match: *mut crate::System::Text::RegularExpressions::Match,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReplacementImplRTL", (al, _cordl_match))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        rep: *mut quest_hook::libil2cpp::Il2CppString,
        concat: *mut crate::System::Text::RegularExpressions::RegexNode,
        _caps: *mut crate::System::Collections::Hashtable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (rep, concat, _caps))?;
        Ok(__cordl_ret)
    }
    pub fn get_Pattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_Pattern", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexReplacement")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Text::RegularExpressions::RegexReplacement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
