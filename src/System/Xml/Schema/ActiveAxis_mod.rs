#[cfg(feature = "System+Xml+Schema+ActiveAxis")]
#[repr(C)]
#[derive(Debug)]
pub struct ActiveAxis {
    __cordl_parent: crate::System::Object,
    pub _currentDepth: i32,
    pub _isActive: bool,
    pub _axisTree: *mut crate::System::Xml::Schema::Asttree,
    pub _axisStack: *mut crate::System::Collections::ArrayList,
}
#[cfg(feature = "System+Xml+Schema+ActiveAxis")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::ActiveAxis =>
    "System.Xml.Schema"."ActiveAxis"
);
#[cfg(feature = "System+Xml+Schema+ActiveAxis")]
impl std::ops::Deref for crate::System::Xml::Schema::ActiveAxis {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+ActiveAxis")]
impl std::ops::DerefMut for crate::System::Xml::Schema::ActiveAxis {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+ActiveAxis")]
impl crate::System::Xml::Schema::ActiveAxis {
    pub fn get_CurrentDepth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_CurrentDepth", ())?;
        Ok(__cordl_ret)
    }
    pub fn MoveToAttribute(
        &mut self,
        localname: *mut crate::System::String,
        URN: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("MoveToAttribute", (localname, URN))?;
        Ok(__cordl_ret)
    }
    pub fn Reactivate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reactivate", ())?;
        Ok(__cordl_ret)
    }
    pub fn EndElement(
        &mut self,
        localname: *mut crate::System::String,
        URN: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndElement", (localname, URN))?;
        Ok(__cordl_ret)
    }
    pub fn MoveToStartElement(
        &mut self,
        localname: *mut crate::System::String,
        URN: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("MoveToStartElement", (localname, URN))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        axisTree: *mut crate::System::Xml::Schema::Asttree,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (axisTree))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        axisTree: *mut crate::System::Xml::Schema::Asttree,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (axisTree))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Schema+ActiveAxis")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::ActiveAxis {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
