#[cfg(feature = "UnityEngine+ProBuilder+Shapes+ShapeAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct ShapeAttribute {
    __cordl_parent: crate::System::Attribute,
    pub name: *mut crate::System::String,
}
#[cfg(feature = "UnityEngine+ProBuilder+Shapes+ShapeAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::Shapes::ShapeAttribute
    => "UnityEngine.ProBuilder.Shapes"."ShapeAttribute"
);
#[cfg(feature = "UnityEngine+ProBuilder+Shapes+ShapeAttribute")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Shapes::ShapeAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Shapes+ShapeAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Shapes::ShapeAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Shapes+ShapeAttribute")]
impl crate::UnityEngine::ProBuilder::Shapes::ShapeAttribute {
    pub fn New(
        n: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (n))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        n: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (n))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Shapes+ShapeAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::Shapes::ShapeAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}