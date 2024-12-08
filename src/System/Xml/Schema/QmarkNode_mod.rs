#[cfg(feature = "System+Xml+Schema+QmarkNode")]
#[repr(C)]
#[derive(Debug)]
pub struct QmarkNode {
    __cordl_parent: crate::System::Xml::Schema::InteriorNode,
}
#[cfg(feature = "System+Xml+Schema+QmarkNode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::QmarkNode =>
    "System.Xml.Schema"."QmarkNode"
);
#[cfg(feature = "System+Xml+Schema+QmarkNode")]
impl std::ops::Deref for crate::System::Xml::Schema::QmarkNode {
    type Target = crate::System::Xml::Schema::InteriorNode;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+QmarkNode")]
impl std::ops::DerefMut for crate::System::Xml::Schema::QmarkNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+QmarkNode")]
impl crate::System::Xml::Schema::QmarkNode {
    pub fn ConstructPos(
        &mut self,
        firstpos: *mut crate::System::Xml::Schema::BitSet,
        lastpos: *mut crate::System::Xml::Schema::BitSet,
        followpos: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Xml::Schema::BitSet,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConstructPos", (firstpos, lastpos, followpos))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsNullable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsNullable", ())?;
        Ok(__cordl_ret)
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Schema+QmarkNode")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::QmarkNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
