#[cfg(feature = "Mono+Math+Prime+PrimalityTest")]
#[repr(C)]
#[derive(Debug)]
pub struct PrimalityTest {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "Mono+Math+Prime+PrimalityTest")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Math::Prime::PrimalityTest =>
    "Mono.Math.Prime"."PrimalityTest"
);
#[cfg(feature = "Mono+Math+Prime+PrimalityTest")]
impl std::ops::Deref for crate::Mono::Math::Prime::PrimalityTest {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Math+Prime+PrimalityTest")]
impl std::ops::DerefMut for crate::Mono::Math::Prime::PrimalityTest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Math+Prime+PrimalityTest")]
impl crate::Mono::Math::Prime::PrimalityTest {
    pub fn Invoke(
        &mut self,
        bi: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        confidence: crate::Mono::Math::Prime::ConfidenceFactor,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", (bi, confidence))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Math+Prime+PrimalityTest")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Math::Prime::PrimalityTest {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
