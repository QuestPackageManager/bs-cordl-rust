#[cfg(feature = "System+Text+RegularExpressions+RegexBoyerMoore")]
#[repr(C)]
#[derive(Debug)]
pub struct RegexBoyerMoore {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Positive: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub NegativeASCII: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
    pub NegativeUnicode: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppArray<i32>>,
    >,
    pub Pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub LowASCII: i32,
    pub HighASCII: i32,
    pub RightToLeft: bool,
    pub CaseInsensitive: bool,
    pub _culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
}
#[cfg(feature = "System+Text+RegularExpressions+RegexBoyerMoore")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Text::RegularExpressions::RegexBoyerMoore =>
    "System.Text.RegularExpressions"."RegexBoyerMoore"
);
#[cfg(feature = "System+Text+RegularExpressions+RegexBoyerMoore")]
impl std::ops::Deref for crate::System::Text::RegularExpressions::RegexBoyerMoore {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexBoyerMoore")]
impl std::ops::DerefMut for crate::System::Text::RegularExpressions::RegexBoyerMoore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexBoyerMoore")]
impl crate::System::Text::RegularExpressions::RegexBoyerMoore {
    pub fn IsMatch(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
        beglimit: i32,
        endlimit: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsMatch", (text, index, beglimit, endlimit))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchPattern(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MatchPattern", (text, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        caseInsensitive: bool,
        rightToLeft: bool,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pattern, caseInsensitive, rightToLeft, culture))?;
        Ok(__cordl_object.into())
    }
    pub fn Scan(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
        beglimit: i32,
        endlimit: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Scan", (text, index, beglimit, endlimit))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        caseInsensitive: bool,
        rightToLeft: bool,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pattern, caseInsensitive, rightToLeft, culture))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexBoyerMoore")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Text::RegularExpressions::RegexBoyerMoore {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
