#[cfg(feature = "UnityEngine+WaitForSeconds")]
#[repr(C)]
#[derive(Debug)]
pub struct WaitForSeconds {
    __cordl_parent: crate::UnityEngine::YieldInstruction,
    pub m_Seconds: f32,
}
#[cfg(feature = "UnityEngine+WaitForSeconds")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::WaitForSeconds => "UnityEngine"
    ."WaitForSeconds"
);
#[cfg(feature = "UnityEngine+WaitForSeconds")]
impl std::ops::Deref for crate::UnityEngine::WaitForSeconds {
    type Target = crate::UnityEngine::YieldInstruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+WaitForSeconds")]
impl std::ops::DerefMut for crate::UnityEngine::WaitForSeconds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+WaitForSeconds")]
impl crate::UnityEngine::WaitForSeconds {
    pub fn New(
        seconds: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seconds))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        seconds: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (seconds))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+WaitForSeconds")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::WaitForSeconds {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
