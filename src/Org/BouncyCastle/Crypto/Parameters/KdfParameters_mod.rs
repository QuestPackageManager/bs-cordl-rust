#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+KdfParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct KdfParameters {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub iv: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub shared: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+KdfParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Parameters::KdfParameters =>
    "Org.BouncyCastle.Crypto.Parameters"."KdfParameters"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+KdfParameters")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Parameters::KdfParameters {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+KdfParameters")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Parameters::KdfParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+KdfParameters")]
impl crate::Org::BouncyCastle::Crypto::Parameters::KdfParameters {
    pub fn GetIV(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GetIV", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSharedSecret(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GetSharedSecret", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        shared: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        iv: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (shared, iv))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        shared: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        iv: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (shared, iv))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+KdfParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Parameters::KdfParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+KdfParameters")]
impl AsRef<crate::Org::BouncyCastle::Crypto::IDerivationParameters>
for crate::Org::BouncyCastle::Crypto::Parameters::KdfParameters {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::IDerivationParameters {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+KdfParameters")]
impl AsMut<crate::Org::BouncyCastle::Crypto::IDerivationParameters>
for crate::Org::BouncyCastle::Crypto::Parameters::KdfParameters {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Crypto::IDerivationParameters {
        unsafe { std::mem::transmute(self) }
    }
}
