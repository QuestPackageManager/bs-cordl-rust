#[cfg(feature = "cordl_class_UnityEngine+InspectorOrderAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct InspectorOrderAttribute {
    __cordl_parent: crate::UnityEngine::PropertyAttribute,
    pub _m_inspectorSort_k__BackingField: crate::UnityEngine::InspectorSort,
    pub _m_sortDirection_k__BackingField: crate::UnityEngine::InspectorSortDirection,
}
#[cfg(feature = "cordl_class_UnityEngine+InspectorOrderAttribute")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::InspectorOrderAttribute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "InspectorOrderAttribute";
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
#[cfg(feature = "UnityEngine+InspectorOrderAttribute")]
impl std::ops::Deref for crate::UnityEngine::InspectorOrderAttribute {
    type Target = crate::UnityEngine::PropertyAttribute;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InspectorOrderAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::InspectorOrderAttribute {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InspectorOrderAttribute")]
impl crate::UnityEngine::InspectorOrderAttribute {
    pub fn get_m_inspectorSort(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InspectorSort> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InspectorSort,
                        0usize,
                    >("get_m_inspectorSort")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_m_inspectorSort", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InspectorSort = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_m_sortDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InspectorSortDirection> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InspectorSortDirection,
                        0usize,
                    >("get_m_sortDirection")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_m_sortDirection", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InspectorSortDirection = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InspectorOrderAttribute")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::InspectorOrderAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
