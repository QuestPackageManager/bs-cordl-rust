#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsServerContextImpl")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsServerContextImpl {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsContext,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsServerContextImpl")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::TlsServerContextImpl =>
    "Org.BouncyCastle.Crypto.Tls"."TlsServerContextImpl"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsServerContextImpl")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::TlsServerContextImpl {
    type Target = quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsContext,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsServerContextImpl")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::TlsServerContextImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsServerContextImpl")]
impl crate::Org::BouncyCastle::Crypto::Tls::TlsServerContextImpl {
    pub fn New(
        secureRandom: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        securityParameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::SecurityParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (secureRandom, securityParameters))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        secureRandom: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        securityParameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::SecurityParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (secureRandom, securityParameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsServer(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsServer", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsServerContextImpl")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::TlsServerContextImpl {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsServerContextImpl")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::TlsContext>>
for crate::Org::BouncyCastle::Crypto::Tls::TlsServerContextImpl {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::TlsContext> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsServerContextImpl")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::TlsContext>>
for crate::Org::BouncyCastle::Crypto::Tls::TlsServerContextImpl {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsServerContextImpl")]
impl AsRef<
    quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::TlsServerContext>,
> for crate::Org::BouncyCastle::Crypto::Tls::TlsServerContextImpl {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::TlsServerContext,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsServerContextImpl")]
impl AsMut<
    quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::TlsServerContext>,
> for crate::Org::BouncyCastle::Crypto::Tls::TlsServerContextImpl {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::TlsServerContext,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
