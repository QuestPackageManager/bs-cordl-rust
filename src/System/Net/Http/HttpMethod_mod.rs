#[cfg(feature = "System+Net+Http+HttpMethod")]
#[repr(C)]
#[derive(Debug)]
pub struct HttpMethod {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub method: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "System+Net+Http+HttpMethod")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::HttpMethod =>
    "System.Net.Http"."HttpMethod"
);
#[cfg(feature = "System+Net+Http+HttpMethod")]
impl std::ops::Deref for crate::System::Net::Http::HttpMethod {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+HttpMethod")]
impl std::ops::DerefMut for crate::System::Net::Http::HttpMethod {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+HttpMethod")]
impl crate::System::Net::Http::HttpMethod {
    pub fn Equals_HttpMethod0(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpMethod>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
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
    pub fn New(
        method: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (method))?;
        Ok(__cordl_object.into())
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
    pub fn _ctor(
        &mut self,
        method: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (method))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Get() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpMethod>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::Http::HttpMethod,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_Get", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Method(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Method", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Post() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpMethod>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::Http::HttpMethod,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_Post", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        left: quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpMethod>,
        right: quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpMethod>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (left, right))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Http+HttpMethod")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::Http::HttpMethod {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+Http+HttpMethod")]
impl AsRef<crate::System::IEquatable_1<*mut crate::System::Net::Http::HttpMethod>>
for crate::System::Net::Http::HttpMethod {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<*mut crate::System::Net::Http::HttpMethod> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+Http+HttpMethod")]
impl AsMut<crate::System::IEquatable_1<*mut crate::System::Net::Http::HttpMethod>>
for crate::System::Net::Http::HttpMethod {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<*mut crate::System::Net::Http::HttpMethod> {
        unsafe { std::mem::transmute(self) }
    }
}
