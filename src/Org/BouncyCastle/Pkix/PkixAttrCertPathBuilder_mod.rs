#[cfg(feature = "Org+BouncyCastle+Pkix+PkixAttrCertPathBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct PkixAttrCertPathBuilder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub certPathException: quest_hook::libil2cpp::Gc<crate::System::Exception>,
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixAttrCertPathBuilder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Pkix::PkixAttrCertPathBuilder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Pkix";
    const CLASS_NAME: &'static str = "PkixAttrCertPathBuilder";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixAttrCertPathBuilder")]
impl std::ops::Deref for crate::Org::BouncyCastle::Pkix::PkixAttrCertPathBuilder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixAttrCertPathBuilder")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Pkix::PkixAttrCertPathBuilder {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixAttrCertPathBuilder")]
impl crate::Org::BouncyCastle::Pkix::PkixAttrCertPathBuilder {
    pub fn Build_IX509AttributeCertificate_X509Certificate_PkixBuilderParameters_IList1(
        &mut self,
        attrCert: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::IX509AttributeCertificate,
        >,
        tbvCert: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Certificate,
        >,
        pkixParams: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixBuilderParameters,
        >,
        tbvPath: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPathBuilderResult,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::X509::IX509AttributeCertificate,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::X509::X509Certificate,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixBuilderParameters,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Pkix::PkixCertPathBuilderResult,
                        >,
                        4usize,
                    >("Build")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Build",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPathBuilderResult,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (attrCert, tbvCert, pkixParams, tbvPath))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Build_PkixBuilderParameters0(
        &mut self,
        pkixParams: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixBuilderParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPathBuilderResult,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Pkix::PkixBuilderParameters,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Pkix::PkixCertPathBuilderResult,
                        >,
                        1usize,
                    >("Build")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Build",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPathBuilderResult,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (pkixParams))? };
        Ok(__cordl_ret.into())
    }
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixAttrCertPathBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Pkix::PkixAttrCertPathBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
