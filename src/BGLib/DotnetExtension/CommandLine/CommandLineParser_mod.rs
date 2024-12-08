#[cfg(feature = "BGLib+DotnetExtension+CommandLine+CommandLineParser")]
#[repr(C)]
#[derive(Debug)]
pub struct CommandLineParser {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BGLib+DotnetExtension+CommandLine+CommandLineParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BGLib::DotnetExtension::CommandLine::CommandLineParser =>
    "BGLib.DotnetExtension.CommandLine"."CommandLineParser"
);
#[cfg(feature = "BGLib+DotnetExtension+CommandLine+CommandLineParser")]
impl std::ops::Deref for crate::BGLib::DotnetExtension::CommandLine::CommandLineParser {
    type Target = crate::System::Object;
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
    #[cfg(feature = "BGLib+DotnetExtension+CommandLine+CommandLineParser+__c")]
    pub type __c = crate::BGLib::DotnetExtension::CommandLine::CommandLineParser___c;
    #[cfg(
        feature = "BGLib+DotnetExtension+CommandLine+CommandLineParser+__c__DisplayClass7_0"
    )]
    pub type __c__DisplayClass7_0 = crate::BGLib::DotnetExtension::CommandLine::CommandLineParser___c__DisplayClass7_0;
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
