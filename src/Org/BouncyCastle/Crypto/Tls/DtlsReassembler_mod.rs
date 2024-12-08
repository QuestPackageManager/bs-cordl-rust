#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReassembler")]
#[repr(C)]
#[derive(Debug)]
pub struct DtlsReassembler {
    __cordl_parent: crate::System::Object,
    pub mMsgType: u8,
    pub mBody: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mMissing: *mut crate::System::Collections::IList,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReassembler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::DtlsReassembler
    => "Org.BouncyCastle.Crypto.Tls"."DtlsReassembler"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReassembler")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::DtlsReassembler {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReassembler")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::DtlsReassembler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReassembler")]
impl crate::Org::BouncyCastle::Crypto::Tls::DtlsReassembler {
    #[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReassembler+Range")]
    pub type Range = crate::Org::BouncyCastle::Crypto::Tls::DtlsReassembler_Range;
    pub fn ContributeFragment(
        &mut self,
        msg_type: u8,
        length: i32,
        buf: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        off: i32,
        fragment_offset: i32,
        fragment_length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ContributeFragment",
                (msg_type, length, buf, off, fragment_offset, fragment_length),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetBodyIfComplete(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetBodyIfComplete", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(msg_type: u8, length: i32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (msg_type, length))?;
        Ok(__cordl_object)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        msg_type: u8,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (msg_type, length))?;
        Ok(__cordl_ret)
    }
    pub fn get_MsgType(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("get_MsgType", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReassembler")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::DtlsReassembler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReassembler+Range")]
#[repr(C)]
#[derive(Debug)]
pub struct DtlsReassembler_Range {
    __cordl_parent: crate::System::Object,
    pub mStart: i32,
    pub mEnd: i32,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReassembler+Range")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::DtlsReassembler_Range =>
    "Org.BouncyCastle.Crypto.Tls"."DtlsReassembler/Range"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReassembler+Range")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::DtlsReassembler_Range {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReassembler+Range")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Tls::DtlsReassembler_Range {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReassembler+Range")]
impl crate::Org::BouncyCastle::Crypto::Tls::DtlsReassembler_Range {
    pub fn New(start: i32, end: i32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (start, end))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        start: i32,
        end: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (start, end))?;
        Ok(__cordl_ret)
    }
    pub fn get_End(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_End", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Start(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_End(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_End", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Start(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Start", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReassembler+Range")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::DtlsReassembler_Range {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
