#[cfg(feature = "UnityEngine+InspectorOrderAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct InspectorOrderAttribute {
    __cordl_parent: crate::UnityEngine::PropertyAttribute,
    pub _m_inspectorSort_k__BackingField: crate::UnityEngine::InspectorSort,
    pub _m_sortDirection_k__BackingField: crate::UnityEngine::InspectorSortDirection,
}
#[cfg(feature = "UnityEngine+InspectorOrderAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InspectorOrderAttribute =>
    "UnityEngine"."InspectorOrderAttribute"
);
#[cfg(feature = "UnityEngine+InspectorOrderAttribute")]
impl std::ops::Deref for crate::UnityEngine::InspectorOrderAttribute {
    type Target = crate::UnityEngine::PropertyAttribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InspectorOrderAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::InspectorOrderAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InspectorOrderAttribute")]
impl crate::UnityEngine::InspectorOrderAttribute {
    pub fn get_m_inspectorSort(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InspectorSort> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InspectorSort = __cordl_object
            .invoke("get_m_inspectorSort", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_m_sortDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InspectorSortDirection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InspectorSortDirection = __cordl_object
            .invoke("get_m_sortDirection", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InspectorOrderAttribute")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::InspectorOrderAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
