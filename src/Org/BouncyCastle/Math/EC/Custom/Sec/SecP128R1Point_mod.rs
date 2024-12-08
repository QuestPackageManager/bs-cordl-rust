#[cfg(feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP128R1Point")]
#[repr(C)]
#[derive(Debug)]
pub struct SecP128R1Point {
    __cordl_parent: crate::Org::BouncyCastle::Math::EC::AbstractFpPoint,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP128R1Point")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Custom::Sec::SecP128R1Point =>
    "Org.BouncyCastle.Math.EC.Custom.Sec"."SecP128R1Point"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP128R1Point")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP128R1Point {
    type Target = crate::Org::BouncyCastle::Math::EC::AbstractFpPoint;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP128R1Point")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP128R1Point {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP128R1Point")]
impl crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP128R1Point {
    pub fn Negate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECPoint = __cordl_object
            .invoke("Negate", ())?;
        Ok(__cordl_ret)
    }
    pub fn ThreeTimes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECPoint = __cordl_object
            .invoke("ThreeTimes", ())?;
        Ok(__cordl_ret)
    }
    pub fn TwicePlus(
        &mut self,
        b: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECPoint = __cordl_object
            .invoke("TwicePlus", (b))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ECCurve_ECFieldElement_ECFieldElement0(
        &mut self,
        curve: *mut crate::Org::BouncyCastle::Math::EC::ECCurve,
        x: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        y: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (curve, x, y))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool1(
        &mut self,
        curve: *mut crate::Org::BouncyCastle::Math::EC::ECCurve,
        x: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        y: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        withCompression: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (curve, x, y, withCompression))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray__cordl_bool2(
        &mut self,
        curve: *mut crate::Org::BouncyCastle::Math::EC::ECCurve,
        x: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        y: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        zs: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        >,
        withCompression: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (curve, x, y, zs, withCompression))?;
        Ok(__cordl_ret)
    }
    pub fn Twice(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECPoint = __cordl_object
            .invoke("Twice", ())?;
        Ok(__cordl_ret)
    }
    pub fn Add(
        &mut self,
        b: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECPoint = __cordl_object
            .invoke("Add", (b))?;
        Ok(__cordl_ret)
    }
    pub fn Detach(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECPoint = __cordl_object
            .invoke("Detach", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_ECCurve_ECFieldElement_ECFieldElement0(
        curve: *mut crate::Org::BouncyCastle::Math::EC::ECCurve,
        x: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        y: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (curve, x, y))?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool1(
        curve: *mut crate::Org::BouncyCastle::Math::EC::ECCurve,
        x: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        y: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        withCompression: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (curve, x, y, withCompression))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray__cordl_bool2(
        curve: *mut crate::Org::BouncyCastle::Math::EC::ECCurve,
        x: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        y: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        zs: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        >,
        withCompression: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (curve, x, y, zs, withCompression))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP128R1Point")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP128R1Point {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
