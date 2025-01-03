#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+Kdf+DHKdfParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct DHKdfParameters {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub algorithm: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    >,
    pub keySize: i32,
    pub z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub extraInfo: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+Kdf+DHKdfParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Agreement::Kdf::DHKdfParameters =>
    "Org.BouncyCastle.Crypto.Agreement.Kdf"."DHKdfParameters"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+Kdf+DHKdfParameters")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Agreement::Kdf::DHKdfParameters {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+Kdf+DHKdfParameters")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Agreement::Kdf::DHKdfParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+Kdf+DHKdfParameters")]
impl crate::Org::BouncyCastle::Crypto::Agreement::Kdf::DHKdfParameters {
    pub fn GetExtraInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GetExtraInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetZ(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GetZ", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_DerObjectIdentifier_i32_Il2CppArray0(
        algorithm: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
        keySize: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm, keySize, z))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppArray1(
        algorithm: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
        keySize: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        extraInfo: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm, keySize, z, extraInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_DerObjectIdentifier_i32_Il2CppArray0(
        &mut self,
        algorithm: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
        keySize: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algorithm, keySize, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray1(
        &mut self,
        algorithm: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
        keySize: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        extraInfo: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algorithm, keySize, z, extraInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Algorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerObjectIdentifier>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        > = __cordl_object.invoke("get_Algorithm", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_KeySize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_KeySize", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+Kdf+DHKdfParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Agreement::Kdf::DHKdfParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+Kdf+DHKdfParameters")]
impl AsRef<crate::Org::BouncyCastle::Crypto::IDerivationParameters>
for crate::Org::BouncyCastle::Crypto::Agreement::Kdf::DHKdfParameters {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::IDerivationParameters {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+Kdf+DHKdfParameters")]
impl AsMut<crate::Org::BouncyCastle::Crypto::IDerivationParameters>
for crate::Org::BouncyCastle::Crypto::Agreement::Kdf::DHKdfParameters {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Crypto::IDerivationParameters {
        unsafe { std::mem::transmute(self) }
    }
}
