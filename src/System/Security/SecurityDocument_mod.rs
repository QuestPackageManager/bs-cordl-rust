#[cfg(feature = "System+Security+SecurityDocument")]
#[repr(C)]
#[derive(Debug)]
pub struct SecurityDocument {
    __cordl_parent: crate::System::Object,
    pub m_data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "System+Security+SecurityDocument")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::SecurityDocument =>
    "System.Security"."SecurityDocument"
);
#[cfg(feature = "System+Security+SecurityDocument")]
impl std::ops::Deref for crate::System::Security::SecurityDocument {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+SecurityDocument")]
impl std::ops::DerefMut for crate::System::Security::SecurityDocument {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+SecurityDocument")]
impl crate::System::Security::SecurityDocument {
    pub fn AddString(
        &mut self,
        str: *mut crate::System::String,
        position: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddString", (str, position))?;
        Ok(__cordl_ret)
    }
    pub fn AddToken(
        &mut self,
        b: u8,
        position: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddToken", (b, position))?;
        Ok(__cordl_ret)
    }
    pub fn AppendString(
        &mut self,
        str: *mut crate::System::String,
        position: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AppendString", (str, position))?;
        Ok(__cordl_ret)
    }
    pub fn GetElement(
        &mut self,
        position: i32,
        bCreate: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Security::SecurityElement> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::SecurityElement = __cordl_object
            .invoke("GetElement", (position, bCreate))?;
        Ok(__cordl_ret)
    }
    pub fn GetRootElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Security::SecurityElement> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::SecurityElement = __cordl_object
            .invoke("GetRootElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetString(
        &mut self,
        position: quest_hook::libil2cpp::ByRefMut<i32>,
        bCreate: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetString", (position, bCreate))?;
        Ok(__cordl_ret)
    }
    pub fn GuaranteeSize(
        &mut self,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GuaranteeSize", (_cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn InternalGetElement(
        &mut self,
        position: quest_hook::libil2cpp::ByRefMut<i32>,
        bCreate: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Security::SecurityElement> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::SecurityElement = __cordl_object
            .invoke("InternalGetElement", (position, bCreate))?;
        Ok(__cordl_ret)
    }
    pub fn New(numData: i32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (numData))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        numData: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (numData))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Security+SecurityDocument")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Security::SecurityDocument {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
