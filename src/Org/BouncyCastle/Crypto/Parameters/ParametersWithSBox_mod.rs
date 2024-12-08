#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ParametersWithSBox")]
#[repr(C)]
#[derive(Debug)]
pub struct ParametersWithSBox {
    __cordl_parent: crate::System::Object,
    pub parameters: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    pub sBox: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ParametersWithSBox")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Parameters::ParametersWithSBox =>
    "Org.BouncyCastle.Crypto.Parameters"."ParametersWithSBox"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ParametersWithSBox")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Parameters::ParametersWithSBox {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ParametersWithSBox")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Parameters::ParametersWithSBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ParametersWithSBox")]
impl crate::Org::BouncyCastle::Crypto::Parameters::ParametersWithSBox {
    pub fn GetSBox(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetSBox", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        parameters: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
        sBox: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parameters, sBox))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        parameters: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
        sBox: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (parameters, sBox))?;
        Ok(__cordl_ret)
    }
    pub fn get_Parameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters = __cordl_object
            .invoke("get_Parameters", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ParametersWithSBox")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Parameters::ParametersWithSBox {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
