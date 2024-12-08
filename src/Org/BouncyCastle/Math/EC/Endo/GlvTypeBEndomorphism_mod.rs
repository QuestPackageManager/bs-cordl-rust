#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvTypeBEndomorphism")]
#[repr(C)]
#[derive(Debug)]
pub struct GlvTypeBEndomorphism {
    __cordl_parent: crate::System::Object,
    pub m_parameters: *mut crate::Org::BouncyCastle::Math::EC::Endo::GlvTypeBParameters,
    pub m_pointMap: *mut crate::Org::BouncyCastle::Math::EC::ECPointMap,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvTypeBEndomorphism")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Endo::GlvTypeBEndomorphism =>
    "Org.BouncyCastle.Math.EC.Endo"."GlvTypeBEndomorphism"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvTypeBEndomorphism")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::Endo::GlvTypeBEndomorphism {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvTypeBEndomorphism")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Endo::GlvTypeBEndomorphism {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvTypeBEndomorphism")]
impl crate::Org::BouncyCastle::Math::EC::Endo::GlvTypeBEndomorphism {
    pub fn DecomposeScalar(
        &mut self,
        k: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Math::BigInteger,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("DecomposeScalar", (k))?;
        Ok(__cordl_ret)
    }
    pub fn get_PointMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECPointMap,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECPointMap = __cordl_object
            .invoke("get_PointMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        curve: *mut crate::Org::BouncyCastle::Math::EC::ECCurve,
        parameters: *mut crate::Org::BouncyCastle::Math::EC::Endo::GlvTypeBParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (curve, parameters))?;
        Ok(__cordl_ret)
    }
    pub fn get_HasEfficientPointMap(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasEfficientPointMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        curve: *mut crate::Org::BouncyCastle::Math::EC::ECCurve,
        parameters: *mut crate::Org::BouncyCastle::Math::EC::Endo::GlvTypeBParameters,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (curve, parameters))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvTypeBEndomorphism")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Endo::GlvTypeBEndomorphism {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
