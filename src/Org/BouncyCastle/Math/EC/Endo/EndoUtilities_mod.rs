#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+EndoUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct EndoUtilities {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+EndoUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Math::EC::Endo::EndoUtilities
    => "Org.BouncyCastle.Math.EC.Endo"."EndoUtilities"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+EndoUtilities")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::Endo::EndoUtilities {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+EndoUtilities")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Math::EC::Endo::EndoUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+EndoUtilities")]
impl crate::Org::BouncyCastle::Math::EC::Endo::EndoUtilities {
    #[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+EndoUtilities+MapPointCallback")]
    pub type MapPointCallback = crate::Org::BouncyCastle::Math::EC::Endo::EndoUtilities_MapPointCallback;
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+EndoUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Endo::EndoUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+EndoUtilities+MapPointCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct EndoUtilities_MapPointCallback {
    __cordl_parent: crate::System::Object,
    pub m_endomorphism: *mut crate::Org::BouncyCastle::Math::EC::Endo::ECEndomorphism,
    pub m_point: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+EndoUtilities+MapPointCallback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Endo::EndoUtilities_MapPointCallback =>
    "Org.BouncyCastle.Math.EC.Endo"."EndoUtilities/MapPointCallback"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+EndoUtilities+MapPointCallback")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::EC::Endo::EndoUtilities_MapPointCallback {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+EndoUtilities+MapPointCallback")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Endo::EndoUtilities_MapPointCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+EndoUtilities+MapPointCallback")]
impl crate::Org::BouncyCastle::Math::EC::Endo::EndoUtilities_MapPointCallback {
    pub fn CheckExisting(
        &mut self,
        existingEndo: *mut crate::Org::BouncyCastle::Math::EC::Endo::EndoPreCompInfo,
        endomorphism: *mut crate::Org::BouncyCastle::Math::EC::Endo::ECEndomorphism,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CheckExisting", (existingEndo, endomorphism))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        endomorphism: *mut crate::Org::BouncyCastle::Math::EC::Endo::ECEndomorphism,
        point: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (endomorphism, point))?;
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
        endomorphism: *mut crate::Org::BouncyCastle::Math::EC::Endo::ECEndomorphism,
        point: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (endomorphism, point))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+EndoUtilities+MapPointCallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Endo::EndoUtilities_MapPointCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
