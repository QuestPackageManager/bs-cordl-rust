#[cfg(feature = "Org+BouncyCastle+Bcpg+Sig+Features")]
#[repr(C)]
#[derive(Debug)]
pub struct Features {
    __cordl_parent: crate::Org::BouncyCastle::Bcpg::SignatureSubpacket,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+Sig+Features")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Bcpg::Sig::Features =>
    "Org.BouncyCastle.Bcpg.Sig"."Features"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+Sig+Features")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::Sig::Features {
    type Target = crate::Org::BouncyCastle::Bcpg::SignatureSubpacket;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+Sig+Features")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Bcpg::Sig::Features {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+Sig+Features")]
impl crate::Org::BouncyCastle::Bcpg::Sig::Features {
    pub fn FeatureToByteArray(
        feature: u8,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FeatureToByteArray", (feature))?;
        Ok(__cordl_ret.into())
    }
    pub fn New__cordl_bool_Il2CppArray0(
        critical: bool,
        isLongLength: bool,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (critical, isLongLength, data))?;
        Ok(__cordl_object.into())
    }
    pub fn New_u8_1(
        critical: bool,
        feature: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (critical, feature))?;
        Ok(__cordl_object.into())
    }
    pub fn SetSupportsFeature(
        &mut self,
        feature: u8,
        support: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSupportsFeature", (feature, support))?;
        Ok(__cordl_ret.into())
    }
    pub fn SupportsFeature(
        &mut self,
        feature: u8,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("SupportsFeature", (feature))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool_Il2CppArray0(
        &mut self,
        critical: bool,
        isLongLength: bool,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (critical, isLongLength, data))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u8_1(
        &mut self,
        critical: bool,
        feature: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (critical, feature))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SupportsModificationDetection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_SupportsModificationDetection", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+Sig+Features")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::Sig::Features {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
