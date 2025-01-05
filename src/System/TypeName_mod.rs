#[cfg(feature = "System+TypeName")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeName {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+TypeName")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::TypeName => "System"."TypeName"
);
#[cfg(feature = "System+TypeName")]
impl std::ops::Deref for crate::System::TypeName {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+TypeName")]
impl std::ops::DerefMut for crate::System::TypeName {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+TypeName")]
impl crate::System::TypeName {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_DisplayName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_DisplayName", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+TypeName")]
impl quest_hook::libil2cpp::ObjectType for crate::System::TypeName {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+TypeName")]
impl AsRef<
    crate::System::IEquatable_1<quest_hook::libil2cpp::Gc<crate::System::TypeName>>,
> for crate::System::TypeName {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::System::TypeName>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+TypeName")]
impl AsMut<
    crate::System::IEquatable_1<quest_hook::libil2cpp::Gc<crate::System::TypeName>>,
> for crate::System::TypeName {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::System::TypeName>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
