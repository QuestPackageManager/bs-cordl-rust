#[cfg(feature = "UnityEngine+UIElements+DynamicAtlasSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct DynamicAtlasSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_MinAtlasSize: i32,
    pub m_MaxAtlasSize: i32,
    pub m_MaxSubTextureSize: i32,
    pub m_ActiveFilters: crate::UnityEngine::UIElements::DynamicAtlasFilters,
    pub m_CustomFilter: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::DynamicAtlasCustomFilter,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+DynamicAtlasSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::DynamicAtlasSettings =>
    "UnityEngine.UIElements"."DynamicAtlasSettings"
);
#[cfg(feature = "UnityEngine+UIElements+DynamicAtlasSettings")]
impl std::ops::Deref for crate::UnityEngine::UIElements::DynamicAtlasSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DynamicAtlasSettings")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::DynamicAtlasSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DynamicAtlasSettings")]
impl crate::UnityEngine::UIElements::DynamicAtlasSettings {
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
    pub fn get_activeFilters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::DynamicAtlasFilters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::DynamicAtlasFilters = __cordl_object
            .invoke("get_activeFilters", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_customFilter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::DynamicAtlasCustomFilter,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::DynamicAtlasCustomFilter,
        > = __cordl_object.invoke("get_customFilter", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultFilters() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::DynamicAtlasFilters,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::DynamicAtlasFilters = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_defaultFilters", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaults() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DynamicAtlasSettings>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::DynamicAtlasSettings,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_defaults", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxAtlasSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_maxAtlasSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxSubTextureSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_maxSubTextureSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_minAtlasSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_minAtlasSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_activeFilters(
        &mut self,
        value: crate::UnityEngine::UIElements::DynamicAtlasFilters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_activeFilters", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_customFilter(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::DynamicAtlasCustomFilter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_customFilter", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_maxAtlasSize(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_maxAtlasSize", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_maxSubTextureSize(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_maxSubTextureSize", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_minAtlasSize(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_minAtlasSize", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+DynamicAtlasSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::DynamicAtlasSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
