#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+MqvPublicParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct MqvPublicParameters {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub staticPublicKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters,
    pub ephemeralPublicKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+MqvPublicParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Parameters::MqvPublicParameters =>
    "Org.BouncyCastle.Crypto.Parameters"."MqvPublicParameters"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+MqvPublicParameters")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Parameters::MqvPublicParameters {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+MqvPublicParameters")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Parameters::MqvPublicParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+MqvPublicParameters")]
impl crate::Org::BouncyCastle::Crypto::Parameters::MqvPublicParameters {
    pub fn New(
        staticPublicKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters,
        >,
        ephemeralPublicKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (staticPublicKey, ephemeralPublicKey))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        staticPublicKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters,
        >,
        ephemeralPublicKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (staticPublicKey, ephemeralPublicKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_EphemeralPublicKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters,
        > = __cordl_object.invoke("get_EphemeralPublicKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_StaticPublicKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters,
        > = __cordl_object.invoke("get_StaticPublicKey", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+MqvPublicParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Parameters::MqvPublicParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+MqvPublicParameters")]
impl AsRef<crate::Org::BouncyCastle::Crypto::ICipherParameters>
for crate::Org::BouncyCastle::Crypto::Parameters::MqvPublicParameters {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::ICipherParameters {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+MqvPublicParameters")]
impl AsMut<crate::Org::BouncyCastle::Crypto::ICipherParameters>
for crate::Org::BouncyCastle::Crypto::Parameters::MqvPublicParameters {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::ICipherParameters {
        unsafe { std::mem::transmute(self) }
    }
}
