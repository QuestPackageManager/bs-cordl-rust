#[cfg(feature = "System+Xml+Linq+XNamespace")]
#[repr(C)]
#[derive(Debug)]
pub struct XNamespace {
    __cordl_parent: crate::System::Object,
    pub _namespaceName: *mut crate::System::String,
    pub _hashCode: i32,
    pub _names: *mut crate::System::Xml::Linq::XHashtable_1<
        *mut crate::System::Xml::Linq::XName,
    >,
}
#[cfg(feature = "System+Xml+Linq+XNamespace")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Linq::XNamespace =>
    "System.Xml.Linq"."XNamespace"
);
#[cfg(feature = "System+Xml+Linq+XNamespace")]
impl std::ops::Deref for crate::System::Xml::Linq::XNamespace {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Linq+XNamespace")]
impl std::ops::DerefMut for crate::System::Xml::Linq::XNamespace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Linq+XNamespace")]
impl crate::System::Xml::Linq::XNamespace {
    pub fn _ctor(
        &mut self,
        namespaceName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (namespaceName))?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn Equals(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn get_NamespaceName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_NamespaceName", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetName_String0(
        &mut self,
        localName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Linq::XName> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Linq::XName = __cordl_object
            .invoke("GetName", (localName))?;
        Ok(__cordl_ret)
    }
    pub fn GetName_i32_i32_1(
        &mut self,
        localName: *mut crate::System::String,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Linq::XName> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Linq::XName = __cordl_object
            .invoke("GetName", (localName, index, count))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        namespaceName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (namespaceName))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Linq+XNamespace")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Linq::XNamespace {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
