#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsFatalAlert")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsFatalAlert {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Tls::TlsException,
    pub alertDescription: u8,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsFatalAlert")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::TlsFatalAlert =>
    "Org.BouncyCastle.Crypto.Tls"."TlsFatalAlert"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsFatalAlert")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::TlsFatalAlert {
    type Target = crate::Org::BouncyCastle::Crypto::Tls::TlsException;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsFatalAlert")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::TlsFatalAlert {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsFatalAlert")]
impl crate::Org::BouncyCastle::Crypto::Tls::TlsFatalAlert {
    pub fn New_Exception1(
        alertDescription: u8,
        alertCause: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (alertDescription, alertCause))?;
        Ok(__cordl_object)
    }
    pub fn New_u8_0(alertDescription: u8) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (alertDescription))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_Exception1(
        &mut self,
        alertDescription: u8,
        alertCause: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (alertDescription, alertCause))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_u8_0(
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
    pub fn get_AlertDescription(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("get_AlertDescription", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsFatalAlert")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::TlsFatalAlert {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
