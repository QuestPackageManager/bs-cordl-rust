#[cfg(feature = "UnityEngine+UIElements+FocusChangeDirection")]
#[repr(C)]
#[derive(Debug)]
pub struct FocusChangeDirection {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Value: i32,
}
#[cfg(feature = "UnityEngine+UIElements+FocusChangeDirection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::FocusChangeDirection =>
    "UnityEngine.UIElements"."FocusChangeDirection"
);
#[cfg(feature = "UnityEngine+UIElements+FocusChangeDirection")]
impl std::ops::Deref for crate::UnityEngine::UIElements::FocusChangeDirection {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+FocusChangeDirection")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::FocusChangeDirection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+FocusChangeDirection")]
impl crate::UnityEngine::UIElements::FocusChangeDirection {
    pub fn ApplyTo(
        &mut self,
        focusController: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::FocusController,
        >,
        f: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Focusable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyTo", (focusController, f))?;
        Ok(__cordl_ret.into())
    }
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
    pub fn New(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (value))?;
        Ok(__cordl_object.into())
    }
    pub fn System_IDisposable_Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.IDisposable.Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lastValue() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::FocusChangeDirection>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::FocusChangeDirection,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_lastValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_none() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::FocusChangeDirection>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::FocusChangeDirection,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_none", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unspecified() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::FocusChangeDirection>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::FocusChangeDirection,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_unspecified", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        fcd: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::FocusChangeDirection,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (fcd))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+FocusChangeDirection")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::FocusChangeDirection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+FocusChangeDirection")]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::UIElements::FocusChangeDirection {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+FocusChangeDirection")]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::UIElements::FocusChangeDirection {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
