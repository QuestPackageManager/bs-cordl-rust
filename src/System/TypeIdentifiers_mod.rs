#[cfg(feature = "System+TypeIdentifiers")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeIdentifiers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+TypeIdentifiers")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::TypeIdentifiers {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "TypeIdentifiers";
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
#[cfg(feature = "System+TypeIdentifiers")]
impl std::ops::Deref for crate::System::TypeIdentifiers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::TypeIdentifiers as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<crate::System::TypeIdentifier>,
                1usize,
            >("FromDisplay")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::TypeIdentifiers as quest_hook::libil2cpp::Type >
                    ::class(), "FromDisplay", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::TypeIdentifier> = unsafe {
            method.invoke_unchecked((), (displayName))?
        };
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
    __cordl_parent: crate::System::TypeNames_ATypeName,
    pub displayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub internal_name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "System+TypeIdentifiers+Display")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::TypeIdentifiers_Display {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "TypeIdentifiers/Display";
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
#[cfg(feature = "System+TypeIdentifiers+Display")]
impl std::ops::Deref for crate::System::TypeIdentifiers_Display {
    type Target = crate::System::TypeNames_ATypeName;
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::TypeIdentifiers_Display as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("GetInternalName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::TypeIdentifiers_Display as
                    quest_hook::libil2cpp::Type > ::class(), "GetInternalName", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::TypeIdentifiers_Display as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::TypeIdentifiers_Display as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (displayName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_DisplayName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::TypeIdentifiers_Display as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_DisplayName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::TypeIdentifiers_Display as
                    quest_hook::libil2cpp::Type > ::class(), "get_DisplayName", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_InternalName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::TypeIdentifiers_Display as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_InternalName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::TypeIdentifiers_Display as
                    quest_hook::libil2cpp::Type > ::class(), "get_InternalName", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
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
impl AsRef<
    crate::System::IEquatable_1<quest_hook::libil2cpp::Gc<crate::System::TypeName>>,
> for crate::System::TypeIdentifiers_Display {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::System::TypeName>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+TypeIdentifiers+Display")]
impl AsMut<
    crate::System::IEquatable_1<quest_hook::libil2cpp::Gc<crate::System::TypeName>>,
> for crate::System::TypeIdentifiers_Display {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::System::TypeName>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+TypeIdentifiers+Display")]
impl AsRef<crate::System::TypeIdentifier> for crate::System::TypeIdentifiers_Display {
    fn as_ref(&self) -> &crate::System::TypeIdentifier {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+TypeIdentifiers+Display")]
impl AsMut<crate::System::TypeIdentifier> for crate::System::TypeIdentifiers_Display {
    fn as_mut(&mut self) -> &mut crate::System::TypeIdentifier {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+TypeIdentifiers+Display")]
impl AsRef<crate::System::TypeName> for crate::System::TypeIdentifiers_Display {
    fn as_ref(&self) -> &crate::System::TypeName {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+TypeIdentifiers+Display")]
impl AsMut<crate::System::TypeName> for crate::System::TypeIdentifiers_Display {
    fn as_mut(&mut self) -> &mut crate::System::TypeName {
        unsafe { std::mem::transmute(self) }
    }
}
