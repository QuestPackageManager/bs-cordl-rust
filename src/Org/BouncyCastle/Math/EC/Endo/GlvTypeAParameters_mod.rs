#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvTypeAParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct GlvTypeAParameters {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_i: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    pub m_lambda: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    pub m_splitParams: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Math::EC::Endo::ScalarSplitParameters,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvTypeAParameters")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Math::EC::Endo::GlvTypeAParameters {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Math.EC.Endo";
    const CLASS_NAME: &'static str = "GlvTypeAParameters";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
        i: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        lambda: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        splitParams: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Endo::ScalarSplitParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (i, lambda, splitParams))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        i: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        lambda: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        splitParams: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Endo::ScalarSplitParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (i, lambda, splitParams))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_I(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("get_I", ())?;
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
