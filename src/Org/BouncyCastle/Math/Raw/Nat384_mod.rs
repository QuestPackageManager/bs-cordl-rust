#[cfg(feature = "Org+BouncyCastle+Math+Raw+Nat384")]
#[repr(C)]
#[derive(Debug)]
pub struct Nat384 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Math+Raw+Nat384")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Math::Raw::Nat384 =>
    "Org.BouncyCastle.Math.Raw"."Nat384"
);
#[cfg(feature = "Org+BouncyCastle+Math+Raw+Nat384")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::Raw::Nat384 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Raw+Nat384")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Math::Raw::Nat384 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Raw+Nat384")]
impl crate::Org::BouncyCastle::Math::Raw::Nat384 {
    pub fn Mul(
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zz: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Mul", (x, y, zz))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Square(
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zz: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Square", (x, zz))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Raw+Nat384")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Math::Raw::Nat384 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
