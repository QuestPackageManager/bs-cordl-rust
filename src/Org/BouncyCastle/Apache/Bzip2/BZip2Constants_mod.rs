#[cfg(feature = "Org+BouncyCastle+Apache+Bzip2+BZip2Constants")]
#[repr(C)]
#[derive(Debug)]
pub struct BZip2Constants {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Apache+Bzip2+BZip2Constants")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Apache::Bzip2::BZip2Constants {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Apache.Bzip2";
    const CLASS_NAME: &'static str = "BZip2Constants";
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
#[cfg(feature = "Org+BouncyCastle+Apache+Bzip2+BZip2Constants")]
impl std::ops::Deref for crate::Org::BouncyCastle::Apache::Bzip2::BZip2Constants {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Apache+Bzip2+BZip2Constants")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Apache::Bzip2::BZip2Constants {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Apache+Bzip2+BZip2Constants")]
impl crate::Org::BouncyCastle::Apache::Bzip2::BZip2Constants {
    pub const G_SIZE: i32 = 50i32;
    pub const MAX_ALPHA_SIZE: i32 = 258i32;
    pub const MAX_CODE_LEN: i32 = 23i32;
    pub const MAX_SELECTORS: i32 = 18002i32;
    pub const NUM_OVERSHOOT_BYTES: i32 = 20i32;
    pub const N_GROUPS: i32 = 6i32;
    pub const N_ITERS: i32 = 4i32;
    pub const RUNA: i32 = 0i32;
    pub const RUNB: i32 = 1i32;
    pub const baseBlockSize: i32 = 100000i32;
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Apache+Bzip2+BZip2Constants")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Apache::Bzip2::BZip2Constants {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
