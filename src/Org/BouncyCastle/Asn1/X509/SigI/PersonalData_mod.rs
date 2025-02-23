#[cfg(feature = "Org+BouncyCastle+Asn1+X509+SigI+PersonalData")]
#[repr(C)]
#[derive(Debug)]
pub struct PersonalData {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub nameOrPseudonym: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::SigI::NameOrPseudonym,
    >,
    pub nameDistinguisher: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Math::BigInteger,
    >,
    pub dateOfBirth: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
    >,
    pub placeOfBirth: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
    >,
    pub gender: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub postalAddress: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+SigI+PersonalData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::X509::SigI::PersonalData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.X509.SigI";
    const CLASS_NAME: &'static str = "PersonalData";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+SigI+PersonalData")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::X509::SigI::PersonalData {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+SigI+PersonalData")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::X509::SigI::PersonalData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+SigI+PersonalData")]
impl crate::Org::BouncyCastle::Asn1::X509::SigI::PersonalData {
    pub fn GetInstance(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::SigI::PersonalData,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::X509::SigI::PersonalData,
                >,
                1usize,
            >("GetInstance")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetInstance", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::SigI::PersonalData,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn New_Asn1Sequence0(
        seq: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Sequence>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object.into())
    }
    pub fn New_NameOrPseudonym_BigInteger_DerGeneralizedTime_DirectoryString_Il2CppString_DirectoryString1(
        nameOrPseudonym: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::SigI::NameOrPseudonym,
        >,
        nameDistinguisher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        >,
        dateOfBirth: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
        >,
        placeOfBirth: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
        >,
        gender: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        postalAddress: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    nameOrPseudonym,
                    nameDistinguisher,
                    dateOfBirth,
                    placeOfBirth,
                    gender,
                    postalAddress,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn ToAsn1Object(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Object>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Object>,
                0usize,
            >("ToAsn1Object")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToAsn1Object", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Object,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Asn1Sequence0(
        &mut self,
        seq: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Sequence>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::Asn1Sequence,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (seq))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_NameOrPseudonym_BigInteger_DerGeneralizedTime_DirectoryString_Il2CppString_DirectoryString1(
        &mut self,
        nameOrPseudonym: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::SigI::NameOrPseudonym,
        >,
        nameDistinguisher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        >,
        dateOfBirth: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
        >,
        placeOfBirth: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
        >,
        gender: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        postalAddress: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::X509::SigI::NameOrPseudonym,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Math::BigInteger,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        nameOrPseudonym,
                        nameDistinguisher,
                        dateOfBirth,
                        placeOfBirth,
                        gender,
                        postalAddress,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_DateOfBirth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerGeneralizedTime>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
                >,
                0usize,
            >("get_DateOfBirth")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_DateOfBirth", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_Gender(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_Gender")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Gender", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_NameDistinguisher(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
                0usize,
            >("get_NameDistinguisher")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_NameDistinguisher", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_NameOrPseudonym(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::SigI::NameOrPseudonym,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::X509::SigI::NameOrPseudonym,
                >,
                0usize,
            >("get_NameOrPseudonym")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_NameOrPseudonym", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::SigI::NameOrPseudonym,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_PlaceOfBirth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X500::DirectoryString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
                >,
                0usize,
            >("get_PlaceOfBirth")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_PlaceOfBirth", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_PostalAddress(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X500::DirectoryString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
                >,
                0usize,
            >("get_PostalAddress")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_PostalAddress", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+SigI+PersonalData")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::X509::SigI::PersonalData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
