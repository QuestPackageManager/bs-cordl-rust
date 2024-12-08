#[cfg(feature = "Org+BouncyCastle+Math+EC+ScaleXNegateYPointMap")]
#[repr(C)]
#[derive(Debug)]
pub struct ScaleXNegateYPointMap {
    __cordl_parent: crate::System::Object,
    pub scale: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+ScaleXNegateYPointMap")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::ScaleXNegateYPointMap => "Org.BouncyCastle.Math.EC"
    ."ScaleXNegateYPointMap"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+ScaleXNegateYPointMap")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::ScaleXNegateYPointMap {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+ScaleXNegateYPointMap")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Math::EC::ScaleXNegateYPointMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+ScaleXNegateYPointMap")]
impl crate::Org::BouncyCastle::Math::EC::ScaleXNegateYPointMap {
    pub fn Map(
        &mut self,
        p: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECPoint = __cordl_object
            .invoke("Map", (p))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        scale: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (scale))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        scale: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (scale))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+ScaleXNegateYPointMap")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::ScaleXNegateYPointMap {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}