#[cfg(feature = "System+Xml+Schema+IdRefNode")]
#[repr(C)]
#[derive(Debug)]
pub struct IdRefNode {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub LineNo: i32,
    pub LinePos: i32,
    pub Next: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::IdRefNode>,
}
#[cfg(feature = "System+Xml+Schema+IdRefNode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::IdRefNode =>
    "System.Xml.Schema"."IdRefNode"
);
#[cfg(feature = "System+Xml+Schema+IdRefNode")]
impl std::ops::Deref for crate::System::Xml::Schema::IdRefNode {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+IdRefNode")]
impl std::ops::DerefMut for crate::System::Xml::Schema::IdRefNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+IdRefNode")]
impl crate::System::Xml::Schema::IdRefNode {
    pub fn New(
        next: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::IdRefNode>,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        lineNo: i32,
        linePos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (next, id, lineNo, linePos))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        next: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::IdRefNode>,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        lineNo: i32,
        linePos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (next, id, lineNo, linePos))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+IdRefNode")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::IdRefNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
