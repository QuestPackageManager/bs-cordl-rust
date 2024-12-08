#[cfg(feature = "Org+BouncyCastle+Bcpg+S2k")]
#[repr(C)]
#[derive(Debug)]
pub struct S2k {
    __cordl_parent: crate::Org::BouncyCastle::Bcpg::BcpgObject,
    pub _cordl_type: i32,
    pub algorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
    pub iv: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub itCount: i32,
    pub protectionMode: i32,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+S2k")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Bcpg::S2k =>
    "Org.BouncyCastle.Bcpg"."S2k"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+S2k")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::S2k {
    type Target = crate::Org::BouncyCastle::Bcpg::BcpgObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+S2k")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Bcpg::S2k {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+S2k")]
impl crate::Org::BouncyCastle::Bcpg::S2k {
    pub const ExpBias: i32 = 6i32;
    pub const GnuDummyS2K: i32 = 101i32;
    pub const GnuProtectionModeDivertToCard: i32 = 2i32;
    pub const GnuProtectionModeNoPrivateKey: i32 = 1i32;
    pub const Salted: i32 = 1i32;
    pub const SaltedAndIterated: i32 = 3i32;
    pub const Simple: i32 = 0i32;
    pub fn get_IterationCount(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_IterationCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HashAlgorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag = __cordl_object
            .invoke("get_HashAlgorithm", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Stream0(
        &mut self,
        inStr: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (inStr))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_HashAlgorithmTag1(
        &mut self,
        algorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algorithm))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_HashAlgorithmTag_Il2CppArray2(
        &mut self,
        algorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        iv: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algorithm, iv))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_HashAlgorithmTag_Il2CppArray_i32_3(
        &mut self,
        algorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        iv: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        itCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algorithm, iv, itCount))?;
        Ok(__cordl_ret)
    }
    pub fn get_ProtectionMode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ProtectionMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetIV(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetIV", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Type(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Type", ())?;
        Ok(__cordl_ret)
    }
    pub fn Encode(
        &mut self,
        bcpgOut: *mut crate::Org::BouncyCastle::Bcpg::BcpgOutputStream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Encode", (bcpgOut))?;
        Ok(__cordl_ret)
    }
    pub fn GetIterationCount(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("GetIterationCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_Stream0(
        inStr: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (inStr))?;
        Ok(__cordl_object)
    }
    pub fn New_HashAlgorithmTag1(
        algorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm))?;
        Ok(__cordl_object)
    }
    pub fn New_HashAlgorithmTag_Il2CppArray2(
        algorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        iv: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm, iv))?;
        Ok(__cordl_object)
    }
    pub fn New_HashAlgorithmTag_Il2CppArray_i32_3(
        algorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        iv: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        itCount: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm, iv, itCount))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+S2k")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Bcpg::S2k {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
