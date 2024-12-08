#[cfg(feature = "Org+BouncyCastle+Crypto+AsymmetricKeyParameter")]
#[repr(C)]
#[derive(Debug)]
pub struct AsymmetricKeyParameter {
    __cordl_parent: crate::System::Object,
    pub privateKey: bool,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+AsymmetricKeyParameter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::AsymmetricKeyParameter => "Org.BouncyCastle.Crypto"
    ."AsymmetricKeyParameter"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+AsymmetricKeyParameter")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+AsymmetricKeyParameter")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+AsymmetricKeyParameter")]
impl crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter {
    pub fn Equals_AsymmetricKeyParameter1(
        &mut self,
        other: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object0(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(privateKey: bool) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (privateKey))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        privateKey: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (privateKey))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsPrivate(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsPrivate", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+AsymmetricKeyParameter")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}