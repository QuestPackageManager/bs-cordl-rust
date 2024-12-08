#[cfg(feature = "System+Xml+Schema+AxisStack")]
#[repr(C)]
#[derive(Debug)]
pub struct AxisStack {
    __cordl_parent: crate::System::Object,
    pub _stack: *mut crate::System::Collections::ArrayList,
    pub _subtree: *mut crate::System::Xml::Schema::ForwardAxis,
    pub _parent: *mut crate::System::Xml::Schema::ActiveAxis,
}
#[cfg(feature = "System+Xml+Schema+AxisStack")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::AxisStack =>
    "System.Xml.Schema"."AxisStack"
);
#[cfg(feature = "System+Xml+Schema+AxisStack")]
impl std::ops::Deref for crate::System::Xml::Schema::AxisStack {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+AxisStack")]
impl std::ops::DerefMut for crate::System::Xml::Schema::AxisStack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+AxisStack")]
impl crate::System::Xml::Schema::AxisStack {
    pub fn MoveToParent(
        &mut self,
        name: *mut crate::System::String,
        URN: *mut crate::System::String,
        depth: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveToParent", (name, URN, depth))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        faxis: *mut crate::System::Xml::Schema::ForwardAxis,
        parent: *mut crate::System::Xml::Schema::ActiveAxis,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (faxis, parent))?;
        Ok(__cordl_ret)
    }
    pub fn MoveToAttribute(
        &mut self,
        name: *mut crate::System::String,
        URN: *mut crate::System::String,
        depth: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("MoveToAttribute", (name, URN, depth))?;
        Ok(__cordl_ret)
    }
    pub fn get_Length(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Length", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Subtree(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::ForwardAxis> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::ForwardAxis = __cordl_object
            .invoke("get_Subtree", ())?;
        Ok(__cordl_ret)
    }
    pub fn Push(
        &mut self,
        depth: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Push", (depth))?;
        Ok(__cordl_ret)
    }
    pub fn MoveToChild(
        &mut self,
        name: *mut crate::System::String,
        URN: *mut crate::System::String,
        depth: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("MoveToChild", (name, URN, depth))?;
        Ok(__cordl_ret)
    }
    pub fn Pop(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object.invoke("Pop", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        faxis: *mut crate::System::Xml::Schema::ForwardAxis,
        parent: *mut crate::System::Xml::Schema::ActiveAxis,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (faxis, parent))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Schema+AxisStack")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::AxisStack {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
