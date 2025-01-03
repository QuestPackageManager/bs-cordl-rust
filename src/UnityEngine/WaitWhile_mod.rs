#[cfg(feature = "UnityEngine+WaitWhile")]
#[repr(C)]
#[derive(Debug)]
pub struct WaitWhile {
    __cordl_parent: crate::UnityEngine::CustomYieldInstruction,
    pub m_Predicate: quest_hook::libil2cpp::Gc<crate::System::Func_1<bool>>,
}
#[cfg(feature = "UnityEngine+WaitWhile")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::WaitWhile => "UnityEngine"
    ."WaitWhile"
);
#[cfg(feature = "UnityEngine+WaitWhile")]
impl std::ops::Deref for crate::UnityEngine::WaitWhile {
    type Target = crate::UnityEngine::CustomYieldInstruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+WaitWhile")]
impl std::ops::DerefMut for crate::UnityEngine::WaitWhile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+WaitWhile")]
impl crate::UnityEngine::WaitWhile {
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
#[cfg(feature = "UnityEngine+WaitWhile")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::WaitWhile {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
