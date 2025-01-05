#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+IesParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct IesParameters {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub derivation: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub encoding: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub macKeySize: i32,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+IesParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Parameters::IesParameters =>
    "Org.BouncyCastle.Crypto.Parameters"."IesParameters"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+IesParameters")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Parameters::IesParameters {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+IesParameters")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Parameters::IesParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+IesParameters")]
impl crate::Org::BouncyCastle::Crypto::Parameters::IesParameters {
    pub fn GetDerivationV(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GetDerivationV", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEncodingV(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GetEncodingV", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        derivation: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        encoding: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        macKeySize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (derivation, encoding, macKeySize))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        derivation: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        encoding: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        macKeySize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (derivation, encoding, macKeySize))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MacKeySize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_MacKeySize", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+IesParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Parameters::IesParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+IesParameters")]
impl AsRef<crate::Org::BouncyCastle::Crypto::ICipherParameters>
for crate::Org::BouncyCastle::Crypto::Parameters::IesParameters {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::ICipherParameters {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+IesParameters")]
impl AsMut<crate::Org::BouncyCastle::Crypto::ICipherParameters>
for crate::Org::BouncyCastle::Crypto::Parameters::IesParameters {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::ICipherParameters {
        unsafe { std::mem::transmute(self) }
    }
}
