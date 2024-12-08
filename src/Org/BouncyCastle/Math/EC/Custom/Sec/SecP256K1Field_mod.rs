#[cfg(feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP256K1Field")]
#[repr(C)]
#[derive(Debug)]
pub struct SecP256K1Field {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP256K1Field")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Custom::Sec::SecP256K1Field =>
    "Org.BouncyCastle.Math.EC.Custom.Sec"."SecP256K1Field"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP256K1Field")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP256K1Field {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP256K1Field")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP256K1Field {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP256K1Field")]
impl crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP256K1Field {
    pub const P7: u32 = 3515088895u32;
    pub const PExt15: u32 = 80839679u32;
    pub const PInv33: u32 = 268751235u32;
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP256K1Field")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP256K1Field {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
