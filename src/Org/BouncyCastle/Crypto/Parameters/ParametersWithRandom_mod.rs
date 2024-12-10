#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ParametersWithRandom")]
#[repr(C)]
#[derive(Debug)]
pub struct ParametersWithRandom {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub parameters: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    pub random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ParametersWithRandom")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Parameters::ParametersWithRandom =>
    "Org.BouncyCastle.Crypto.Parameters"."ParametersWithRandom"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ParametersWithRandom")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Parameters::ParametersWithRandom {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ParametersWithRandom")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Parameters::ParametersWithRandom {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ParametersWithRandom")]
impl crate::Org::BouncyCastle::Crypto::Parameters::ParametersWithRandom {
    pub fn GetRandom(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Security::SecureRandom>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        > = __cordl_object.invoke("GetRandom", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_ICipherParameters1(
        parameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parameters))?;
        Ok(__cordl_object.into())
    }
    pub fn New_SecureRandom0(
        parameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherParameters,
        >,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parameters, random))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_ICipherParameters1(
        &mut self,
        parameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SecureRandom0(
        &mut self,
        parameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherParameters,
        >,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (parameters, random))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Parameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::ICipherParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherParameters,
        > = __cordl_object.invoke("get_Parameters", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Random(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Security::SecureRandom>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        > = __cordl_object.invoke("get_Random", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ParametersWithRandom")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Parameters::ParametersWithRandom {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ParametersWithRandom")]
impl AsRef<crate::Org::BouncyCastle::Crypto::ICipherParameters>
for crate::Org::BouncyCastle::Crypto::Parameters::ParametersWithRandom {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::ICipherParameters {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ParametersWithRandom")]
impl AsMut<crate::Org::BouncyCastle::Crypto::ICipherParameters>
for crate::Org::BouncyCastle::Crypto::Parameters::ParametersWithRandom {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::ICipherParameters {
        unsafe { std::mem::transmute(self) }
    }
}
