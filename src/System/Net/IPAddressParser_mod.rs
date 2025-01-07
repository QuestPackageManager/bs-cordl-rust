#[cfg(feature = "System+Net+IPAddressParser")]
#[repr(C)]
#[derive(Debug)]
pub struct IPAddressParser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+IPAddressParser")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::IPAddressParser {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "IPAddressParser";
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
#[cfg(feature = "System+Net+IPAddressParser")]
impl std::ops::Deref for crate::System::Net::IPAddressParser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+IPAddressParser")]
impl std::ops::DerefMut for crate::System::Net::IPAddressParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+IPAddressParser")]
impl crate::System::Net::IPAddressParser {
    pub fn AppendHex(
        value: u16,
        buffer: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AppendHex", (value, buffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn AppendSections(
        address: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u16>>,
        fromInclusive: i32,
        toExclusive: i32,
        buffer: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AppendSections", (address, fromInclusive, toExclusive, buffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtractIPv4Address(
        address: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u16>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExtractIPv4Address", (address))?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatIPv4AddressNumber(
        number: i32,
        addressString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        offset: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FormatIPv4AddressNumber", (number, addressString, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn IPv4AddressToStringHelper(
        address: u32,
        addressString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IPv4AddressToStringHelper", (address, addressString))?;
        Ok(__cordl_ret.into())
    }
    pub fn IPv4AddressToString_StringBuilder1(
        address: u32,
        destination: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IPv4AddressToString", (address, destination))?;
        Ok(__cordl_ret.into())
    }
    pub fn IPv4AddressToString_u32_0(
        address: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IPv4AddressToString", (address))?;
        Ok(__cordl_ret.into())
    }
    pub fn IPv6AddressToString(
        address: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u16>>,
        scopeId: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IPv6AddressToString", (address, scopeId))?;
        Ok(__cordl_ret.into())
    }
    pub fn IPv6AddressToStringHelper(
        address: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u16>>,
        scopeId: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IPv6AddressToStringHelper", (address, scopeId))?;
        Ok(__cordl_ret.into())
    }
    pub fn Ipv4StringToAddress(
        ipSpan: crate::System::ReadOnlySpan_1<char>,
        address: quest_hook::libil2cpp::ByRefMut<i64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Ipv4StringToAddress", (ipSpan, address))?;
        Ok(__cordl_ret.into())
    }
    pub fn Ipv6StringToAddress(
        ipSpan: crate::System::ReadOnlySpan_1<char>,
        numbers: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        numbersLength: i32,
        scope: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Ipv6StringToAddress", (ipSpan, numbers, numbersLength, scope))?;
        Ok(__cordl_ret.into())
    }
    pub fn Parse(
        ipSpan: crate::System::ReadOnlySpan_1<char>,
        tryParse: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parse", (ipSpan, tryParse))?;
        Ok(__cordl_ret.into())
    }
    pub fn Reverse(number: u16) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Reverse", (number))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+IPAddressParser")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::IPAddressParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
