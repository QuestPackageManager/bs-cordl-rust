#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsTransport")]
#[repr(C)]
#[derive(Debug)]
pub struct DtlsTransport {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mRecordLayer: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsRecordLayer,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsTransport")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::DtlsTransport =>
    "Org.BouncyCastle.Crypto.Tls"."DtlsTransport"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsTransport")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::DtlsTransport {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsTransport")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::DtlsTransport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsTransport")]
impl crate::Org::BouncyCastle::Crypto::Tls::DtlsTransport {
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetReceiveLimit(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetReceiveLimit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSendLimit(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetSendLimit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        recordLayer: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsRecordLayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (recordLayer))?;
        Ok(__cordl_object.into())
    }
    pub fn Receive(
        &mut self,
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
        len: i32,
        waitMillis: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Receive", (buf, off, len, waitMillis))?;
        Ok(__cordl_ret.into())
    }
    pub fn Send(
        &mut self,
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Send", (buf, off, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        recordLayer: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsRecordLayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (recordLayer))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsTransport")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::DtlsTransport {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsTransport")]
impl AsRef<crate::Org::BouncyCastle::Crypto::Tls::DatagramTransport>
for crate::Org::BouncyCastle::Crypto::Tls::DtlsTransport {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::Tls::DatagramTransport {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsTransport")]
impl AsMut<crate::Org::BouncyCastle::Crypto::Tls::DatagramTransport>
for crate::Org::BouncyCastle::Crypto::Tls::DtlsTransport {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Crypto::Tls::DatagramTransport {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsTransport")]
impl AsRef<crate::Org::BouncyCastle::Crypto::Tls::TlsCloseable>
for crate::Org::BouncyCastle::Crypto::Tls::DtlsTransport {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::Tls::TlsCloseable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsTransport")]
impl AsMut<crate::Org::BouncyCastle::Crypto::Tls::TlsCloseable>
for crate::Org::BouncyCastle::Crypto::Tls::DtlsTransport {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::Tls::TlsCloseable {
        unsafe { std::mem::transmute(self) }
    }
}
