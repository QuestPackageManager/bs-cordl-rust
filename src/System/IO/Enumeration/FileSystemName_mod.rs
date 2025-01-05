#[cfg(feature = "System+IO+Enumeration+FileSystemName")]
#[repr(C)]
#[derive(Debug)]
pub struct FileSystemName {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+IO+Enumeration+FileSystemName")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IO::Enumeration::FileSystemName =>
    "System.IO.Enumeration"."FileSystemName"
);
#[cfg(feature = "System+IO+Enumeration+FileSystemName")]
impl std::ops::Deref for crate::System::IO::Enumeration::FileSystemName {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+Enumeration+FileSystemName")]
impl std::ops::DerefMut for crate::System::IO::Enumeration::FileSystemName {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+Enumeration+FileSystemName")]
impl crate::System::IO::Enumeration::FileSystemName {
    pub fn MatchPattern(
        expression: crate::System::ReadOnlySpan_1<char>,
        name: crate::System::ReadOnlySpan_1<char>,
        ignoreCase: bool,
        useExtendedWildcards: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "MatchPattern",
                (expression, name, ignoreCase, useExtendedWildcards),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchesSimpleExpression(
        expression: crate::System::ReadOnlySpan_1<char>,
        name: crate::System::ReadOnlySpan_1<char>,
        ignoreCase: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MatchesSimpleExpression", (expression, name, ignoreCase))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchesWin32Expression(
        expression: crate::System::ReadOnlySpan_1<char>,
        name: crate::System::ReadOnlySpan_1<char>,
        ignoreCase: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MatchesWin32Expression", (expression, name, ignoreCase))?;
        Ok(__cordl_ret.into())
    }
    pub fn TranslateWin32Expression(
        expression: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TranslateWin32Expression", (expression))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+IO+Enumeration+FileSystemName")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::IO::Enumeration::FileSystemName {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
