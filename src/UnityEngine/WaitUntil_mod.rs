#[cfg(feature = "UnityEngine+WaitUntil")]
#[repr(C)]
#[derive(Debug)]
pub struct WaitUntil {
    __cordl_parent: crate::UnityEngine::CustomYieldInstruction,
    pub m_Predicate: quest_hook::libil2cpp::Gc<crate::System::Func_1<bool>>,
}
#[cfg(feature = "UnityEngine+WaitUntil")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::WaitUntil {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "WaitUntil";
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
#[cfg(feature = "UnityEngine+WaitUntil")]
impl std::ops::Deref for crate::UnityEngine::WaitUntil {
    type Target = crate::UnityEngine::CustomYieldInstruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+WaitUntil")]
impl std::ops::DerefMut for crate::UnityEngine::WaitUntil {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+WaitUntil")]
impl crate::UnityEngine::WaitUntil {
    pub fn New(
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_1<bool>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (predicate))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_1<bool>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_keepWaiting(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_keepWaiting", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+WaitUntil")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::WaitUntil {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
