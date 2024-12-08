#[cfg(feature = "System+Xml+Schema+ForwardAxis")]
#[repr(C)]
#[derive(Debug)]
pub struct ForwardAxis {
    __cordl_parent: crate::System::Object,
    pub _topNode: *mut crate::System::Xml::Schema::DoubleLinkAxis,
    pub _rootNode: *mut crate::System::Xml::Schema::DoubleLinkAxis,
    pub _isAttribute: bool,
    pub _isDss: bool,
    pub _isSelfAxis: bool,
}
#[cfg(feature = "System+Xml+Schema+ForwardAxis")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::ForwardAxis =>
    "System.Xml.Schema"."ForwardAxis"
);
#[cfg(feature = "System+Xml+Schema+ForwardAxis")]
impl std::ops::Deref for crate::System::Xml::Schema::ForwardAxis {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+ForwardAxis")]
impl std::ops::DerefMut for crate::System::Xml::Schema::ForwardAxis {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+ForwardAxis")]
impl crate::System::Xml::Schema::ForwardAxis {
    pub fn New(
        axis: *mut crate::System::Xml::Schema::DoubleLinkAxis,
        isdesorself: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (axis, isdesorself))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        axis: *mut crate::System::Xml::Schema::DoubleLinkAxis,
        isdesorself: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (axis, isdesorself))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsAttribute(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsAttribute", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsDss(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDss", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsSelfAxis(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsSelfAxis", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_RootNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::DoubleLinkAxis> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::DoubleLinkAxis = __cordl_object
            .invoke("get_RootNode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TopNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::DoubleLinkAxis> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::DoubleLinkAxis = __cordl_object
            .invoke("get_TopNode", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Schema+ForwardAxis")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::ForwardAxis {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
