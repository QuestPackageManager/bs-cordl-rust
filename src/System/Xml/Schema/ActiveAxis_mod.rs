#[cfg(feature = "System+Xml+Schema+ActiveAxis")]
#[repr(C)]
#[derive(Debug)]
pub struct ActiveAxis {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _currentDepth: i32,
    pub _isActive: bool,
    pub _axisTree: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::Asttree>,
    pub _axisStack: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
}
#[cfg(feature = "System+Xml+Schema+ActiveAxis")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::ActiveAxis =>
    "System.Xml.Schema"."ActiveAxis"
);
#[cfg(feature = "System+Xml+Schema+ActiveAxis")]
impl std::ops::Deref for crate::System::Xml::Schema::ActiveAxis {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn EndElement(
        &mut self,
        localname: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        URN: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndElement", (localname, URN))?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveToAttribute(
        &mut self,
        localname: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        URN: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("MoveToAttribute", (localname, URN))?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveToStartElement(
        &mut self,
        localname: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        URN: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("MoveToStartElement", (localname, URN))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        axisTree: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::Asttree>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (axisTree))?;
        Ok(__cordl_object.into())
    }
    pub fn Reactivate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reactivate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        axisTree: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::Asttree>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (axisTree))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CurrentDepth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_CurrentDepth", ())?;
        Ok(__cordl_ret.into())
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
