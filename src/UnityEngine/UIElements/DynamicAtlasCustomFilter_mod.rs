#[cfg(feature = "UnityEngine+UIElements+DynamicAtlasCustomFilter")]
#[repr(C)]
#[derive(Debug)]
pub struct DynamicAtlasCustomFilter {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "UnityEngine+UIElements+DynamicAtlasCustomFilter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::DynamicAtlasCustomFilter => "UnityEngine.UIElements"
    ."DynamicAtlasCustomFilter"
);
#[cfg(feature = "UnityEngine+UIElements+DynamicAtlasCustomFilter")]
impl std::ops::Deref for crate::UnityEngine::UIElements::DynamicAtlasCustomFilter {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DynamicAtlasCustomFilter")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::DynamicAtlasCustomFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DynamicAtlasCustomFilter")]
impl crate::UnityEngine::UIElements::DynamicAtlasCustomFilter {
    pub fn Invoke(
        &mut self,
        texture: *mut crate::UnityEngine::Texture2D,
        filtersToApply: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::DynamicAtlasFilters,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Invoke", (texture, filtersToApply))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+DynamicAtlasCustomFilter")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::DynamicAtlasCustomFilter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
