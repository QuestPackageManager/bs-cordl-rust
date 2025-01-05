#[cfg(feature = "System+Xml+ValidateNames")]
#[repr(C)]
#[derive(Debug)]
pub struct ValidateNames {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Xml+ValidateNames")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::ValidateNames => "System.Xml"
    ."ValidateNames"
);
#[cfg(feature = "System+Xml+ValidateNames")]
impl std::ops::Deref for crate::System::Xml::ValidateNames {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+ValidateNames")]
impl std::ops::DerefMut for crate::System::Xml::ValidateNames {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+ValidateNames")]
impl crate::System::Xml::ValidateNames {
    pub fn GetInvalidNameException(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        offsetStartChar: i32,
        offsetBadChar: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInvalidNameException", (s, offsetStartChar, offsetBadChar))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNameNoNamespaces(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNameNoNamespaces", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseNCName_Il2CppString1(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseNCName", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseNCName_i32_0(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseNCName", (s, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseNameNoNamespaces(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseNameNoNamespaces", (s, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseNmtoken(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseNmtoken", (s, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseNmtokenNoNamespaces(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseNmtokenNoNamespaces", (s, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseQName(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        offset: i32,
        colonOffset: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseQName", (s, offset, colonOffset))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseQNameThrow(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        prefix: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        localName: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseQNameThrow", (s, prefix, localName))?;
        Ok(__cordl_ret.into())
    }
    pub fn SplitQName(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        prefix: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        lname: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SplitQName", (name, prefix, lname))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowInvalidName(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        offsetStartChar: i32,
        offsetBadChar: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowInvalidName", (s, offsetStartChar, offsetBadChar))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+ValidateNames")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::ValidateNames {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
