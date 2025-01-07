#[cfg(feature = "System+TypeIdentifier")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeIdentifier {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+TypeIdentifier")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::TypeIdentifier {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "TypeIdentifier";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+TypeIdentifier")]
impl std::ops::Deref for crate::System::TypeIdentifier {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+TypeIdentifier")]
impl std::ops::DerefMut for crate::System::TypeIdentifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+TypeIdentifier")]
impl crate::System::TypeIdentifier {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
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
#[cfg(feature = "System+TypeIdentifier")]
impl quest_hook::libil2cpp::ObjectType for crate::System::TypeIdentifier {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+TypeIdentifier")]
impl AsRef<
    crate::System::IEquatable_1<quest_hook::libil2cpp::Gc<crate::System::TypeName>>,
> for crate::System::TypeIdentifier {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::System::TypeName>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+TypeIdentifier")]
impl AsMut<
    crate::System::IEquatable_1<quest_hook::libil2cpp::Gc<crate::System::TypeName>>,
> for crate::System::TypeIdentifier {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::System::TypeName>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+TypeIdentifier")]
impl AsRef<crate::System::TypeName> for crate::System::TypeIdentifier {
    fn as_ref(&self) -> &crate::System::TypeName {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+TypeIdentifier")]
impl AsMut<crate::System::TypeName> for crate::System::TypeIdentifier {
    fn as_mut(&mut self) -> &mut crate::System::TypeName {
        unsafe { std::mem::transmute(self) }
    }
}
