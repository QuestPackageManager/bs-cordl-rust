#[cfg(feature = "System+Net+IPAddress")]
#[repr(C)]
#[derive(Debug)]
pub struct IPAddress {
    __cordl_parent: crate::System::Object,
    pub _addressOrScopeId: u32,
    pub _numbers: *mut quest_hook::libil2cpp::Il2CppArray<u16>,
    pub _toString: *mut crate::System::String,
    pub _hashCode: i32,
}
#[cfg(feature = "System+Net+IPAddress")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::IPAddress => "System.Net"
    ."IPAddress"
);
#[cfg(feature = "System+Net+IPAddress")]
impl std::ops::Deref for crate::System::Net::IPAddress {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+IPAddress")]
impl std::ops::DerefMut for crate::System::Net::IPAddress {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+IPAddress")]
impl crate::System::Net::IPAddress {
    pub const LoopbackMask: i64 = 255i64;
    pub const NumberOfLabels: i32 = 8i32;
    #[cfg(feature = "System+Net+IPAddress+ReadOnlyIPAddress")]
    pub type ReadOnlyIPAddress = crate::GlobalNamespace::IPAddress_ReadOnlyIPAddress;
    pub fn Equals_Object1(
        &mut self,
        comparand: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (comparand))?;
        Ok(__cordl_ret)
    }
    pub fn Equals__cordl_bool0(
        &mut self,
        comparandObj: *mut crate::System::Object,
        compareScopeId: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Equals", (comparandObj, compareScopeId))?;
        Ok(__cordl_ret)
    }
    pub fn GetAddressBytes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetAddressBytes", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn MapToIPv6(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::IPAddress> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::IPAddress = __cordl_object
            .invoke("MapToIPv6", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_Il2CppArray5(
        address: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (address))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray_i64_1(
        address: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        scopeid: i64,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (address, scopeid))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray_u32_4(
        numbers: *mut quest_hook::libil2cpp::Il2CppArray<u16>,
        scopeid: u32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (numbers, scopeid))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppObject_i32_u32_3(
        numbers: *mut quest_hook::libil2cpp::Il2CppObject,
        numbersLength: i32,
        scopeid: u32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (numbers, numbersLength, scopeid))?;
        Ok(__cordl_object)
    }
    pub fn New_ReadOnlySpan_1_6(
        address: crate::System::ReadOnlySpan_1<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (address))?;
        Ok(__cordl_object)
    }
    pub fn New_ReadOnlySpan_1_i64_2(
        address: crate::System::ReadOnlySpan_1<u8>,
        scopeid: i64,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (address, scopeid))?;
        Ok(__cordl_object)
    }
    pub fn New_i64_0(newAddress: i64) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (newAddress))?;
        Ok(__cordl_object)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn TryWriteBytes(
        &mut self,
        destination: crate::System::Span_1<u8>,
        bytesWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryWriteBytes", (destination, bytesWritten))?;
        Ok(__cordl_ret)
    }
    pub fn WriteIPv4Bytes(
        &mut self,
        destination: crate::System::Span_1<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteIPv4Bytes", (destination))?;
        Ok(__cordl_ret)
    }
    pub fn WriteIPv6Bytes(
        &mut self,
        destination: crate::System::Span_1<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteIPv6Bytes", (destination))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray5(
        &mut self,
        address: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (address))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray_i64_1(
        &mut self,
        address: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        scopeid: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (address, scopeid))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray_u32_4(
        &mut self,
        numbers: *mut quest_hook::libil2cpp::Il2CppArray<u16>,
        scopeid: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (numbers, scopeid))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppObject_i32_u32_3(
        &mut self,
        numbers: *mut quest_hook::libil2cpp::Il2CppObject,
        numbersLength: i32,
        scopeid: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (numbers, numbersLength, scopeid))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ReadOnlySpan_1_6(
        &mut self,
        address: crate::System::ReadOnlySpan_1<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (address))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ReadOnlySpan_1_i64_2(
        &mut self,
        address: crate::System::ReadOnlySpan_1<u8>,
        scopeid: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (address, scopeid))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i64_0(
        &mut self,
        newAddress: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (newAddress))?;
        Ok(__cordl_ret)
    }
    pub fn get_AddressFamily(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::Sockets::AddressFamily> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::Sockets::AddressFamily = __cordl_object
            .invoke("get_AddressFamily", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsIPv4(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsIPv4", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsIPv6(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsIPv6", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsIPv6Multicast(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsIPv6Multicast", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PrivateAddress(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_PrivateAddress", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PrivateScopeId(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_PrivateScopeId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ScopeId(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_ScopeId", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_PrivateAddress(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PrivateAddress", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_PrivateScopeId(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PrivateScopeId", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+IPAddress")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::IPAddress {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
