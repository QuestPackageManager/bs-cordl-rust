#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+KdfFeedbackParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct KdfFeedbackParameters {
    __cordl_parent: crate::System::Object,
    pub ki: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub iv: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub useCounter: bool,
    pub r: i32,
    pub fixedInputData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+KdfFeedbackParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Parameters::KdfFeedbackParameters =>
    "Org.BouncyCastle.Crypto.Parameters"."KdfFeedbackParameters"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+KdfFeedbackParameters")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Parameters::KdfFeedbackParameters {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+KdfFeedbackParameters")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Parameters::KdfFeedbackParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+KdfFeedbackParameters")]
impl crate::Org::BouncyCastle::Crypto::Parameters::KdfFeedbackParameters {
    pub fn New(
        ki: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        iv: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        fixedInputData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        r: i32,
        useCounter: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ki, iv, fixedInputData, r, useCounter))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        ki: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        iv: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        fixedInputData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        r: i32,
        useCounter: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ki, iv, fixedInputData, r, useCounter))?;
        Ok(__cordl_ret)
    }
    pub fn get_FixedInputData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("get_FixedInputData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Iv(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("get_Iv", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Ki(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("get_Ki", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_R(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_R", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UseCounter(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_UseCounter", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+KdfFeedbackParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Parameters::KdfFeedbackParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}