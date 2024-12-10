#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+SXprUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct SXprUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+SXprUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities
    => "Org.BouncyCastle.Bcpg.OpenPgp"."SXprUtilities"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+SXprUtilities")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+SXprUtilities")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+SXprUtilities")]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities {
    #[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+SXprUtilities+MyS2k")]
    pub type MyS2k = crate::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities_MyS2k;
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
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+SXprUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+SXprUtilities+MyS2k")]
#[repr(C)]
#[derive(Debug)]
pub struct SXprUtilities_MyS2k {
    __cordl_parent: crate::Org::BouncyCastle::Bcpg::S2k,
    pub mIterationCount64: i64,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+SXprUtilities+MyS2k")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities_MyS2k =>
    "Org.BouncyCastle.Bcpg.OpenPgp"."SXprUtilities/MyS2k"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+SXprUtilities+MyS2k")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities_MyS2k {
    type Target = crate::Org::BouncyCastle::Bcpg::S2k;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+SXprUtilities+MyS2k")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities_MyS2k {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+SXprUtilities+MyS2k")]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities_MyS2k {
    pub fn New(
        algorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        iv: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        iterationCount64: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm, iv, iterationCount64))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        algorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        iv: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        iterationCount64: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algorithm, iv, iterationCount64))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IterationCount(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_IterationCount", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+SXprUtilities+MyS2k")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities_MyS2k {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
