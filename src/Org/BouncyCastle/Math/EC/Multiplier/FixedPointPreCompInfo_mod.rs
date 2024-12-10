#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+FixedPointPreCompInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct FixedPointPreCompInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_offset: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    pub m_lookupTable: *mut crate::Org::BouncyCastle::Math::EC::ECLookupTable,
    pub m_width: i32,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+FixedPointPreCompInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Multiplier::FixedPointPreCompInfo =>
    "Org.BouncyCastle.Math.EC.Multiplier"."FixedPointPreCompInfo"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+FixedPointPreCompInfo")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::EC::Multiplier::FixedPointPreCompInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+FixedPointPreCompInfo")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Multiplier::FixedPointPreCompInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+FixedPointPreCompInfo")]
impl crate::Org::BouncyCastle::Math::EC::Multiplier::FixedPointPreCompInfo {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LookupTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECLookupTable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECLookupTable,
        > = __cordl_object.invoke("get_LookupTable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Offset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = __cordl_object.invoke("get_Offset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Width(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Width", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_LookupTable(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECLookupTable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_LookupTable", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Offset(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Offset", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Width(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Width", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+FixedPointPreCompInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Multiplier::FixedPointPreCompInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+FixedPointPreCompInfo")]
impl AsRef<crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo>
for crate::Org::BouncyCastle::Math::EC::Multiplier::FixedPointPreCompInfo {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+FixedPointPreCompInfo")]
impl AsMut<crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo>
for crate::Org::BouncyCastle::Math::EC::Multiplier::FixedPointPreCompInfo {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo {
        unsafe { std::mem::transmute(self) }
    }
}
