#[cfg(feature = "System+IPv6AddressHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct IPv6AddressHelper {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+IPv6AddressHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IPv6AddressHelper => "System"
    ."IPv6AddressHelper"
);
#[cfg(feature = "System+IPv6AddressHelper")]
impl std::ops::Deref for crate::System::IPv6AddressHelper {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IPv6AddressHelper")]
impl std::ops::DerefMut for crate::System::IPv6AddressHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IPv6AddressHelper")]
impl crate::System::IPv6AddressHelper {
    pub fn FindCompressionRange(
        numbers: crate::System::ReadOnlySpan_1<u16>,
    ) -> quest_hook::libil2cpp::Result<crate::System::ValueTuple_2<i32, i32>> {
        let __cordl_ret: crate::System::ValueTuple_2<i32, i32> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindCompressionRange", (numbers))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalIsValid(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        start: i32,
        end: quest_hook::libil2cpp::ByRefMut<i32>,
        validateStrictAddress: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalIsValid", (name, start, end, validateStrictAddress))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsLoopback(
        numbers: crate::System::ReadOnlySpan_1<u16>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsLoopback", (numbers))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValid(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        start: i32,
        end: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValid", (name, start, end))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidStrict(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        start: i32,
        end: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValidStrict", (name, start, end))?;
        Ok(__cordl_ret.into())
    }
    pub fn Parse(
        address: crate::System::ReadOnlySpan_1<char>,
        numbers: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        start: i32,
        scopeId: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parse", (address, numbers, start, scopeId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseCanonicalName(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        isLoopback: quest_hook::libil2cpp::ByRefMut<bool>,
        scopeId: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseCanonicalName", (str, start, isLoopback, scopeId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldHaveIpv4Embedded(
        numbers: crate::System::ReadOnlySpan_1<u16>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShouldHaveIpv4Embedded", (numbers))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+IPv6AddressHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IPv6AddressHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
