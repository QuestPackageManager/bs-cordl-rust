#[cfg(feature = "Org+BouncyCastle+Asn1+Anssi+AnssiNamedCurves")]
#[repr(C)]
#[derive(Debug)]
pub struct AnssiNamedCurves {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Anssi+AnssiNamedCurves")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Anssi::AnssiNamedCurves
    => "Org.BouncyCastle.Asn1.Anssi"."AnssiNamedCurves"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Anssi+AnssiNamedCurves")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Anssi::AnssiNamedCurves {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Anssi+AnssiNamedCurves")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Anssi::AnssiNamedCurves {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Anssi+AnssiNamedCurves")]
impl crate::Org::BouncyCastle::Asn1::Anssi::AnssiNamedCurves {
    #[cfg(feature = "Org+BouncyCastle+Asn1+Anssi+AnssiNamedCurves+Frp256v1Holder")]
    pub type Frp256v1Holder = crate::Org::BouncyCastle::Asn1::Anssi::AnssiNamedCurves_Frp256v1Holder;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Anssi+AnssiNamedCurves")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Anssi::AnssiNamedCurves {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Anssi+AnssiNamedCurves+Frp256v1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct AnssiNamedCurves_Frp256v1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Anssi+AnssiNamedCurves+Frp256v1Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::Anssi::AnssiNamedCurves_Frp256v1Holder =>
    "Org.BouncyCastle.Asn1.Anssi"."AnssiNamedCurves/Frp256v1Holder"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Anssi+AnssiNamedCurves+Frp256v1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Anssi::AnssiNamedCurves_Frp256v1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Anssi+AnssiNamedCurves+Frp256v1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Anssi::AnssiNamedCurves_Frp256v1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Anssi+AnssiNamedCurves+Frp256v1Holder")]
impl crate::Org::BouncyCastle::Asn1::Anssi::AnssiNamedCurves_Frp256v1Holder {
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Anssi+AnssiNamedCurves+Frp256v1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Anssi::AnssiNamedCurves_Frp256v1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
