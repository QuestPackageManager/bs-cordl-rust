#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCertPathBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct PkixCertPathBuilder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub certPathException: *mut crate::System::Exception,
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCertPathBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Pkix::PkixCertPathBuilder =>
    "Org.BouncyCastle.Pkix"."PkixCertPathBuilder"
);
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCertPathBuilder")]
impl std::ops::Deref for crate::Org::BouncyCastle::Pkix::PkixCertPathBuilder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCertPathBuilder")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Pkix::PkixCertPathBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCertPathBuilder")]
impl crate::Org::BouncyCastle::Pkix::PkixCertPathBuilder {
    pub fn Build_PkixBuilderParameters0(
        &mut self,
        pkixParams: *mut crate::Org::BouncyCastle::Pkix::PkixBuilderParameters,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Pkix::PkixCertPathBuilderResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Pkix::PkixCertPathBuilderResult = __cordl_object
            .invoke("Build", (pkixParams))?;
        Ok(__cordl_ret)
    }
    pub fn Build_X509Certificate_PkixBuilderParameters_IList1(
        &mut self,
        tbvCert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
        pkixParams: *mut crate::Org::BouncyCastle::Pkix::PkixBuilderParameters,
        tbvPath: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Pkix::PkixCertPathBuilderResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Pkix::PkixCertPathBuilderResult = __cordl_object
            .invoke("Build", (tbvCert, pkixParams, tbvPath))?;
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
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCertPathBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Pkix::PkixCertPathBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
