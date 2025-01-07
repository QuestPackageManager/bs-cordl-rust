#[cfg(feature = "System+IO+Enumeration+FileSystemName")]
#[repr(C)]
#[derive(Debug)]
pub struct FileSystemName {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+IO+Enumeration+FileSystemName")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::IO::Enumeration::FileSystemName {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.IO.Enumeration";
    const CLASS_NAME: &'static str = "FileSystemName";
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
#[cfg(feature = "System+IO+Enumeration+FileSystemName")]
impl std::ops::Deref for crate::System::IO::Enumeration::FileSystemName {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
