#[cfg(feature = "System+ArraySpec")]
#[repr(C)]
#[derive(Debug)]
pub struct ArraySpec {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub dimensions: i32,
    pub bound: bool,
}
#[cfg(feature = "System+ArraySpec")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ArraySpec => "System"."ArraySpec"
);
#[cfg(feature = "System+ArraySpec")]
impl std::ops::Deref for crate::System::ArraySpec {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ArraySpec")]
impl std::ops::DerefMut for crate::System::ArraySpec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ArraySpec")]
impl crate::System::ArraySpec {
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
        dimensions: i32,
        bound: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dimensions, bound))?;
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
        dimensions: i32,
        bound: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dimensions, bound))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+ArraySpec")]
impl quest_hook::libil2cpp::ObjectType for crate::System::ArraySpec {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+ArraySpec")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::ModifierSpec>>
for crate::System::ArraySpec {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::ModifierSpec> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+ArraySpec")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::ModifierSpec>>
for crate::System::ArraySpec {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::ModifierSpec> {
        unsafe { std::mem::transmute(self) }
    }
}
