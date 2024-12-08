#[cfg(feature = "Newtonsoft+Json+DefaultJsonNameTable")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultJsonNameTable {
    __cordl_parent: crate::Newtonsoft::Json::JsonNameTable,
    pub _count: i32,
    pub _entries: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::Newtonsoft::Json::DefaultJsonNameTable_Entry,
    >,
    pub _mask: i32,
}
#[cfg(feature = "Newtonsoft+Json+DefaultJsonNameTable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::DefaultJsonNameTable =>
    "Newtonsoft.Json"."DefaultJsonNameTable"
);
#[cfg(feature = "Newtonsoft+Json+DefaultJsonNameTable")]
impl std::ops::Deref for crate::Newtonsoft::Json::DefaultJsonNameTable {
    type Target = crate::Newtonsoft::Json::JsonNameTable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+DefaultJsonNameTable")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::DefaultJsonNameTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+DefaultJsonNameTable")]
impl crate::Newtonsoft::Json::DefaultJsonNameTable {
    #[cfg(feature = "Newtonsoft+Json+DefaultJsonNameTable+Entry")]
    pub type Entry = crate::Newtonsoft::Json::DefaultJsonNameTable_Entry;
    pub fn Add(
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
    pub fn Get(
        &mut self,
        key: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("Get", (key, start, length))?;
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
#[cfg(feature = "Newtonsoft+Json+DefaultJsonNameTable")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::DefaultJsonNameTable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+DefaultJsonNameTable+Entry")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultJsonNameTable_Entry {
    __cordl_parent: crate::System::Object,
    pub Value: *mut crate::System::String,
    pub HashCode: i32,
    pub Next: *mut crate::Newtonsoft::Json::DefaultJsonNameTable_Entry,
}
#[cfg(feature = "Newtonsoft+Json+DefaultJsonNameTable+Entry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::DefaultJsonNameTable_Entry =>
    "Newtonsoft.Json"."DefaultJsonNameTable/Entry"
);
#[cfg(feature = "Newtonsoft+Json+DefaultJsonNameTable+Entry")]
impl std::ops::Deref for crate::Newtonsoft::Json::DefaultJsonNameTable_Entry {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+DefaultJsonNameTable+Entry")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::DefaultJsonNameTable_Entry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+DefaultJsonNameTable+Entry")]
impl crate::Newtonsoft::Json::DefaultJsonNameTable_Entry {
    pub fn New(
        value: *mut crate::System::String,
        hashCode: i32,
        next: *mut crate::Newtonsoft::Json::DefaultJsonNameTable_Entry,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (value, hashCode, next))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        value: *mut crate::System::String,
        hashCode: i32,
        next: *mut crate::Newtonsoft::Json::DefaultJsonNameTable_Entry,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (value, hashCode, next))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Newtonsoft+Json+DefaultJsonNameTable+Entry")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::DefaultJsonNameTable_Entry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
