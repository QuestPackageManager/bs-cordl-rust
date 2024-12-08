#[cfg(feature = "System+Xml+Schema+Positions")]
#[repr(C)]
#[derive(Debug)]
pub struct Positions {
    __cordl_parent: crate::System::Object,
    pub positions: *mut crate::System::Collections::ArrayList,
}
#[cfg(feature = "System+Xml+Schema+Positions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::Positions =>
    "System.Xml.Schema"."Positions"
);
#[cfg(feature = "System+Xml+Schema+Positions")]
impl std::ops::Deref for crate::System::Xml::Schema::Positions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Positions")]
impl std::ops::DerefMut for crate::System::Xml::Schema::Positions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Positions")]
impl crate::System::Xml::Schema::Positions {
    pub fn Add(
        &mut self,
        symbol: i32,
        particle: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Add", (symbol, particle))?;
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
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Item(
        &mut self,
        pos: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::Schema::Position> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::Position = __cordl_object
            .invoke("get_Item", (pos))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Schema+Positions")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::Positions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}