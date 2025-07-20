#[cfg(feature = "Org+BouncyCastle+OpenSsl+IPasswordFinder")]
#[repr(C)]
#[derive(Debug)]
pub struct IPasswordFinder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+OpenSsl+IPasswordFinder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::OpenSsl::IPasswordFinder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.OpenSsl";
    const CLASS_NAME: &'static str = "IPasswordFinder";
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
#[cfg(feature = "Org+BouncyCastle+OpenSsl+IPasswordFinder")]
impl std::ops::Deref for crate::Org::BouncyCastle::OpenSsl::IPasswordFinder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+OpenSsl+IPasswordFinder")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::OpenSsl::IPasswordFinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+OpenSsl+IPasswordFinder")]
impl crate::Org::BouncyCastle::OpenSsl::IPasswordFinder {
    pub fn GetPassword(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::OpenSsl::IPasswordFinder as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
                0usize,
            >("GetPassword")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::OpenSsl::IPasswordFinder as
                    quest_hook::libil2cpp::Type > ::class(), "GetPassword", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<char>,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+OpenSsl+IPasswordFinder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::OpenSsl::IPasswordFinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
