#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSessionImpl")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsSessionImpl {
    __cordl_parent: crate::System::Object,
    pub mSessionID: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mSessionParameters: *mut crate::Org::BouncyCastle::Crypto::Tls::SessionParameters,
    pub mResumable: bool,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSessionImpl")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::TlsSessionImpl
    => "Org.BouncyCastle.Crypto.Tls"."TlsSessionImpl"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSessionImpl")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::TlsSessionImpl {
    type Target = crate::System::Object;
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
        *mut crate::Org::BouncyCastle::Crypto::Tls::SessionParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::SessionParameters = __cordl_object
            .invoke("ExportSessionParameters", ())?;
        Ok(__cordl_ret)
    }
    pub fn Invalidate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invalidate", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        sessionID: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        sessionParameters: *mut crate::Org::BouncyCastle::Crypto::Tls::SessionParameters,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sessionID, sessionParameters))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        sessionID: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        sessionParameters: *mut crate::Org::BouncyCastle::Crypto::Tls::SessionParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sessionID, sessionParameters))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsResumable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsResumable", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SessionID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("get_SessionID", ())?;
        Ok(__cordl_ret)
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
