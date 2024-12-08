#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerableFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct FileSystemEnumerableFactory {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerableFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::IO::Enumeration::FileSystemEnumerableFactory => "System.IO.Enumeration"
    ."FileSystemEnumerableFactory"
);
#[cfg(feature = "System+IO+Enumeration+FileSystemEnumerableFactory")]
impl std::ops::Deref for crate::System::IO::Enumeration::FileSystemEnumerableFactory {
    type Target = crate::System::Object;
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
    #[cfg(
        feature = "System+IO+Enumeration+FileSystemEnumerableFactory+__c__DisplayClass3_0"
    )]
    pub type __c__DisplayClass3_0 = crate::System::IO::Enumeration::FileSystemEnumerableFactory___c__DisplayClass3_0;
    #[cfg(
        feature = "System+IO+Enumeration+FileSystemEnumerableFactory+__c__DisplayClass5_0"
    )]
    pub type __c__DisplayClass5_0 = crate::System::IO::Enumeration::FileSystemEnumerableFactory___c__DisplayClass5_0;
    #[cfg(feature = "System+IO+Enumeration+FileSystemEnumerableFactory+__c")]
    pub type __c = crate::System::IO::Enumeration::FileSystemEnumerableFactory___c;
    #[cfg(
        feature = "System+IO+Enumeration+FileSystemEnumerableFactory+__c__DisplayClass4_0"
    )]
    pub type __c__DisplayClass4_0 = crate::System::IO::Enumeration::FileSystemEnumerableFactory___c__DisplayClass4_0;
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
