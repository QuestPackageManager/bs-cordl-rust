#[cfg(feature = "cordl_class_Unity+Hierarchy+IHierarchySearchQueryParser")]
#[repr(C)]
#[derive(Debug)]
pub struct IHierarchySearchQueryParser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Unity+Hierarchy+IHierarchySearchQueryParser")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Hierarchy::IHierarchySearchQueryParser {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Hierarchy";
    const CLASS_NAME: &'static str = "IHierarchySearchQueryParser";
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
#[cfg(feature = "Unity+Hierarchy+IHierarchySearchQueryParser")]
impl std::ops::Deref for crate::Unity::Hierarchy::IHierarchySearchQueryParser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Hierarchy+IHierarchySearchQueryParser")]
impl std::ops::DerefMut for crate::Unity::Hierarchy::IHierarchySearchQueryParser {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Hierarchy+IHierarchySearchQueryParser")]
impl crate::Unity::Hierarchy::IHierarchySearchQueryParser {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_Unity+Hierarchy+IHierarchySearchQueryParser")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Hierarchy::IHierarchySearchQueryParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
