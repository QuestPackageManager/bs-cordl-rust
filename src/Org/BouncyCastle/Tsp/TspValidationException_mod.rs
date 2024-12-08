#[cfg(feature = "Org+BouncyCastle+Tsp+TspValidationException")]
#[repr(C)]
#[derive(Debug)]
pub struct TspValidationException {
    __cordl_parent: crate::Org::BouncyCastle::Tsp::TspException,
    pub failureCode: i32,
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TspValidationException")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Tsp::TspValidationException
    => "Org.BouncyCastle.Tsp"."TspValidationException"
);
#[cfg(feature = "Org+BouncyCastle+Tsp+TspValidationException")]
impl std::ops::Deref for crate::Org::BouncyCastle::Tsp::TspValidationException {
    type Target = crate::Org::BouncyCastle::Tsp::TspException;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TspValidationException")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Tsp::TspValidationException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TspValidationException")]
impl crate::Org::BouncyCastle::Tsp::TspValidationException {
    pub fn New_String0(
        message: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_1(
        message: *mut crate::System::String,
        failureCode: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message, failureCode))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_String0(
        &mut self,
        message: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (message))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_1(
        &mut self,
        message: *mut crate::System::String,
        failureCode: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (message, failureCode))?;
        Ok(__cordl_ret)
    }
    pub fn get_FailureCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_FailureCode", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TspValidationException")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Tsp::TspValidationException {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}