#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvTypeAParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct GlvTypeAParameters {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_i: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub m_lambda: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub m_splitParams: *mut crate::Org::BouncyCastle::Math::EC::Endo::ScalarSplitParameters,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvTypeAParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Endo::GlvTypeAParameters =>
    "Org.BouncyCastle.Math.EC.Endo"."GlvTypeAParameters"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvTypeAParameters")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::Endo::GlvTypeAParameters {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvTypeAParameters")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Endo::GlvTypeAParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvTypeAParameters")]
impl crate::Org::BouncyCastle::Math::EC::Endo::GlvTypeAParameters {
    pub fn New(
        i: *mut crate::Org::BouncyCastle::Math::BigInteger,
        lambda: *mut crate::Org::BouncyCastle::Math::BigInteger,
        splitParams: *mut crate::Org::BouncyCastle::Math::EC::Endo::ScalarSplitParameters,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (i, lambda, splitParams))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        i: *mut crate::Org::BouncyCastle::Math::BigInteger,
        lambda: *mut crate::Org::BouncyCastle::Math::BigInteger,
        splitParams: *mut crate::Org::BouncyCastle::Math::EC::Endo::ScalarSplitParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (i, lambda, splitParams))?;
        Ok(__cordl_ret)
    }
    pub fn get_I(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_I", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Lambda(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_Lambda", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SplitParams(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::Endo::ScalarSplitParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::Endo::ScalarSplitParameters = __cordl_object
            .invoke("get_SplitParams", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvTypeAParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Endo::GlvTypeAParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
