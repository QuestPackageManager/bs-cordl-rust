#[cfg(feature = "System+PointerSpec")]
#[repr(C)]
#[derive(Debug)]
pub struct PointerSpec {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub pointer_level: i32,
}
#[cfg(feature = "System+PointerSpec")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::PointerSpec => "System"."PointerSpec"
);
#[cfg(feature = "System+PointerSpec")]
impl std::ops::Deref for crate::System::PointerSpec {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+PointerSpec")]
impl std::ops::DerefMut for crate::System::PointerSpec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+PointerSpec")]
impl crate::System::PointerSpec {
    pub fn Append(
        &mut self,
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder> = __cordl_object
            .invoke("Append", (sb))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        pointer_level: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pointer_level))?;
        Ok(__cordl_object.into())
    }
    pub fn Resolve(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("Resolve", (_cordl_type))?;
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
    pub fn _ctor(
        &mut self,
        pointer_level: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pointer_level))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+PointerSpec")]
impl quest_hook::libil2cpp::ObjectType for crate::System::PointerSpec {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+PointerSpec")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::ModifierSpec>>
for crate::System::PointerSpec {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::ModifierSpec> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+PointerSpec")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::ModifierSpec>>
for crate::System::PointerSpec {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::ModifierSpec> {
        unsafe { std::mem::transmute(self) }
    }
}
