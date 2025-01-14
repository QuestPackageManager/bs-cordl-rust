#[cfg(feature = "UnityEngine+WaitForSeconds")]
#[repr(C)]
#[derive(Debug)]
pub struct WaitForSeconds {
    __cordl_parent: crate::UnityEngine::YieldInstruction,
    pub m_Seconds: f32,
}
#[cfg(feature = "UnityEngine+WaitForSeconds")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::WaitForSeconds {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "WaitForSeconds";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f32), quest_hook::libil2cpp::Void, 1usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (seconds))
        };
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
