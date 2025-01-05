#[cfg(feature = "System+TypeIdentifiers")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeIdentifiers {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+TypeIdentifiers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::TypeIdentifiers => "System"
    ."TypeIdentifiers"
);
#[cfg(feature = "System+TypeIdentifiers")]
impl std::ops::Deref for crate::System::TypeIdentifiers {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+TypeIdentifiers")]
impl std::ops::DerefMut for crate::System::TypeIdentifiers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+TypeIdentifiers")]
impl crate::System::TypeIdentifiers {
    #[cfg(feature = "System+TypeIdentifiers+Display")]
    pub type Display = crate::System::TypeIdentifiers_Display;
    pub fn FromDisplay(
        displayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::TypeIdentifier>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::TypeIdentifier> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromDisplay", (displayName))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+TypeIdentifiers")]
impl quest_hook::libil2cpp::ObjectType for crate::System::TypeIdentifiers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+TypeIdentifiers+Display")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeIdentifiers_Display {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::TypeNames_ATypeName>,
    pub displayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub internal_name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "System+TypeIdentifiers+Display")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::TypeIdentifiers_Display => "System"
    ."TypeIdentifiers/Display"
);
#[cfg(feature = "System+TypeIdentifiers+Display")]
impl std::ops::Deref for crate::System::TypeIdentifiers_Display {
    type Target = quest_hook::libil2cpp::Gc<crate::System::TypeNames_ATypeName>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+TypeIdentifiers+Display")]
impl std::ops::DerefMut for crate::System::TypeIdentifiers_Display {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+TypeIdentifiers+Display")]
impl crate::System::TypeIdentifiers_Display {
    pub fn GetInternalName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetInternalName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        displayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (displayName))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        displayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (displayName))?;
        Ok(__cordl_ret.into())
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
    pub fn get_InternalName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_InternalName", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+TypeIdentifiers+Display")]
impl quest_hook::libil2cpp::ObjectType for crate::System::TypeIdentifiers_Display {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+TypeIdentifiers+Display")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::TypeIdentifier>>
for crate::System::TypeIdentifiers_Display {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::TypeIdentifier> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+TypeIdentifiers+Display")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::TypeIdentifier>>
for crate::System::TypeIdentifiers_Display {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::System::TypeIdentifier> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+TypeIdentifiers+Display")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::TypeName>>
for crate::System::TypeIdentifiers_Display {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::TypeName> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+TypeIdentifiers+Display")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::TypeName>>
for crate::System::TypeIdentifiers_Display {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::TypeName> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+TypeIdentifiers+Display")]
impl AsRef<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Gc<crate::System::TypeName>>>
for crate::System::TypeIdentifiers_Display {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Gc<crate::System::TypeName>> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+TypeIdentifiers+Display")]
impl AsMut<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Gc<crate::System::TypeName>>>
for crate::System::TypeIdentifiers_Display {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::System::TypeName>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
