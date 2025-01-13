#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WTauNafMultiplier")]
#[repr(C)]
#[derive(Debug)]
pub struct WTauNafMultiplier {
    __cordl_parent: crate::Org::BouncyCastle::Math::EC::Multiplier::AbstractECMultiplier,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WTauNafMultiplier")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Math::EC::Multiplier::WTauNafMultiplier {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Math.EC.Multiplier";
    const CLASS_NAME: &'static str = "WTauNafMultiplier";
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
    pub fn MultiplyFromWTnaf(
        p: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint,
        >,
        u: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MultiplyFromWTnaf", (p, u))?;
        Ok(__cordl_ret.into())
    }
    pub fn MultiplyPositive(
        &mut self,
        point: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        k: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = __cordl_object.invoke("MultiplyPositive", (point, k))?;
        Ok(__cordl_ret.into())
    }
    pub fn MultiplyWTnaf(
        &mut self,
        p: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint,
        >,
        lambda: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Abc::ZTauElement,
        >,
        a: i8,
        mu: i8,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint,
        > = __cordl_object.invoke("MultiplyWTnaf", (p, lambda, a, mu))?;
        Ok(__cordl_ret.into())
    }
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WTauNafMultiplier+WTauNafCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct WTauNafMultiplier_WTauNafCallback {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_p: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint,
    >,
    pub m_a: i8,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WTauNafMultiplier+WTauNafCallback")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Math::EC::Multiplier::WTauNafMultiplier_WTauNafCallback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Math.EC.Multiplier";
    const CLASS_NAME: &'static str = "WTauNafMultiplier/WTauNafCallback";
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WTauNafMultiplier+WTauNafCallback")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::EC::Multiplier::WTauNafMultiplier_WTauNafCallback {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        p: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint,
        >,
        a: i8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (p, a))?;
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
        p: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint,
        >,
        a: i8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (p, a))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WTauNafMultiplier+WTauNafCallback")]
impl AsRef<crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback>
for crate::Org::BouncyCastle::Math::EC::Multiplier::WTauNafMultiplier_WTauNafCallback {
    fn as_ref(
        &self,
    ) -> &crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WTauNafMultiplier+WTauNafCallback")]
impl AsMut<crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback>
for crate::Org::BouncyCastle::Math::EC::Multiplier::WTauNafMultiplier_WTauNafCallback {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback {
        unsafe { std::mem::transmute(self) }
    }
}
