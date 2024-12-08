#[cfg(feature = "System+Xml+NameTable+Entry")]
#[repr(C)]
#[derive(Debug)]
pub struct NameTable_Entry {
    __cordl_parent: crate::System::Object,
    pub str: *mut crate::System::String,
    pub hashCode: i32,
    pub next: *mut crate::System::Xml::NameTable_Entry,
}
#[cfg(feature = "System+Xml+NameTable+Entry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::NameTable_Entry => "System.Xml"
    ."NameTable/Entry"
);
#[cfg(feature = "System+Xml+NameTable+Entry")]
impl std::ops::Deref for crate::System::Xml::NameTable_Entry {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+NameTable+Entry")]
impl std::ops::DerefMut for crate::System::Xml::NameTable_Entry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+NameTable+Entry")]
impl crate::System::Xml::NameTable_Entry {
    pub fn New(
        str: *mut crate::System::String,
        hashCode: i32,
        next: *mut crate::System::Xml::NameTable_Entry,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (str, hashCode, next))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        str: *mut crate::System::String,
        hashCode: i32,
        next: *mut crate::System::Xml::NameTable_Entry,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (str, hashCode, next))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+NameTable+Entry")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::NameTable_Entry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+NameTable")]
#[repr(C)]
#[derive(Debug)]
pub struct NameTable {
    __cordl_parent: crate::System::Xml::XmlNameTable,
    pub entries: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Xml::NameTable_Entry,
    >,
    pub count: i32,
    pub mask: i32,
    pub hashCodeRandomizer: i32,
}
#[cfg(feature = "System+Xml+NameTable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::NameTable => "System.Xml"
    ."NameTable"
);
#[cfg(feature = "System+Xml+NameTable")]
impl std::ops::Deref for crate::System::Xml::NameTable {
    type Target = crate::System::Xml::XmlNameTable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+NameTable")]
impl std::ops::DerefMut for crate::System::Xml::NameTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+NameTable")]
impl crate::System::Xml::NameTable {
    #[cfg(feature = "System+Xml+NameTable+Entry")]
    pub type Entry = crate::System::Xml::NameTable_Entry;
    pub fn AddEntry(
        &mut self,
        str: *mut crate::System::String,
        hashCode: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("AddEntry", (str, hashCode))?;
        Ok(__cordl_ret)
    }
    pub fn Add_Il2CppArray_i32_i32_1(
        &mut self,
        key: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        start: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("Add", (key, start, len))?;
        Ok(__cordl_ret)
    }
    pub fn Add_String0(
        &mut self,
        key: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("Add", (key))?;
        Ok(__cordl_ret)
    }
    pub fn Get(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("Get", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Grow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Grow", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "System+Xml+NameTable")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::NameTable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
