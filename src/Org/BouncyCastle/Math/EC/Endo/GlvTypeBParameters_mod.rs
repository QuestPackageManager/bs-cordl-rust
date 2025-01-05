#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvTypeBParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct GlvTypeBParameters {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_beta: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    pub m_lambda: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    pub m_splitParams: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Math::EC::Endo::ScalarSplitParameters,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvTypeBParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Endo::GlvTypeBParameters =>
    "Org.BouncyCastle.Math.EC.Endo"."GlvTypeBParameters"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvTypeBParameters")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::Endo::GlvTypeBParameters {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvTypeBParameters")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Endo::GlvTypeBParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvTypeBParameters")]
impl crate::Org::BouncyCastle::Math::EC::Endo::GlvTypeBParameters {
    pub fn New_Il2CppArray_Il2CppArray_BigInteger_BigInteger_i32_0(
        beta: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        lambda: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        v1: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::BigInteger,
            >,
        >,
        v2: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::BigInteger,
            >,
        >,
        g1: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        g2: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        bits: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beta, lambda, v1, v2, g1, g2, bits))?;
        Ok(__cordl_object.into())
    }
    pub fn New_ScalarSplitParameters1(
        beta: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        lambda: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        splitParams: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Endo::ScalarSplitParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beta, lambda, splitParams))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Il2CppArray_Il2CppArray_BigInteger_BigInteger_i32_0(
        &mut self,
        beta: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        lambda: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        v1: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::BigInteger,
            >,
        >,
        v2: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::BigInteger,
            >,
        >,
        g1: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        g2: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        bits: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beta, lambda, v1, v2, g1, g2, bits))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ScalarSplitParameters1(
        &mut self,
        beta: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        lambda: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        splitParams: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Endo::ScalarSplitParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beta, lambda, splitParams))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Beta(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("get_Beta", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Bits(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Bits", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_G1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("get_G1", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_G2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("get_G2", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Lambda(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("get_Lambda", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SplitParams(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Endo::ScalarSplitParameters,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Endo::ScalarSplitParameters,
        > = __cordl_object.invoke("get_SplitParams", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_V1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::BigInteger,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::BigInteger,
            >,
        > = __cordl_object.invoke("get_V1", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_V2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::BigInteger,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::BigInteger,
            >,
        > = __cordl_object.invoke("get_V2", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvTypeBParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Endo::GlvTypeBParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
