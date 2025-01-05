#[cfg(feature = "System+Xml+Serialization+KeyHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct KeyHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Xml+Serialization+KeyHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Serialization::KeyHelper =>
    "System.Xml.Serialization"."KeyHelper"
);
#[cfg(feature = "System+Xml+Serialization+KeyHelper")]
impl std::ops::Deref for crate::System::Xml::Serialization::KeyHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+KeyHelper")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::KeyHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+KeyHelper")]
impl crate::System::Xml::Serialization::KeyHelper {
    pub fn AddField_Il2CppString0(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        n: i32,
        val: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddField", (sb, n, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddField_Il2CppString_Il2CppString1(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        n: i32,
        val: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        def: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddField", (sb, n, val, def))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddField_Type5(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        n: i32,
        val: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddField", (sb, n, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddField__cordl_bool2(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        n: i32,
        val: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddField", (sb, n, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddField__cordl_bool__cordl_bool3(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        n: i32,
        val: bool,
        def: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddField", (sb, n, val, def))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddField_i32_i32_4(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        n: i32,
        val: i32,
        def: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddField", (sb, n, val, def))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Serialization+KeyHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Serialization::KeyHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
