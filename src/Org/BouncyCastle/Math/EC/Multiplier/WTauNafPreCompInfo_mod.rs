#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WTauNafPreCompInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct WTauNafPreCompInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_preComp: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint,
            >,
        >,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WTauNafPreCompInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Multiplier::WTauNafPreCompInfo =>
    "Org.BouncyCastle.Math.EC.Multiplier"."WTauNafPreCompInfo"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WTauNafPreCompInfo")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::EC::Multiplier::WTauNafPreCompInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WTauNafPreCompInfo")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Multiplier::WTauNafPreCompInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WTauNafPreCompInfo")]
impl crate::Org::BouncyCastle::Math::EC::Multiplier::WTauNafPreCompInfo {
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
    pub fn get_PreComp(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint,
                >,
            >,
        > = __cordl_object.invoke("get_PreComp", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_PreComp(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PreComp", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WTauNafPreCompInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Multiplier::WTauNafPreCompInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WTauNafPreCompInfo")]
impl AsRef<crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo>
for crate::Org::BouncyCastle::Math::EC::Multiplier::WTauNafPreCompInfo {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WTauNafPreCompInfo")]
impl AsMut<crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo>
for crate::Org::BouncyCastle::Math::EC::Multiplier::WTauNafPreCompInfo {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo {
        unsafe { std::mem::transmute(self) }
    }
}
