#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerableFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct FileSystemEnumerableFactory {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerableFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::IO::Enumeration::FileSystemEnumerableFactory => "System.IO.Enumeration"
    ."FileSystemEnumerableFactory"
);
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerableFactory")]
impl std::ops::Deref for crate::System::IO::Enumeration::FileSystemEnumerableFactory {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerableFactory")]
impl std::ops::DerefMut for crate::System::IO::Enumeration::FileSystemEnumerableFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerableFactory")]
impl crate::System::IO::Enumeration::FileSystemEnumerableFactory {
    #[cfg(feature = "System+IO+Enumeration+FileSystemEnumerableFactory+__c")]
    pub type __c = crate::System::IO::Enumeration::FileSystemEnumerableFactory___c;
    #[cfg(
        feature = "System+IO+Enumeration+FileSystemEnumerableFactory+__c__DisplayClass3_0"
    )]
    pub type __c__DisplayClass3_0 = crate::System::IO::Enumeration::FileSystemEnumerableFactory___c__DisplayClass3_0;
    #[cfg(
        feature = "System+IO+Enumeration+FileSystemEnumerableFactory+__c__DisplayClass4_0"
    )]
    pub type __c__DisplayClass4_0 = crate::System::IO::Enumeration::FileSystemEnumerableFactory___c__DisplayClass4_0;
    #[cfg(
        feature = "System+IO+Enumeration+FileSystemEnumerableFactory+__c__DisplayClass5_0"
    )]
    pub type __c__DisplayClass5_0 = crate::System::IO::Enumeration::FileSystemEnumerableFactory___c__DisplayClass5_0;
    #[cfg(
        feature = "System+IO+Enumeration+FileSystemEnumerableFactory+__c__DisplayClass6_0"
    )]
    pub type __c__DisplayClass6_0 = crate::System::IO::Enumeration::FileSystemEnumerableFactory___c__DisplayClass6_0;
    #[cfg(
        feature = "System+IO+Enumeration+FileSystemEnumerableFactory+__c__DisplayClass7_0"
    )]
    pub type __c__DisplayClass7_0 = crate::System::IO::Enumeration::FileSystemEnumerableFactory___c__DisplayClass7_0;
    #[cfg(
        feature = "System+IO+Enumeration+FileSystemEnumerableFactory+__c__DisplayClass8_0"
    )]
    pub type __c__DisplayClass8_0 = crate::System::IO::Enumeration::FileSystemEnumerableFactory___c__DisplayClass8_0;
    pub fn DirectoryInfos(
        directory: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        expression: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: quest_hook::libil2cpp::Gc<crate::System::IO::EnumerationOptions>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::IO::DirectoryInfo,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::IO::DirectoryInfo,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DirectoryInfos", (directory, expression, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn FileInfos(
        directory: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        expression: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: quest_hook::libil2cpp::Gc<crate::System::IO::EnumerationOptions>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::IO::FileInfo,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::IO::FileInfo,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FileInfos", (directory, expression, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn FileSystemInfos(
        directory: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        expression: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: quest_hook::libil2cpp::Gc<crate::System::IO::EnumerationOptions>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::IO::FileSystemInfo,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::IO::FileSystemInfo,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FileSystemInfos", (directory, expression, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchesPattern(
        expression: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        name: crate::System::ReadOnlySpan_1<char>,
        options: quest_hook::libil2cpp::Gc<crate::System::IO::EnumerationOptions>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MatchesPattern", (expression, name, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn NormalizeInputs(
        directory: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        expression: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        options: quest_hook::libil2cpp::Gc<crate::System::IO::EnumerationOptions>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NormalizeInputs", (directory, expression, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn UserDirectories(
        directory: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        expression: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: quest_hook::libil2cpp::Gc<crate::System::IO::EnumerationOptions>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UserDirectories", (directory, expression, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn UserEntries(
        directory: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        expression: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: quest_hook::libil2cpp::Gc<crate::System::IO::EnumerationOptions>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UserEntries", (directory, expression, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn UserFiles(
        directory: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        expression: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: quest_hook::libil2cpp::Gc<crate::System::IO::EnumerationOptions>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UserFiles", (directory, expression, options))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerableFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::IO::Enumeration::FileSystemEnumerableFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
