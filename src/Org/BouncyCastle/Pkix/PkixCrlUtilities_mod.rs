#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCrlUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct PkixCrlUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCrlUtilities")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Pkix::PkixCrlUtilities {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Pkix";
    const CLASS_NAME: &'static str = "PkixCrlUtilities";
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
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCrlUtilities")]
impl std::ops::Deref for crate::Org::BouncyCastle::Pkix::PkixCrlUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCrlUtilities")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Pkix::PkixCrlUtilities {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCrlUtilities")]
impl crate::Org::BouncyCastle::Pkix::PkixCrlUtilities {
    pub fn FindCrls_IList2(
        &mut self,
        crlSelect: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::X509CrlStoreSelector,
        >,
        crlStores: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::X509::Store::X509CrlStoreSelector,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::ICollection,
                        >,
                        2usize,
                    >("FindCrls")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FindCrls", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ICollection,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (crlSelect, crlStores))? };
        Ok(__cordl_ret.into())
    }
    pub fn FindCrls_PkixParameters1(
        &mut self,
        crlselect: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::X509CrlStoreSelector,
        >,
        paramsPkix: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Utilities::Collections::ISet>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::X509::Store::X509CrlStoreSelector,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixParameters,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Utilities::Collections::ISet,
                        >,
                        2usize,
                    >("FindCrls")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FindCrls", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (crlselect, paramsPkix))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindCrls_PkixParameters_DateTime0(
        &mut self,
        crlselect: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::X509CrlStoreSelector,
        >,
        paramsPkix: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
        currentDate: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Utilities::Collections::ISet>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::X509::Store::X509CrlStoreSelector,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixParameters,
                            >,
                            crate::System::DateTime,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Utilities::Collections::ISet,
                        >,
                        3usize,
                    >("FindCrls")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FindCrls", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (crlselect, paramsPkix, currentDate))?
        };
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
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCrlUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Pkix::PkixCrlUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
