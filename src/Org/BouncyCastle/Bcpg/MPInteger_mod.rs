#[cfg(feature = "Org+BouncyCastle+Bcpg+MPInteger")]
#[repr(C)]
#[derive(Debug)]
pub struct MPInteger {
    __cordl_parent: crate::Org::BouncyCastle::Bcpg::BcpgObject,
    pub val: *mut crate::Org::BouncyCastle::Math::BigInteger,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+MPInteger")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Bcpg::MPInteger =>
    "Org.BouncyCastle.Bcpg"."MPInteger"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+MPInteger")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::MPInteger {
    type Target = crate::Org::BouncyCastle::Bcpg::BcpgObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+MPInteger")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Bcpg::MPInteger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+MPInteger")]
impl crate::Org::BouncyCastle::Bcpg::MPInteger {
    pub fn Encode(
        &mut self,
        bcpgOut: *mut crate::Org::BouncyCastle::Bcpg::BcpgOutputStream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Encode", (bcpgOut))?;
        Ok(__cordl_ret)
    }
    pub fn New_BcpgInputStream0(
        bcpgIn: *mut crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bcpgIn))?;
        Ok(__cordl_object)
    }
    pub fn New_BigInteger1(
        val: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (val))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_BcpgInputStream0(
        &mut self,
        bcpgIn: *mut crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bcpgIn))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_BigInteger1(
        &mut self,
        val: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (val))?;
        Ok(__cordl_ret)
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_Value", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+MPInteger")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Bcpg::MPInteger {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}