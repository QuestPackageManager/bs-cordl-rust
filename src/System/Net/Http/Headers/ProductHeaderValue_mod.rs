#[cfg(feature = "System+Net+Http+Headers+ProductHeaderValue")]
#[repr(C)]
#[derive(Debug)]
pub struct ProductHeaderValue {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _Name_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _Version_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "System+Net+Http+Headers+ProductHeaderValue")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::Headers::ProductHeaderValue
    => "System.Net.Http.Headers"."ProductHeaderValue"
);
#[cfg(feature = "System+Net+Http+Headers+ProductHeaderValue")]
impl std::ops::Deref for crate::System::Net::Http::Headers::ProductHeaderValue {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+ProductHeaderValue")]
impl std::ops::DerefMut for crate::System::Net::Http::Headers::ProductHeaderValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+ProductHeaderValue")]
impl crate::System::Net::Http::Headers::ProductHeaderValue {
    pub fn Equals(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn System_ICloneable_Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("System.ICloneable.Clone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParse(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        minimalCount: i32,
        result: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Collections::Generic::List_1<
                *mut crate::System::Net::Http::Headers::ProductHeaderValue,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParse", (input, minimalCount, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseElement(
        lexer: quest_hook::libil2cpp::Gc<crate::System::Net::Http::Headers::Lexer>,
        parsedValue: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Net::Http::Headers::ProductHeaderValue,
        >,
        t: quest_hook::libil2cpp::ByRefMut<crate::System::Net::Http::Headers::Token>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseElement", (lexer, parsedValue, t))?;
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
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Name", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Version(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Version", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Name(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Name", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Version(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Version", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Http+Headers+ProductHeaderValue")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Http::Headers::ProductHeaderValue {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+Http+Headers+ProductHeaderValue")]
impl AsRef<crate::System::ICloneable>
for crate::System::Net::Http::Headers::ProductHeaderValue {
    fn as_ref(&self) -> &crate::System::ICloneable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+Http+Headers+ProductHeaderValue")]
impl AsMut<crate::System::ICloneable>
for crate::System::Net::Http::Headers::ProductHeaderValue {
    fn as_mut(&mut self) -> &mut crate::System::ICloneable {
        unsafe { std::mem::transmute(self) }
    }
}
