#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+ScalarSplitParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct ScalarSplitParameters {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_v1A: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub m_v1B: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub m_v2A: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub m_v2B: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub m_g1: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub m_g2: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub m_bits: i32,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+ScalarSplitParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Endo::ScalarSplitParameters =>
    "Org.BouncyCastle.Math.EC.Endo"."ScalarSplitParameters"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+ScalarSplitParameters")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::EC::Endo::ScalarSplitParameters {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+ScalarSplitParameters")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Endo::ScalarSplitParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+ScalarSplitParameters")]
impl crate::Org::BouncyCastle::Math::EC::Endo::ScalarSplitParameters {
    pub fn New(
        v1: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Math::BigInteger,
        >,
        v2: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Math::BigInteger,
        >,
        g1: *mut crate::Org::BouncyCastle::Math::BigInteger,
        g2: *mut crate::Org::BouncyCastle::Math::BigInteger,
        bits: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (v1, v2, g1, g2, bits))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        v1: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Math::BigInteger,
        >,
        v2: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Math::BigInteger,
        >,
        g1: *mut crate::Org::BouncyCastle::Math::BigInteger,
        g2: *mut crate::Org::BouncyCastle::Math::BigInteger,
        bits: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (v1, v2, g1, g2, bits))?;
        Ok(__cordl_ret)
    }
    pub fn get_Bits(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Bits", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_G1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_G1", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_G2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_G2", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_V1A(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_V1A", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_V1B(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_V1B", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_V2A(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_V2A", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_V2B(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_V2B", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+ScalarSplitParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Endo::ScalarSplitParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
