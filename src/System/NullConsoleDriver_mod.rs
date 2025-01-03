#[cfg(feature = "System+NullConsoleDriver")]
#[repr(C)]
#[derive(Debug)]
pub struct NullConsoleDriver {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+NullConsoleDriver")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::NullConsoleDriver => "System"
    ."NullConsoleDriver"
);
#[cfg(feature = "System+NullConsoleDriver")]
impl std::ops::Deref for crate::System::NullConsoleDriver {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+NullConsoleDriver")]
impl std::ops::DerefMut for crate::System::NullConsoleDriver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+NullConsoleDriver")]
impl crate::System::NullConsoleDriver {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ReadKey(
        &mut self,
        intercept: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::ConsoleKeyInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ConsoleKeyInfo = __cordl_object
            .invoke("ReadKey", (intercept))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+NullConsoleDriver")]
impl quest_hook::libil2cpp::ObjectType for crate::System::NullConsoleDriver {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+NullConsoleDriver")]
impl AsRef<crate::System::IConsoleDriver> for crate::System::NullConsoleDriver {
    fn as_ref(&self) -> &crate::System::IConsoleDriver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+NullConsoleDriver")]
impl AsMut<crate::System::IConsoleDriver> for crate::System::NullConsoleDriver {
    fn as_mut(&mut self) -> &mut crate::System::IConsoleDriver {
        unsafe { std::mem::transmute(self) }
    }
}
