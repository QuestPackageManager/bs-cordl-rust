#[cfg(feature = "System+Xml+Schema+NamespaceListV1Compat")]
#[repr(C)]
#[derive(Debug)]
pub struct NamespaceListV1Compat {
    __cordl_parent: crate::System::Xml::Schema::NamespaceList,
}
#[cfg(feature = "System+Xml+Schema+NamespaceListV1Compat")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::NamespaceListV1Compat =>
    "System.Xml.Schema"."NamespaceListV1Compat"
);
#[cfg(feature = "System+Xml+Schema+NamespaceListV1Compat")]
impl std::ops::Deref for crate::System::Xml::Schema::NamespaceListV1Compat {
    type Target = crate::System::Xml::Schema::NamespaceList;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+NamespaceListV1Compat")]
impl std::ops::DerefMut for crate::System::Xml::Schema::NamespaceListV1Compat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+NamespaceListV1Compat")]
impl crate::System::Xml::Schema::NamespaceListV1Compat {
    pub fn _ctor(
        &mut self,
        namespaces: *mut crate::System::String,
        targetNamespace: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (namespaces, targetNamespace))?;
        Ok(__cordl_ret)
    }
    pub fn Allows(
        &mut self,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Allows", (ns))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        namespaces: *mut crate::System::String,
        targetNamespace: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (namespaces, targetNamespace))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Schema+NamespaceListV1Compat")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::NamespaceListV1Compat {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
