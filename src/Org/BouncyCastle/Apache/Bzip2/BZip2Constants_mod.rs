#[cfg(feature = "Org+BouncyCastle+Apache+Bzip2+BZip2Constants")]
#[repr(C)]
#[derive(Debug)]
pub struct BZip2Constants {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Org+BouncyCastle+Apache+Bzip2+BZip2Constants")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Apache::Bzip2::BZip2Constants
    => "Org.BouncyCastle.Apache.Bzip2"."BZip2Constants"
);
#[cfg(feature = "Org+BouncyCastle+Apache+Bzip2+BZip2Constants")]
impl std::ops::Deref for crate::Org::BouncyCastle::Apache::Bzip2::BZip2Constants {
    type Target = crate::System::Object;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
