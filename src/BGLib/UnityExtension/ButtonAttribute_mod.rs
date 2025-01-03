#[cfg(feature = "BGLib+UnityExtension+ButtonAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct ButtonAttribute {
    __cordl_parent: crate::UnityEngine::PropertyAttribute,
    pub title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "BGLib+UnityExtension+ButtonAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::UnityExtension::ButtonAttribute =>
    "BGLib.UnityExtension"."ButtonAttribute"
);
#[cfg(feature = "BGLib+UnityExtension+ButtonAttribute")]
impl std::ops::Deref for crate::BGLib::UnityExtension::ButtonAttribute {
    type Target = crate::UnityEngine::PropertyAttribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+ButtonAttribute")]
impl std::ops::DerefMut for crate::BGLib::UnityExtension::ButtonAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+ButtonAttribute")]
impl crate::BGLib::UnityExtension::ButtonAttribute {
    pub fn New(
        title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (title))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (title))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+UnityExtension+ButtonAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::UnityExtension::ButtonAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
