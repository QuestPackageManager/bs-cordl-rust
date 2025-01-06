#[cfg(feature = "BGLib+DotnetExtension+CommandLine+CommandLineParser")]
#[repr(C)]
#[derive(Debug)]
pub struct CommandLineParser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BGLib+DotnetExtension+CommandLine+CommandLineParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BGLib::DotnetExtension::CommandLine::CommandLineParser =>
    "BGLib.DotnetExtension.CommandLine"."CommandLineParser"
);
#[cfg(feature = "BGLib+DotnetExtension+CommandLine+CommandLineParser")]
impl std::ops::Deref for crate::BGLib::DotnetExtension::CommandLine::CommandLineParser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+DotnetExtension+CommandLine+CommandLineParser")]
impl std::ops::DerefMut
for crate::BGLib::DotnetExtension::CommandLine::CommandLineParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+DotnetExtension+CommandLine+CommandLineParser")]
impl crate::BGLib::DotnetExtension::CommandLine::CommandLineParser {
    pub const kArgumentIdentifierPattern: &'static str = "^(?>\\w|-|_)+$";
    pub fn AddParsedOption(
        parsedOption: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::BGLib::DotnetExtension::CommandLine::ArgumentOption,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        option: crate::BGLib::DotnetExtension::CommandLine::ArgumentOption,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddParsedOption", (parsedOption, option, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateHint(
        options: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::BGLib::DotnetExtension::CommandLine::ArgumentOption,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GenerateHint", (options))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateOptionsMap(
        options: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::BGLib::DotnetExtension::CommandLine::ArgumentOption,
            >,
        >,
        argumentIdentifierRegex: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::Regex,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                crate::BGLib::DotnetExtension::CommandLine::ArgumentOption,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                crate::BGLib::DotnetExtension::CommandLine::ArgumentOption,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GenerateOptionsMap", (options, argumentIdentifierRegex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCommandLineArgs() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCommandLineArgs", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseArgs(
        args: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        startIndex: i32,
        requiredOptions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::BGLib::DotnetExtension::CommandLine::ArgumentOption,
            >,
        >,
        optionsMap: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyDictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                crate::BGLib::DotnetExtension::CommandLine::ArgumentOption,
            >,
        >,
        parsedOption: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::Dictionary_2<
                    crate::BGLib::DotnetExtension::CommandLine::ArgumentOption,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                >,
            >,
        >,
        ignored: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ParseArgs",
                (args, startIndex, requiredOptions, optionsMap, parsedOption, ignored),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseCommandLine(
        args: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        options: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::BGLib::DotnetExtension::CommandLine::ArgumentOption,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::BGLib::DotnetExtension::CommandLine::CommandLineParserResult,
    > {
        let __cordl_ret: crate::BGLib::DotnetExtension::CommandLine::CommandLineParserResult = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseCommandLine", (args, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseEnvironmentCommandLine(
        options: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::BGLib::DotnetExtension::CommandLine::ArgumentOption,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::BGLib::DotnetExtension::CommandLine::CommandLineParserResult,
    > {
        let __cordl_ret: crate::BGLib::DotnetExtension::CommandLine::CommandLineParserResult = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseEnvironmentCommandLine", (options))?;
        Ok(__cordl_ret.into())
    }
    pub fn SelectRequiredOptions(
        options: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::BGLib::DotnetExtension::CommandLine::ArgumentOption,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                crate::BGLib::DotnetExtension::CommandLine::ArgumentOption,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                crate::BGLib::DotnetExtension::CommandLine::ArgumentOption,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SelectRequiredOptions", (options))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+DotnetExtension+CommandLine+CommandLineParser")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::DotnetExtension::CommandLine::CommandLineParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
