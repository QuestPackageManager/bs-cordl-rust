#[cfg(feature = "ColorStateTransition_1")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorStateTransition_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::GlobalNamespace::BaseStateTransition_1<T>,
    pub _transition: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ColorTransitionSO,
    >,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "ColorStateTransition_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ColorStateTransition_1 < T > =>
    ""."ColorStateTransition`1" < T >
);
#[cfg(feature = "ColorStateTransition_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::GlobalNamespace::ColorStateTransition_1<T> {
    type Target = crate::GlobalNamespace::BaseStateTransition_1<T>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ColorStateTransition_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::GlobalNamespace::ColorStateTransition_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ColorStateTransition_1")]
impl<T: quest_hook::libil2cpp::Type> crate::GlobalNamespace::ColorStateTransition_1<T> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_disabledColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_disabledColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_highlightedColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_highlightedColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_normalColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_normalColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pressedColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_pressedColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedAndHighlightedColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_selectedAndHighlightedColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_selectedColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_transition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BaseTransitionSO>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BaseTransitionSO,
        > = __cordl_object.invoke("get_transition", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ColorStateTransition_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ColorStateTransition_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
