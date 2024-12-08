#[cfg(feature = "UnityEngine+UI+ClipperRegistry")]
#[repr(C)]
#[derive(Debug)]
pub struct ClipperRegistry {
    __cordl_parent: crate::System::Object,
    pub m_Clippers: *mut crate::UnityEngine::UI::Collections::IndexedSet_1<
        *mut crate::UnityEngine::UI::IClipper,
    >,
}
#[cfg(feature = "UnityEngine+UI+ClipperRegistry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::ClipperRegistry =>
    "UnityEngine.UI"."ClipperRegistry"
);
#[cfg(feature = "UnityEngine+UI+ClipperRegistry")]
impl std::ops::Deref for crate::UnityEngine::UI::ClipperRegistry {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+ClipperRegistry")]
impl std::ops::DerefMut for crate::UnityEngine::UI::ClipperRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+ClipperRegistry")]
impl crate::UnityEngine::UI::ClipperRegistry {
    pub fn Cull(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Cull", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "UnityEngine+UI+ClipperRegistry")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UI::ClipperRegistry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
