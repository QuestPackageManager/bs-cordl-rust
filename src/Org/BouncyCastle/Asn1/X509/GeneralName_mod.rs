#[cfg(feature = "Org+BouncyCastle+Asn1+X509+GeneralName")]
#[repr(C)]
#[derive(Debug)]
pub struct GeneralName {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub obj: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub tag: i32,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+GeneralName")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::X509::GeneralName =>
    "Org.BouncyCastle.Asn1.X509"."GeneralName"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+GeneralName")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::X509::GeneralName {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+GeneralName")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::X509::GeneralName {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+GeneralName")]
impl crate::Org::BouncyCastle::Asn1::X509::GeneralName {
    pub const DirectoryName: i32 = 4i32;
    pub const DnsName: i32 = 2i32;
    pub const EdiPartyName: i32 = 5i32;
    pub const IPAddress: i32 = 7i32;
    pub const OtherName: i32 = 0i32;
    pub const RegisteredID: i32 = 8i32;
    pub const Rfc822Name: i32 = 1i32;
    pub const UniformResourceIdentifier: i32 = 6i32;
    pub const X400Address: i32 = 3i32;
    pub fn New_Asn1Object_i32_1(
        name: *mut crate::Org::BouncyCastle::Asn1::Asn1Object,
        tag: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, tag))?;
        Ok(__cordl_object)
    }
    pub fn New_X509Name0(
        directoryName: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (directoryName))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_Asn1Encodable2(
        tag: i32,
        name: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tag, name))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_String3(
        tag: i32,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tag, name))?;
        Ok(__cordl_object)
    }
    pub fn ToAsn1Object(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::Asn1Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Object = __cordl_object
            .invoke("ToAsn1Object", ())?;
        Ok(__cordl_ret)
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
    pub fn _ctor_Asn1Object_i32_1(
        &mut self,
        name: *mut crate::Org::BouncyCastle::Asn1::Asn1Object,
        tag: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, tag))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_X509Name0(
        &mut self,
        directoryName: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (directoryName))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_Asn1Encodable2(
        &mut self,
        tag: i32,
        name: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tag, name))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_String3(
        &mut self,
        tag: i32,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tag, name))?;
        Ok(__cordl_ret)
    }
    pub fn copyInts(
        &mut self,
        parsedIp: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        addr: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offSet: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("copyInts", (parsedIp, addr, offSet))?;
        Ok(__cordl_ret)
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable = __cordl_object
            .invoke("get_Name", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TagNo(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_TagNo", ())?;
        Ok(__cordl_ret)
    }
    pub fn parseIPv4(
        &mut self,
        ip: *mut crate::System::String,
        addr: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("parseIPv4", (ip, addr, offset))?;
        Ok(__cordl_ret)
    }
    pub fn parseIPv4Mask(
        &mut self,
        mask: *mut crate::System::String,
        addr: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("parseIPv4Mask", (mask, addr, offset))?;
        Ok(__cordl_ret)
    }
    pub fn parseIPv6(
        &mut self,
        ip: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<i32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<i32> = __cordl_object
            .invoke("parseIPv6", (ip))?;
        Ok(__cordl_ret)
    }
    pub fn parseMask(
        &mut self,
        mask: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<i32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<i32> = __cordl_object
            .invoke("parseMask", (mask))?;
        Ok(__cordl_ret)
    }
    pub fn toGeneralNameEncoding(
        &mut self,
        ip: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("toGeneralNameEncoding", (ip))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+GeneralName")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::X509::GeneralName {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}