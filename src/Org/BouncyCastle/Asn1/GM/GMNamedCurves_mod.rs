#[cfg(feature = "Org+BouncyCastle+Asn1+GM+GMNamedCurves")]
#[repr(C)]
#[derive(Debug)]
pub struct GMNamedCurves {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+GM+GMNamedCurves")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::GM::GMNamedCurves =>
    "Org.BouncyCastle.Asn1.GM"."GMNamedCurves"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+GM+GMNamedCurves")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::GM::GMNamedCurves {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+GM+GMNamedCurves")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::GM::GMNamedCurves {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+GM+GMNamedCurves")]
impl crate::Org::BouncyCastle::Asn1::GM::GMNamedCurves {
    #[cfg(feature = "Org+BouncyCastle+Asn1+GM+GMNamedCurves+SM2P256V1Holder")]
    pub type SM2P256V1Holder = crate::Org::BouncyCastle::Asn1::GM::GMNamedCurves_SM2P256V1Holder;
    #[cfg(feature = "Org+BouncyCastle+Asn1+GM+GMNamedCurves+WapiP192V1Holder")]
    pub type WapiP192V1Holder = crate::Org::BouncyCastle::Asn1::GM::GMNamedCurves_WapiP192V1Holder;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+GM+GMNamedCurves")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::GM::GMNamedCurves {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+GM+GMNamedCurves+SM2P256V1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct GMNamedCurves_SM2P256V1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+GM+GMNamedCurves+SM2P256V1Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::GM::GMNamedCurves_SM2P256V1Holder =>
    "Org.BouncyCastle.Asn1.GM"."GMNamedCurves/SM2P256V1Holder"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+GM+GMNamedCurves+SM2P256V1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::GM::GMNamedCurves_SM2P256V1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+GM+GMNamedCurves+SM2P256V1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::GM::GMNamedCurves_SM2P256V1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+GM+GMNamedCurves+SM2P256V1Holder")]
impl crate::Org::BouncyCastle::Asn1::GM::GMNamedCurves_SM2P256V1Holder {
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters = __cordl_object
            .invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+GM+GMNamedCurves+SM2P256V1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::GM::GMNamedCurves_SM2P256V1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+GM+GMNamedCurves+WapiP192V1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct GMNamedCurves_WapiP192V1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+GM+GMNamedCurves+WapiP192V1Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::GM::GMNamedCurves_WapiP192V1Holder =>
    "Org.BouncyCastle.Asn1.GM"."GMNamedCurves/WapiP192V1Holder"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+GM+GMNamedCurves+WapiP192V1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::GM::GMNamedCurves_WapiP192V1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+GM+GMNamedCurves+WapiP192V1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::GM::GMNamedCurves_WapiP192V1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+GM+GMNamedCurves+WapiP192V1Holder")]
impl crate::Org::BouncyCastle::Asn1::GM::GMNamedCurves_WapiP192V1Holder {
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters = __cordl_object
            .invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+GM+GMNamedCurves+WapiP192V1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::GM::GMNamedCurves_WapiP192V1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
