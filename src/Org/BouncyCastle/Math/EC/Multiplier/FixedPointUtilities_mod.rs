#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+FixedPointUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct FixedPointUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+FixedPointUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Multiplier::FixedPointUtilities =>
    "Org.BouncyCastle.Math.EC.Multiplier"."FixedPointUtilities"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+FixedPointUtilities")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::EC::Multiplier::FixedPointUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+FixedPointUtilities")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Multiplier::FixedPointUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+FixedPointUtilities")]
impl crate::Org::BouncyCastle::Math::EC::Multiplier::FixedPointUtilities {
    #[cfg(
        feature = "Org+BouncyCastle+Math+EC+Multiplier+FixedPointUtilities+FixedPointCallback"
    )]
    pub type FixedPointCallback = crate::Org::BouncyCastle::Math::EC::Multiplier::FixedPointUtilities_FixedPointCallback;
    pub fn GetCombSize(
        c: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCombSize", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFixedPointPreCompInfo(
        preCompInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::FixedPointPreCompInfo,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::FixedPointPreCompInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFixedPointPreCompInfo", (preCompInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Precompute(
        p: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::FixedPointPreCompInfo,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::FixedPointPreCompInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Precompute", (p))?;
        Ok(__cordl_ret.into())
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
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+FixedPointUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Multiplier::FixedPointUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Multiplier+FixedPointUtilities+FixedPointCallback"
)]
#[repr(C)]
#[derive(Debug)]
pub struct FixedPointUtilities_FixedPointCallback {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_p: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
}
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Multiplier+FixedPointUtilities+FixedPointCallback"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Multiplier::FixedPointUtilities_FixedPointCallback =>
    "Org.BouncyCastle.Math.EC.Multiplier"."FixedPointUtilities/FixedPointCallback"
);
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Multiplier+FixedPointUtilities+FixedPointCallback"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::EC::Multiplier::FixedPointUtilities_FixedPointCallback {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Multiplier+FixedPointUtilities+FixedPointCallback"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Multiplier::FixedPointUtilities_FixedPointCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Multiplier+FixedPointUtilities+FixedPointCallback"
)]
impl crate::Org::BouncyCastle::Math::EC::Multiplier::FixedPointUtilities_FixedPointCallback {
    pub fn CheckExisting(
        &mut self,
        existingFP: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::FixedPointPreCompInfo,
        >,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CheckExisting", (existingFP, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckTable(
        &mut self,
        table: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECLookupTable,
        >,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CheckTable", (table, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        p: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (p))?;
        Ok(__cordl_object.into())
    }
    pub fn Precompute(
        &mut self,
        existing: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
        > = __cordl_object.invoke("Precompute", (existing))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        p: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (p))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Multiplier+FixedPointUtilities+FixedPointCallback"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Multiplier::FixedPointUtilities_FixedPointCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Multiplier+FixedPointUtilities+FixedPointCallback"
)]
impl AsRef<crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback>
for crate::Org::BouncyCastle::Math::EC::Multiplier::FixedPointUtilities_FixedPointCallback {
    fn as_ref(
        &self,
    ) -> &crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Multiplier+FixedPointUtilities+FixedPointCallback"
)]
impl AsMut<crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback>
for crate::Org::BouncyCastle::Math::EC::Multiplier::FixedPointUtilities_FixedPointCallback {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback {
        unsafe { std::mem::transmute(self) }
    }
}
