#[cfg(feature = "BGLib+DotnetExtension+CommandLine+CommandLineParserResult")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CommandLineParserResult {
    pub applicationPath: *mut quest_hook::libil2cpp::Il2CppString,
    pub _parsed: *mut crate::System::Collections::Generic::IReadOnlyDictionary_2<
        crate::BGLib::DotnetExtension::CommandLine::ArgumentOption,
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub unexpectedArguments: *mut crate::System::Collections::Generic::IReadOnlyList_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "BGLib+DotnetExtension+CommandLine+CommandLineParserResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BGLib::DotnetExtension::CommandLine::CommandLineParserResult =>
    "BGLib.DotnetExtension.CommandLine"."CommandLineParserResult"
);
#[cfg(feature = "BGLib+DotnetExtension+CommandLine+CommandLineParserResult")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BGLib::DotnetExtension::CommandLine::CommandLineParserResult {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BGLib+DotnetExtension+CommandLine+CommandLineParserResult")]
impl crate::BGLib::DotnetExtension::CommandLine::CommandLineParserResult {
    #[cfg(
        feature = "BGLib+DotnetExtension+CommandLine+CommandLineParserResult+__c__DisplayClass10_0"
    )]
    pub type __c__DisplayClass10_0 = crate::BGLib::DotnetExtension::CommandLine::CommandLineParserResult___c__DisplayClass10_0;
    #[cfg(
        feature = "BGLib+DotnetExtension+CommandLine+CommandLineParserResult+__c__DisplayClass7_0"
    )]
    pub type __c__DisplayClass7_0 = crate::BGLib::DotnetExtension::CommandLine::CommandLineParserResult___c__DisplayClass7_0;
    pub fn Contains_ArgumentOption0(
        &mut self,
        option: crate::BGLib::DotnetExtension::CommandLine::ArgumentOption,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Contains",
            (option),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Contains_Il2CppString1(
        &mut self,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Contains",
            (identifier),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValueOrDefault(
        &mut self,
        option: crate::BGLib::DotnetExtension::CommandLine::ArgumentOption,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetValueOrDefault",
            (option),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        applicationPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parsed: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyDictionary_2<
                crate::BGLib::DotnetExtension::CommandLine::ArgumentOption,
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
        unexpectedArguments: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (applicationPath, parsed, unexpectedArguments),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item_ArgumentOption0(
        &mut self,
        option: crate::BGLib::DotnetExtension::CommandLine::ArgumentOption,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Item", (option))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item_Il2CppString1(
        &mut self,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Item", (identifier))?;
        Ok(__cordl_ret.into())
    }
}
