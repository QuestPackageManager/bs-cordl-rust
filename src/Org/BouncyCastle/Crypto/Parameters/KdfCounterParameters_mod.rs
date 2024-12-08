#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+KdfCounterParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct KdfCounterParameters {
    __cordl_parent: crate::System::Object,
    pub ki: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub fixedInputDataCounterPrefix: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub fixedInputDataCounterSuffix: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub r: i32,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+KdfCounterParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Parameters::KdfCounterParameters =>
    "Org.BouncyCastle.Crypto.Parameters"."KdfCounterParameters"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+KdfCounterParameters")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Parameters::KdfCounterParameters {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+KdfCounterParameters")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Parameters::KdfCounterParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+KdfCounterParameters")]
impl crate::Org::BouncyCastle::Crypto::Parameters::KdfCounterParameters {
    pub fn New_Il2CppArray_i32_1(
        ki: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        fixedInputDataCounterPrefix: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        fixedInputDataCounterSuffix: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        r: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (ki, fixedInputDataCounterPrefix, fixedInputDataCounterSuffix, r),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_i32_0(
        ki: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        fixedInputDataCounterSuffix: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        r: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ki, fixedInputDataCounterSuffix, r))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_Il2CppArray_i32_1(
        &mut self,
        ki: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        fixedInputDataCounterPrefix: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        fixedInputDataCounterSuffix: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        r: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (ki, fixedInputDataCounterPrefix, fixedInputDataCounterSuffix, r),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_0(
        &mut self,
        ki: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        fixedInputDataCounterSuffix: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        r: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ki, fixedInputDataCounterSuffix, r))?;
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
    pub fn get_FixedInputDataCounterPrefix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("get_FixedInputDataCounterPrefix", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_FixedInputDataCounterSuffix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("get_FixedInputDataCounterSuffix", ())?;
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
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+KdfCounterParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Parameters::KdfCounterParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
