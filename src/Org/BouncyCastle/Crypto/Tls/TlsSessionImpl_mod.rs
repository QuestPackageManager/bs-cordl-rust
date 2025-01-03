#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSessionImpl")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsSessionImpl {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mSessionID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub mSessionParameters: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::SessionParameters,
    >,
    pub mResumable: bool,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSessionImpl")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::TlsSessionImpl
    => "Org.BouncyCastle.Crypto.Tls"."TlsSessionImpl"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSessionImpl")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::TlsSessionImpl {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSessionImpl")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::TlsSessionImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSessionImpl")]
impl crate::Org::BouncyCastle::Crypto::Tls::TlsSessionImpl {
    pub fn ExportSessionParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::SessionParameters,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::SessionParameters,
        > = __cordl_object.invoke("ExportSessionParameters", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Invalidate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invalidate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        sessionID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        sessionParameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::SessionParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sessionID, sessionParameters))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        sessionID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        sessionParameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::SessionParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sessionID, sessionParameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsResumable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsResumable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SessionID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("get_SessionID", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSessionImpl")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::TlsSessionImpl {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSessionImpl")]
impl AsRef<crate::Org::BouncyCastle::Crypto::Tls::TlsSession>
for crate::Org::BouncyCastle::Crypto::Tls::TlsSessionImpl {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::Tls::TlsSession {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSessionImpl")]
impl AsMut<crate::Org::BouncyCastle::Crypto::Tls::TlsSession>
for crate::Org::BouncyCastle::Crypto::Tls::TlsSessionImpl {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::Tls::TlsSession {
        unsafe { std::mem::transmute(self) }
    }
}
