#[cfg(feature = "Org+BouncyCastle+Crypto+ICipherBuilderWithKey")]
#[repr(C)]
#[derive(Debug)]
pub struct ICipherBuilderWithKey {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+ICipherBuilderWithKey")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::ICipherBuilderWithKey {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto";
    const CLASS_NAME: &'static str = "ICipherBuilderWithKey";
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
#[cfg(feature = "Org+BouncyCastle+Crypto+ICipherBuilderWithKey")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::ICipherBuilderWithKey {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+ICipherBuilderWithKey")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::ICipherBuilderWithKey {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+ICipherBuilderWithKey")]
impl crate::Org::BouncyCastle::Crypto::ICipherBuilderWithKey {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_Key(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::ICipherParameters>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::ICipherParameters,
                        >,
                        0usize,
                    >("get_Key")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "get_Key",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherParameters,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+ICipherBuilderWithKey")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::ICipherBuilderWithKey {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+ICipherBuilderWithKey")]
impl AsRef<crate::Org::BouncyCastle::Crypto::ICipherBuilder>
for crate::Org::BouncyCastle::Crypto::ICipherBuilderWithKey {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::ICipherBuilder {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+ICipherBuilderWithKey")]
impl AsMut<crate::Org::BouncyCastle::Crypto::ICipherBuilder>
for crate::Org::BouncyCastle::Crypto::ICipherBuilderWithKey {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::ICipherBuilder {
        unsafe { std::mem::transmute(self) }
    }
}
