#[cfg(feature = "HEU_ExampleEvergreenQuery")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_ExampleEvergreenQuery {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "HEU_ExampleEvergreenQuery")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for HEU_ExampleEvergreenQuery => ""
    ."HEU_ExampleEvergreenQuery"
);
#[cfg(feature = "HEU_ExampleEvergreenQuery")]
impl std::ops::Deref for HEU_ExampleEvergreenQuery {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HEU_ExampleEvergreenQuery")]
impl std::ops::DerefMut for HEU_ExampleEvergreenQuery {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HEU_ExampleEvergreenQuery")]
impl HEU_ExampleEvergreenQuery {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "HEU_ExampleEvergreenQuery")]
impl quest_hook::libil2cpp::ObjectType for HEU_ExampleEvergreenQuery {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
