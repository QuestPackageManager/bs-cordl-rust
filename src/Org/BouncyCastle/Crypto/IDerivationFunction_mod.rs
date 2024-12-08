#[cfg(feature = "Org+BouncyCastle+Crypto+IDerivationFunction")]
#[repr(C)]
#[derive(Debug)]
pub struct IDerivationFunction {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+IDerivationFunction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::IDerivationFunction
    => "Org.BouncyCastle.Crypto"."IDerivationFunction"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+IDerivationFunction")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::IDerivationFunction {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+IDerivationFunction")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::IDerivationFunction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+IDerivationFunction")]
impl crate::Org::BouncyCastle::Crypto::IDerivationFunction {
    pub fn get_Digest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Crypto::IDigest> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::IDigest = __cordl_object
            .invoke("get_Digest", ())?;
        Ok(__cordl_ret)
    }
    pub fn GenerateBytes(
        &mut self,
        output: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        outOff: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GenerateBytes", (output, outOff, length))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        parameters: *mut crate::Org::BouncyCastle::Crypto::IDerivationParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (parameters))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+IDerivationFunction")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::IDerivationFunction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
