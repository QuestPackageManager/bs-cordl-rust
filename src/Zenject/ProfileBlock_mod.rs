#[cfg(feature = "Zenject+ProfileBlock")]
#[repr(C)]
#[derive(Debug)]
pub struct ProfileBlock {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+ProfileBlock")]
unsafe impl quest_hook::libil2cpp::Type for crate::Zenject::ProfileBlock {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "ProfileBlock";
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
#[cfg(feature = "Zenject+ProfileBlock")]
impl std::ops::Deref for crate::Zenject::ProfileBlock {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ProfileBlock")]
impl std::ops::DerefMut for crate::Zenject::ProfileBlock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ProfileBlock")]
impl crate::Zenject::ProfileBlock {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppString1(
        sampleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sampleName))?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool0(
        sampleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        rootBlock: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sampleName, rootBlock))?;
        Ok(__cordl_object.into())
    }
    pub fn Start_0() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ProfileBlock>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::ProfileBlock> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start_Il2CppString3(
        sampleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ProfileBlock>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::ProfileBlock> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Start", (sampleName))?;
        Ok(__cordl_ret.into())
    }
    pub fn Start_Il2CppString_Il2CppObject2(
        sampleNameFormat: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ProfileBlock>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::ProfileBlock> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Start", (sampleNameFormat, obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn Start_Il2CppString_Il2CppObject_Il2CppObject1(
        sampleNameFormat: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        obj1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        obj2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ProfileBlock>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::ProfileBlock> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Start", (sampleNameFormat, obj1, obj2))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString1(
        &mut self,
        sampleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sampleName))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool0(
        &mut self,
        sampleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        rootBlock: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sampleName, rootBlock))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ProfilePattern() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::Regex>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::Regex,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ProfilePattern", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ProfilePattern(
        value: quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::Regex>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_ProfilePattern", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+ProfileBlock")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::ProfileBlock {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+ProfileBlock")]
impl AsRef<crate::System::IDisposable> for crate::Zenject::ProfileBlock {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+ProfileBlock")]
impl AsMut<crate::System::IDisposable> for crate::Zenject::ProfileBlock {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
