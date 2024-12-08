#[cfg(feature = "UnityEngine+RuntimeInitializeOnLoadMethodAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimeInitializeOnLoadMethodAttribute {
    __cordl_parent: crate::UnityEngine::Scripting::PreserveAttribute,
    pub m_LoadType: crate::UnityEngine::RuntimeInitializeLoadType,
}
#[cfg(feature = "UnityEngine+RuntimeInitializeOnLoadMethodAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::RuntimeInitializeOnLoadMethodAttribute => "UnityEngine"
    ."RuntimeInitializeOnLoadMethodAttribute"
);
#[cfg(feature = "UnityEngine+RuntimeInitializeOnLoadMethodAttribute")]
impl std::ops::Deref for crate::UnityEngine::RuntimeInitializeOnLoadMethodAttribute {
    type Target = crate::UnityEngine::Scripting::PreserveAttribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+RuntimeInitializeOnLoadMethodAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::RuntimeInitializeOnLoadMethodAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+RuntimeInitializeOnLoadMethodAttribute")]
impl crate::UnityEngine::RuntimeInitializeOnLoadMethodAttribute {
    pub fn _ctor(
        &mut self,
        loadType: crate::UnityEngine::RuntimeInitializeLoadType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (loadType))?;
        Ok(__cordl_ret)
    }
    pub fn set_loadType(
        &mut self,
        value: crate::UnityEngine::RuntimeInitializeLoadType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_loadType", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        loadType: crate::UnityEngine::RuntimeInitializeLoadType,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (loadType))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+RuntimeInitializeOnLoadMethodAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::RuntimeInitializeOnLoadMethodAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
