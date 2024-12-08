#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+DHParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct DHParameters {
    __cordl_parent: crate::System::Object,
    pub p: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub g: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub q: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub j: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub m: i32,
    pub l: i32,
    pub validation: *mut crate::Org::BouncyCastle::Crypto::Parameters::DHValidationParameters,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+DHParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Parameters::DHParameters =>
    "Org.BouncyCastle.Crypto.Parameters"."DHParameters"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+DHParameters")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Parameters::DHParameters {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+DHParameters")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Parameters::DHParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+DHParameters")]
impl crate::Org::BouncyCastle::Crypto::Parameters::DHParameters {
    pub const DefaultMinimumLength: i32 = 160i32;
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_G(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_G", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_P(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_P", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Q(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_Q", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_J(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_J", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_L(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_L", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_M(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_M", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ValidationParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Parameters::DHValidationParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Parameters::DHValidationParameters = __cordl_object
            .invoke("get_ValidationParameters", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_BigInteger_BigInteger0(
        &mut self,
        p: *mut crate::Org::BouncyCastle::Math::BigInteger,
        g: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (p, g))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_BigInteger1(
        &mut self,
        p: *mut crate::Org::BouncyCastle::Math::BigInteger,
        g: *mut crate::Org::BouncyCastle::Math::BigInteger,
        q: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (p, g, q))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_BigInteger_i32_2(
        &mut self,
        p: *mut crate::Org::BouncyCastle::Math::BigInteger,
        g: *mut crate::Org::BouncyCastle::Math::BigInteger,
        q: *mut crate::Org::BouncyCastle::Math::BigInteger,
        l: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (p, g, q, l))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_BigInteger_i32_i32_3(
        &mut self,
        p: *mut crate::Org::BouncyCastle::Math::BigInteger,
        g: *mut crate::Org::BouncyCastle::Math::BigInteger,
        q: *mut crate::Org::BouncyCastle::Math::BigInteger,
        m: i32,
        l: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (p, g, q, m, l))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_BigInteger_BigInteger_DHValidationParameters4(
        &mut self,
        p: *mut crate::Org::BouncyCastle::Math::BigInteger,
        g: *mut crate::Org::BouncyCastle::Math::BigInteger,
        q: *mut crate::Org::BouncyCastle::Math::BigInteger,
        j: *mut crate::Org::BouncyCastle::Math::BigInteger,
        validation: *mut crate::Org::BouncyCastle::Crypto::Parameters::DHValidationParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (p, g, q, j, validation))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_BigInteger_i32_i32_BigInteger_DHValidationParameters5(
        &mut self,
        p: *mut crate::Org::BouncyCastle::Math::BigInteger,
        g: *mut crate::Org::BouncyCastle::Math::BigInteger,
        q: *mut crate::Org::BouncyCastle::Math::BigInteger,
        m: i32,
        l: i32,
        j: *mut crate::Org::BouncyCastle::Math::BigInteger,
        validation: *mut crate::Org::BouncyCastle::Crypto::Parameters::DHValidationParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (p, g, q, m, l, j, validation))?;
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
    pub fn Equals_DHParameters1(
        &mut self,
        other: *mut crate::Org::BouncyCastle::Crypto::Parameters::DHParameters,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
        Ok(__cordl_ret)
    }
    pub fn New_BigInteger_BigInteger0(
        p: *mut crate::Org::BouncyCastle::Math::BigInteger,
        g: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (p, g))?;
        Ok(__cordl_object)
    }
    pub fn New_BigInteger1(
        p: *mut crate::Org::BouncyCastle::Math::BigInteger,
        g: *mut crate::Org::BouncyCastle::Math::BigInteger,
        q: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (p, g, q))?;
        Ok(__cordl_object)
    }
    pub fn New_BigInteger_i32_2(
        p: *mut crate::Org::BouncyCastle::Math::BigInteger,
        g: *mut crate::Org::BouncyCastle::Math::BigInteger,
        q: *mut crate::Org::BouncyCastle::Math::BigInteger,
        l: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (p, g, q, l))?;
        Ok(__cordl_object)
    }
    pub fn New_BigInteger_i32_i32_3(
        p: *mut crate::Org::BouncyCastle::Math::BigInteger,
        g: *mut crate::Org::BouncyCastle::Math::BigInteger,
        q: *mut crate::Org::BouncyCastle::Math::BigInteger,
        m: i32,
        l: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (p, g, q, m, l))?;
        Ok(__cordl_object)
    }
    pub fn New_BigInteger_BigInteger_DHValidationParameters4(
        p: *mut crate::Org::BouncyCastle::Math::BigInteger,
        g: *mut crate::Org::BouncyCastle::Math::BigInteger,
        q: *mut crate::Org::BouncyCastle::Math::BigInteger,
        j: *mut crate::Org::BouncyCastle::Math::BigInteger,
        validation: *mut crate::Org::BouncyCastle::Crypto::Parameters::DHValidationParameters,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (p, g, q, j, validation))?;
        Ok(__cordl_object)
    }
    pub fn New_BigInteger_i32_i32_BigInteger_DHValidationParameters5(
        p: *mut crate::Org::BouncyCastle::Math::BigInteger,
        g: *mut crate::Org::BouncyCastle::Math::BigInteger,
        q: *mut crate::Org::BouncyCastle::Math::BigInteger,
        m: i32,
        l: i32,
        j: *mut crate::Org::BouncyCastle::Math::BigInteger,
        validation: *mut crate::Org::BouncyCastle::Crypto::Parameters::DHValidationParameters,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (p, g, q, m, l, j, validation))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+DHParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Parameters::DHParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
