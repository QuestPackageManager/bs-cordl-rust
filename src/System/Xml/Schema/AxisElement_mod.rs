#[cfg(feature = "System+Xml+Schema+AxisElement")]
#[repr(C)]
#[derive(Debug)]
pub struct AxisElement {
    __cordl_parent: crate::System::Object,
    pub curNode: *mut crate::System::Xml::Schema::DoubleLinkAxis,
    pub rootDepth: i32,
    pub curDepth: i32,
    pub isMatch: bool,
}
#[cfg(feature = "System+Xml+Schema+AxisElement")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::AxisElement =>
    "System.Xml.Schema"."AxisElement"
);
#[cfg(feature = "System+Xml+Schema+AxisElement")]
impl std::ops::Deref for crate::System::Xml::Schema::AxisElement {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+AxisElement")]
impl std::ops::DerefMut for crate::System::Xml::Schema::AxisElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+AxisElement")]
impl crate::System::Xml::Schema::AxisElement {
    pub fn MoveToChild(
        &mut self,
        name: *mut crate::System::String,
        URN: *mut crate::System::String,
        depth: i32,
        parent: *mut crate::System::Xml::Schema::ForwardAxis,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("MoveToChild", (name, URN, depth, parent))?;
        Ok(__cordl_ret)
    }
    pub fn MoveToParent(
        &mut self,
        depth: i32,
        parent: *mut crate::System::Xml::Schema::ForwardAxis,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveToParent", (depth, parent))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        node: *mut crate::System::Xml::Schema::DoubleLinkAxis,
        depth: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (node, depth))?;
        Ok(__cordl_object)
    }
    pub fn SetDepth(
        &mut self,
        depth: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDepth", (depth))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        node: *mut crate::System::Xml::Schema::DoubleLinkAxis,
        depth: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (node, depth))?;
        Ok(__cordl_ret)
    }
    pub fn get_CurNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::DoubleLinkAxis> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::DoubleLinkAxis = __cordl_object
            .invoke("get_CurNode", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Schema+AxisElement")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::AxisElement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
