#[cfg(feature = "cordl_class_Org+BouncyCastle+Tsp+TimeStampResponseGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct TimeStampResponseGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub status: crate::Org::BouncyCastle::Asn1::Cmp::PkiStatus,
    pub statusStrings: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Asn1EncodableVector,
    >,
    pub failInfo: i32,
    pub tokenGenerator: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Tsp::TimeStampTokenGenerator,
    >,
    pub acceptedAlgorithms: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    pub acceptedPolicies: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    pub acceptedExtensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Tsp+TimeStampResponseGenerator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Tsp::TimeStampResponseGenerator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Tsp";
    const CLASS_NAME: &'static str = "TimeStampResponseGenerator";
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
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampResponseGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Tsp::TimeStampResponseGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampResponseGenerator")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Tsp::TimeStampResponseGenerator {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampResponseGenerator")]
impl crate::Org::BouncyCastle::Tsp::TimeStampResponseGenerator {
    #[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampResponseGenerator+FailInfo")]
    pub type FailInfo = crate::Org::BouncyCastle::Tsp::TimeStampResponseGenerator_FailInfo;
    pub fn AddStatusString(
        &mut self,
        statusString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("AddStatusString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddStatusString", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (statusString))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateFailResponse(
        &mut self,
        status: crate::Org::BouncyCastle::Asn1::Cmp::PkiStatus,
        failInfoField: i32,
        statusString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Tsp::TimeStampResponse>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Org::BouncyCastle::Asn1::Cmp::PkiStatus,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Tsp::TimeStampResponse,
                        >,
                        3usize,
                    >("GenerateFailResponse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GenerateFailResponse", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Tsp::TimeStampResponse,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (status, failInfoField, statusString))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Generate_DateTime0(
        &mut self,
        request: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Tsp::TimeStampRequest,
        >,
        serialNumber: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        >,
        genTime: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Tsp::TimeStampResponse>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Tsp::TimeStampRequest,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Math::BigInteger,
                            >,
                            crate::System::DateTime,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Tsp::TimeStampResponse,
                        >,
                        3usize,
                    >("Generate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Generate", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Tsp::TimeStampResponse,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (request, serialNumber, genTime))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Generate_DateTimeObject1(
        &mut self,
        request: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Tsp::TimeStampRequest,
        >,
        serialNumber: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        >,
        genTime: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Date::DateTimeObject,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Tsp::TimeStampResponse>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Tsp::TimeStampRequest,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Math::BigInteger,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Utilities::Date::DateTimeObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Tsp::TimeStampResponse,
                        >,
                        3usize,
                    >("Generate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Generate", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Tsp::TimeStampResponse,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (request, serialNumber, genTime))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPkiStatusInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiStatusInfo>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::Cmp::PkiStatusInfo,
                        >,
                        0usize,
                    >("GetPkiStatusInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPkiStatusInfo", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiStatusInfo,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New_IList1(
        tokenGenerator: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Tsp::TimeStampTokenGenerator,
        >,
        acceptedAlgorithms: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        acceptedPolicy: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tokenGenerator, acceptedAlgorithms, acceptedPolicy))?;
        Ok(__cordl_object.into())
    }
    pub fn New_IList_IList2(
        tokenGenerator: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Tsp::TimeStampTokenGenerator,
        >,
        acceptedAlgorithms: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        acceptedPolicies: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        acceptedExtensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    tokenGenerator,
                    acceptedAlgorithms,
                    acceptedPolicies,
                    acceptedExtensions,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_TimeStampTokenGenerator_IList0(
        tokenGenerator: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Tsp::TimeStampTokenGenerator,
        >,
        acceptedAlgorithms: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tokenGenerator, acceptedAlgorithms))?;
        Ok(__cordl_object.into())
    }
    pub fn SetFailInfoField(
        &mut self,
        field: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetFailInfoField")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetFailInfoField", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (field))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IList1(
        &mut self,
        tokenGenerator: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Tsp::TimeStampTokenGenerator,
        >,
        acceptedAlgorithms: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        acceptedPolicy: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Tsp::TimeStampTokenGenerator,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                            quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (tokenGenerator, acceptedAlgorithms, acceptedPolicy),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IList_IList2(
        &mut self,
        tokenGenerator: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Tsp::TimeStampTokenGenerator,
        >,
        acceptedAlgorithms: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        acceptedPolicies: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        acceptedExtensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Tsp::TimeStampTokenGenerator,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                            quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                            quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        tokenGenerator,
                        acceptedAlgorithms,
                        acceptedPolicies,
                        acceptedExtensions,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TimeStampTokenGenerator_IList0(
        &mut self,
        tokenGenerator: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Tsp::TimeStampTokenGenerator,
        >,
        acceptedAlgorithms: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Tsp::TimeStampTokenGenerator,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (tokenGenerator, acceptedAlgorithms))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Tsp+TimeStampResponseGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Tsp::TimeStampResponseGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Tsp+TimeStampResponseGenerator+FailInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct TimeStampResponseGenerator_FailInfo {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::DerBitString,
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Tsp+TimeStampResponseGenerator+FailInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Tsp::TimeStampResponseGenerator_FailInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Tsp";
    const CLASS_NAME: &'static str = "TimeStampResponseGenerator/FailInfo";
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
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampResponseGenerator+FailInfo")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Tsp::TimeStampResponseGenerator_FailInfo {
    type Target = crate::Org::BouncyCastle::Asn1::DerBitString;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampResponseGenerator+FailInfo")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Tsp::TimeStampResponseGenerator_FailInfo {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampResponseGenerator+FailInfo")]
impl crate::Org::BouncyCastle::Tsp::TimeStampResponseGenerator_FailInfo {
    pub fn New(
        failInfoValue: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (failInfoValue))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        failInfoValue: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (failInfoValue))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Tsp+TimeStampResponseGenerator+FailInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Tsp::TimeStampResponseGenerator_FailInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
