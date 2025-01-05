#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+ValidityPreCompInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct ValidityPreCompInfo {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub failed: bool,
    pub curveEquationPassed: bool,
    pub orderPassed: bool,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+ValidityPreCompInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Multiplier::ValidityPreCompInfo =>
    "Org.BouncyCastle.Math.EC.Multiplier"."ValidityPreCompInfo"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+ValidityPreCompInfo")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::EC::Multiplier::ValidityPreCompInfo {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+ValidityPreCompInfo")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Multiplier::ValidityPreCompInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+ValidityPreCompInfo")]
impl crate::Org::BouncyCastle::Math::EC::Multiplier::ValidityPreCompInfo {
    pub fn HasCurveEquationPassed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasCurveEquationPassed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasFailed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasFailed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasOrderPassed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasOrderPassed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ReportCurveEquationPassed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReportCurveEquationPassed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReportFailed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReportFailed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReportOrderPassed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReportOrderPassed", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+ValidityPreCompInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Multiplier::ValidityPreCompInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+ValidityPreCompInfo")]
impl AsRef<
    quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
    >,
> for crate::Org::BouncyCastle::Math::EC::Multiplier::ValidityPreCompInfo {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+ValidityPreCompInfo")]
impl AsMut<
    quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
    >,
> for crate::Org::BouncyCastle::Math::EC::Multiplier::ValidityPreCompInfo {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
