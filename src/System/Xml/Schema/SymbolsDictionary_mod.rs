#[cfg(feature = "System+Xml+Schema+SymbolsDictionary")]
#[repr(C)]
#[derive(Debug)]
pub struct SymbolsDictionary {
    __cordl_parent: crate::System::Object,
    pub last: i32,
    pub names: *mut crate::System::Collections::Hashtable,
    pub wildcards: *mut crate::System::Collections::Hashtable,
    pub particles: *mut crate::System::Collections::ArrayList,
    pub particleLast: *mut crate::System::Object,
    pub isUpaEnforced: bool,
}
#[cfg(feature = "System+Xml+Schema+SymbolsDictionary")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::SymbolsDictionary =>
    "System.Xml.Schema"."SymbolsDictionary"
);
#[cfg(feature = "System+Xml+Schema+SymbolsDictionary")]
impl std::ops::Deref for crate::System::Xml::Schema::SymbolsDictionary {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+SymbolsDictionary")]
impl std::ops::DerefMut for crate::System::Xml::Schema::SymbolsDictionary {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+SymbolsDictionary")]
impl crate::System::Xml::Schema::SymbolsDictionary {
    pub fn get_Item(
        &mut self,
        name: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Item", (name))?;
        Ok(__cordl_ret)
    }
    pub fn set_IsUpaEnforced(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsUpaEnforced", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddName(
        &mut self,
        name: *mut crate::System::Xml::XmlQualifiedName,
        particle: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("AddName", (name, particle))?;
        Ok(__cordl_ret)
    }
    pub fn GetNamespaceListSymbols(
        &mut self,
        list: *mut crate::System::Xml::Schema::NamespaceList,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ICollection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ICollection = __cordl_object
            .invoke("GetNamespaceListSymbols", (list))?;
        Ok(__cordl_ret)
    }
    pub fn Exists(
        &mut self,
        name: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Exists", (name))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsUpaEnforced(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsUpaEnforced", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetParticle(
        &mut self,
        symbol: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetParticle", (symbol))?;
        Ok(__cordl_ret)
    }
    pub fn NameOf(
        &mut self,
        symbol: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("NameOf", (symbol))?;
        Ok(__cordl_ret)
    }
    pub fn AddNamespaceList(
        &mut self,
        list: *mut crate::System::Xml::Schema::NamespaceList,
        particle: *mut crate::System::Object,
        allowLocal: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddNamespaceList", (list, particle, allowLocal))?;
        Ok(__cordl_ret)
    }
    pub fn AddWildcard(
        &mut self,
        wildcard: *mut crate::System::String,
        particle: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddWildcard", (wildcard, particle))?;
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
#[cfg(feature = "System+Xml+Schema+SymbolsDictionary")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::SymbolsDictionary {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
