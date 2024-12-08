#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsFatalAlertReceived")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsFatalAlertReceived {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Tls::TlsException,
    pub alertDescription: u8,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsFatalAlertReceived")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::TlsFatalAlertReceived =>
    "Org.BouncyCastle.Crypto.Tls"."TlsFatalAlertReceived"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsFatalAlertReceived")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::TlsFatalAlertReceived {
    type Target = crate::Org::BouncyCastle::Crypto::Tls::TlsException;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsFatalAlertReceived")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Tls::TlsFatalAlertReceived {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsFatalAlertReceived")]
impl crate::Org::BouncyCastle::Crypto::Tls::TlsFatalAlertReceived {
    pub fn get_AlertDescription(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("get_AlertDescription", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        alertDescription: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (alertDescription))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        alertDescription: u8,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (alertDescription))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsFatalAlertReceived")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::TlsFatalAlertReceived {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
