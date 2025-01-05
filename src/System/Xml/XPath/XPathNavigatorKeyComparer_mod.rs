#[cfg(feature = "System+Xml+XPath+XPathNavigatorKeyComparer")]
#[repr(C)]
#[derive(Debug)]
pub struct XPathNavigatorKeyComparer {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Xml+XPath+XPathNavigatorKeyComparer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XPath::XPathNavigatorKeyComparer =>
    "System.Xml.XPath"."XPathNavigatorKeyComparer"
);
#[cfg(feature = "System+Xml+XPath+XPathNavigatorKeyComparer")]
impl std::ops::Deref for crate::System::Xml::XPath::XPathNavigatorKeyComparer {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XPath+XPathNavigatorKeyComparer")]
impl std::ops::DerefMut for crate::System::Xml::XPath::XPathNavigatorKeyComparer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XPath+XPathNavigatorKeyComparer")]
impl crate::System::Xml::XPath::XPathNavigatorKeyComparer {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn System_Collections_IEqualityComparer_Equals(
        &mut self,
        obj1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        obj2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Collections.IEqualityComparer.Equals", (obj1, obj2))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEqualityComparer_GetHashCode(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("System.Collections.IEqualityComparer.GetHashCode", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+XPath+XPathNavigatorKeyComparer")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::XPath::XPathNavigatorKeyComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+XPath+XPathNavigatorKeyComparer")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::Collections::IEqualityComparer>>
for crate::System::Xml::XPath::XPathNavigatorKeyComparer {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::System::Collections::IEqualityComparer> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Xml+XPath+XPathNavigatorKeyComparer")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::Collections::IEqualityComparer>>
for crate::System::Xml::XPath::XPathNavigatorKeyComparer {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::System::Collections::IEqualityComparer> {
        unsafe { std::mem::transmute(self) }
    }
}
