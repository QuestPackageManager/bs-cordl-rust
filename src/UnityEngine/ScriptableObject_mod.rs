#[cfg(feature = "UnityEngine+ScriptableObject")]
#[repr(C)]
#[derive(Debug)]
pub struct ScriptableObject {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
}
#[cfg(feature = "UnityEngine+ScriptableObject")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ScriptableObject => "UnityEngine"
    ."ScriptableObject"
);
#[cfg(feature = "UnityEngine+ScriptableObject")]
impl std::ops::Deref for crate::UnityEngine::ScriptableObject {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ScriptableObject")]
impl std::ops::DerefMut for crate::UnityEngine::ScriptableObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ScriptableObject")]
impl crate::UnityEngine::ScriptableObject {
    pub fn CreateInstance_1<T>() -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateInstance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInstance_Gc0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ScriptableObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateInstance", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateScriptableObject(
        _cordl_self: quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateScriptableObject", (_cordl_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateScriptableObjectInstanceFromType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        applyDefaultsAndReset: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ScriptableObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateScriptableObjectInstanceFromType",
                (_cordl_type, applyDefaultsAndReset),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "UnityEngine+ScriptableObject")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ScriptableObject {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
