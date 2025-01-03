#[cfg(feature = "System+Text+RegularExpressions+RegexReplacement")]
#[repr(C)]
#[derive(Debug)]
pub struct RegexReplacement {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _strings: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    >,
    pub _rules: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<i32>,
    >,
    pub _Pattern_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
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
    pub fn GetOrCreate(
        replRef: quest_hook::libil2cpp::Gc<
            crate::System::WeakReference_1<
                *mut crate::System::Text::RegularExpressions::RegexReplacement,
            >,
        >,
        replacement: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        caps: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
        capsize: i32,
        capnames: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
        roptions: crate::System::Text::RegularExpressions::RegexOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::RegexReplacement,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::RegexReplacement,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetOrCreate",
                (replRef, replacement, caps, capsize, capnames, roptions),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        rep: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        concat: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::RegexNode,
        >,
        _caps: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (rep, concat, _caps))?;
        Ok(__cordl_object.into())
    }
    pub fn Replace(
        &mut self,
        regex: quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::Regex>,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        count: i32,
        startat: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("Replace", (regex, input, count, startat))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReplacementImpl(
        &mut self,
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        _cordl_match: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::Match,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReplacementImpl", (sb, _cordl_match))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReplacementImplRTL(
        &mut self,
        al: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
        _cordl_match: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::Match,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReplacementImplRTL", (al, _cordl_match))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        rep: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        concat: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::RegexNode,
        >,
        _caps: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (rep, concat, _caps))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Pattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Pattern", ())?;
        Ok(__cordl_ret.into())
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
