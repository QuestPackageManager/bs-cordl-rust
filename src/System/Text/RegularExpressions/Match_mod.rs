#[cfg(feature = "System+Text+RegularExpressions+Match")]
#[repr(C)]
#[derive(Debug)]
pub struct Match {
    __cordl_parent: crate::System::Text::RegularExpressions::Group,
    pub _groupcoll: quest_hook::libil2cpp::Gc<
        crate::System::Text::RegularExpressions::GroupCollection,
    >,
    pub _regex: quest_hook::libil2cpp::Gc<
        crate::System::Text::RegularExpressions::Regex,
    >,
    pub _textbeg: i32,
    pub _textpos: i32,
    pub _textend: i32,
    pub _textstart: i32,
    pub _matches: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        >,
    >,
    pub _matchcount: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub _balancing: bool,
}
#[cfg(feature = "System+Text+RegularExpressions+Match")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Text::RegularExpressions::Match {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Text.RegularExpressions";
    const CLASS_NAME: &'static str = "Match";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, i32),
                quest_hook::libil2cpp::Void,
                3usize,
            >("AddMatch")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddMatch", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (cap, start, len))
        };
        Ok(__cordl_ret.into())
    }
    pub fn BalanceMatch(
        &mut self,
        cap: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("BalanceMatch")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "BalanceMatch", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (cap))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GroupToStringImpl(
        &mut self,
        groupnum: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<char>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                crate::System::ReadOnlySpan_1<char>,
                1usize,
            >("GroupToStringImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GroupToStringImpl", 1usize
                )
            });
        let __cordl_ret: crate::System::ReadOnlySpan_1<char> = unsafe {
            method.invoke_unchecked(self, (groupnum))
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsMatched(&mut self, cap: i32) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), bool, 1usize>("IsMatched")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsMatched", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (cap)) };
        Ok(__cordl_ret.into())
    }
    pub fn LastGroupToStringImpl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<char>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::System::ReadOnlySpan_1<char>,
                0usize,
            >("LastGroupToStringImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LastGroupToStringImpl", 0usize
                )
            });
        let __cordl_ret: crate::System::ReadOnlySpan_1<char> = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn MatchIndex(&mut self, cap: i32) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), i32, 1usize>("MatchIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "MatchIndex", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (cap)) };
        Ok(__cordl_ret.into())
    }
    pub fn MatchLength(&mut self, cap: i32) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), i32, 1usize>("MatchLength")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "MatchLength", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (cap)) };
        Ok(__cordl_ret.into())
    }
    pub fn New_1() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Regex_i32_Il2CppString_i32_i32_i32_0(
        regex: quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::Regex>,
        capcount: i32,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        begpos: i32,
        len: i32,
        startpos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (regex, capcount, text, begpos, len, startpos))?;
        Ok(__cordl_object.into())
    }
    pub fn NextMatch(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::Match>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Text::RegularExpressions::Match,
                >,
                0usize,
            >("NextMatch")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NextMatch", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::Match,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveMatch(
        &mut self,
        cap: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("RemoveMatch")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RemoveMatch", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (cap))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Reset(
        &mut self,
        regex: quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::Regex>,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        textbeg: i32,
        textend: i32,
        textstart: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Text::RegularExpressions::Regex,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    i32,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("Reset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Reset", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (regex, text, textbeg, textend, textstart))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Tidy(
        &mut self,
        textpos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("Tidy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Tidy", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (textpos))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Regex_i32_Il2CppString_i32_i32_i32_0(
        &mut self,
        regex: quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::Regex>,
        capcount: i32,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        begpos: i32,
        len: i32,
        startpos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Text::RegularExpressions::Regex,
                    >,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    i32,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (regex, capcount, text, begpos, len, startpos))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Empty() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::Match>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Text::RegularExpressions::Match,
                >,
                0usize,
            >("get_Empty")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Empty", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::Match,
        > = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_Groups(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::GroupCollection,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Text::RegularExpressions::GroupCollection,
                >,
                0usize,
            >("get_Groups")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Groups", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::GroupCollection,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
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
