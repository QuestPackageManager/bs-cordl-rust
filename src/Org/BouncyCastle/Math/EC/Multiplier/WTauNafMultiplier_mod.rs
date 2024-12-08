#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WTauNafMultiplier+WTauNafCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct WTauNafMultiplier_WTauNafCallback {
    __cordl_parent: crate::System::Object,
    pub m_p: *mut crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint,
    pub m_a: i8,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WTauNafMultiplier+WTauNafCallback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Multiplier::WTauNafMultiplier_WTauNafCallback =>
    "Org.BouncyCastle.Math.EC.Multiplier"."WTauNafMultiplier/WTauNafCallback"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WTauNafMultiplier+WTauNafCallback")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::EC::Multiplier::WTauNafMultiplier_WTauNafCallback {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WTauNafMultiplier+WTauNafCallback")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Multiplier::WTauNafMultiplier_WTauNafCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WTauNafMultiplier+WTauNafCallback")]
impl crate::Org::BouncyCastle::Math::EC::Multiplier::WTauNafMultiplier_WTauNafCallback {
    pub fn New(
        p: *mut crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint,
        a: i8,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (p, a))?;
        Ok(__cordl_object)
    }
    pub fn Precompute(
        &mut self,
        existing: *mut crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo = __cordl_object
            .invoke("Precompute", (existing))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        p: *mut crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint,
        a: i8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (p, a))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WTauNafMultiplier+WTauNafCallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Multiplier::WTauNafMultiplier_WTauNafCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WTauNafMultiplier")]
#[repr(C)]
#[derive(Debug)]
pub struct WTauNafMultiplier {
    __cordl_parent: crate::Org::BouncyCastle::Math::EC::Multiplier::AbstractECMultiplier,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WTauNafMultiplier")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Multiplier::WTauNafMultiplier =>
    "Org.BouncyCastle.Math.EC.Multiplier"."WTauNafMultiplier"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WTauNafMultiplier")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::EC::Multiplier::WTauNafMultiplier {
    type Target = crate::Org::BouncyCastle::Math::EC::Multiplier::AbstractECMultiplier;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WTauNafMultiplier")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Multiplier::WTauNafMultiplier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WTauNafMultiplier")]
impl crate::Org::BouncyCastle::Math::EC::Multiplier::WTauNafMultiplier {
    #[cfg(
        feature = "Org+BouncyCastle+Math+EC+Multiplier+WTauNafMultiplier+WTauNafCallback"
    )]
    pub type WTauNafCallback = crate::Org::BouncyCastle::Math::EC::Multiplier::WTauNafMultiplier_WTauNafCallback;
    pub fn MultiplyPositive(
        &mut self,
        point: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
        k: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECPoint = __cordl_object
            .invoke("MultiplyPositive", (point, k))?;
        Ok(__cordl_ret)
    }
    pub fn MultiplyWTnaf(
        &mut self,
        p: *mut crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint,
        lambda: *mut crate::Org::BouncyCastle::Math::EC::Abc::ZTauElement,
        a: i8,
        mu: i8,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint = __cordl_object
            .invoke("MultiplyWTnaf", (p, lambda, a, mu))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WTauNafMultiplier")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Multiplier::WTauNafMultiplier {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
