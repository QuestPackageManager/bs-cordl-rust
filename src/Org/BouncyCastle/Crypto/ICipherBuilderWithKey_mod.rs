#[cfg(feature = "Org+BouncyCastle+Crypto+ICipherBuilderWithKey")]
#[repr(C)]
#[derive(Debug)]
pub struct ICipherBuilderWithKey {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+ICipherBuilderWithKey")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::ICipherBuilderWithKey
    => "Org.BouncyCastle.Crypto"."ICipherBuilderWithKey"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+ICipherBuilderWithKey")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::ICipherBuilderWithKey {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+ICipherBuilderWithKey")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::ICipherBuilderWithKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+ICipherBuilderWithKey")]
impl crate::Org::BouncyCastle::Crypto::ICipherBuilderWithKey {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_Key(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::ICipherParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherParameters,
        > = __cordl_object.invoke("get_Key", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+ICipherBuilderWithKey")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::ICipherBuilderWithKey {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+ICipherBuilderWithKey")]
impl AsRef<crate::Org::BouncyCastle::Crypto::ICipherBuilder>
for crate::Org::BouncyCastle::Crypto::ICipherBuilderWithKey {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::ICipherBuilder {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+ICipherBuilderWithKey")]
impl AsMut<crate::Org::BouncyCastle::Crypto::ICipherBuilder>
for crate::Org::BouncyCastle::Crypto::ICipherBuilderWithKey {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::ICipherBuilder {
        unsafe { std::mem::transmute(self) }
    }
}
