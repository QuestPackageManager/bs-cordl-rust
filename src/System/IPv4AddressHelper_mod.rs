#[cfg(feature = "System+IPv4AddressHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct IPv4AddressHelper {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+IPv4AddressHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IPv4AddressHelper => "System"
    ."IPv4AddressHelper"
);
#[cfg(feature = "System+IPv4AddressHelper")]
impl std::ops::Deref for crate::System::IPv4AddressHelper {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IPv4AddressHelper")]
impl std::ops::DerefMut for crate::System::IPv4AddressHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IPv4AddressHelper")]
impl crate::System::IPv4AddressHelper {
    pub fn IsValid(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        start: i32,
        end: quest_hook::libil2cpp::ByRefMut<i32>,
        allowIPv6: bool,
        notImplicitFile: bool,
        unknownScheme: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "IsValid",
                (name, start, end, allowIPv6, notImplicitFile, unknownScheme),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidCanonical(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        start: i32,
        end: quest_hook::libil2cpp::ByRefMut<i32>,
        allowIPv6: bool,
        notImplicitFile: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValidCanonical", (name, start, end, allowIPv6, notImplicitFile))?;
        Ok(__cordl_ret.into())
    }
    pub fn Parse(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        numbers: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        start: i32,
        end: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parse", (name, numbers, start, end))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseCanonical(
        name: crate::System::ReadOnlySpan_1<char>,
        numbers: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        start: i32,
        end: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseCanonical", (name, numbers, start, end))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseCanonicalName(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        end: i32,
        isLoopback: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseCanonicalName", (str, start, end, isLoopback))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseHostNumber(
        str: crate::System::ReadOnlySpan_1<char>,
        start: i32,
        end: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseHostNumber", (str, start, end))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseNonCanonical(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        start: i32,
        end: quest_hook::libil2cpp::ByRefMut<i32>,
        notImplicitFile: bool,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseNonCanonical", (name, start, end, notImplicitFile))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+IPv4AddressHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IPv4AddressHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
