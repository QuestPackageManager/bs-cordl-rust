#[cfg(feature = "Org+BouncyCastle+Pkix+PkixNameConstraintValidatorException")]
#[repr(C)]
#[derive(Debug)]
pub struct PkixNameConstraintValidatorException {
    __cordl_parent: crate::System::Exception,
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixNameConstraintValidatorException")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Pkix::PkixNameConstraintValidatorException =>
    "Org.BouncyCastle.Pkix"."PkixNameConstraintValidatorException"
);
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixNameConstraintValidatorException")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Pkix::PkixNameConstraintValidatorException {
    type Target = crate::System::Exception;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixNameConstraintValidatorException")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Pkix::PkixNameConstraintValidatorException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixNameConstraintValidatorException")]
impl crate::Org::BouncyCastle::Pkix::PkixNameConstraintValidatorException {
    pub fn _ctor(
        &mut self,
        msg: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (msg))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        msg: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (msg))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixNameConstraintValidatorException")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Pkix::PkixNameConstraintValidatorException {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
