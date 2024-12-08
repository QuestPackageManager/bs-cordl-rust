#[cfg(feature = "Oculus+Platform+Samples+EntitlementCheck+EntitlementCheck")]
#[repr(C)]
#[derive(Debug)]
pub struct EntitlementCheck {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub exitAppOnFailure: bool,
}
#[cfg(feature = "Oculus+Platform+Samples+EntitlementCheck+EntitlementCheck")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Oculus::Platform::Samples::EntitlementCheck::EntitlementCheck =>
    "Oculus.Platform.Samples.EntitlementCheck"."EntitlementCheck"
);
#[cfg(feature = "Oculus+Platform+Samples+EntitlementCheck+EntitlementCheck")]
impl std::ops::Deref
for crate::Oculus::Platform::Samples::EntitlementCheck::EntitlementCheck {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Samples+EntitlementCheck+EntitlementCheck")]
impl std::ops::DerefMut
for crate::Oculus::Platform::Samples::EntitlementCheck::EntitlementCheck {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Samples+EntitlementCheck+EntitlementCheck")]
impl crate::Oculus::Platform::Samples::EntitlementCheck::EntitlementCheck {
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn EntitlementCheckCallback(
        &mut self,
        msg: *mut crate::Oculus::Platform::Message,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EntitlementCheckCallback", (msg))?;
        Ok(__cordl_ret)
    }
    pub fn HandleEntitlementCheckResult(
        &mut self,
        result: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleEntitlementCheckResult", (result))?;
        Ok(__cordl_ret)
    }
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
#[cfg(feature = "Oculus+Platform+Samples+EntitlementCheck+EntitlementCheck")]
impl quest_hook::libil2cpp::ObjectType
for crate::Oculus::Platform::Samples::EntitlementCheck::EntitlementCheck {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
