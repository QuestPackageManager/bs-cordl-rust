#[cfg(feature = "Tayx+Graphy+Utils+G_ExtensionMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct G_ExtensionMethods {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Tayx+Graphy+Utils+G_ExtensionMethods")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Tayx::Graphy::Utils::G_ExtensionMethods =>
    "Tayx.Graphy.Utils"."G_ExtensionMethods"
);
#[cfg(feature = "Tayx+Graphy+Utils+G_ExtensionMethods")]
impl std::ops::Deref for crate::Tayx::Graphy::Utils::G_ExtensionMethods {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Tayx+Graphy+Utils+G_ExtensionMethods")]
impl std::ops::DerefMut for crate::Tayx::Graphy::Utils::G_ExtensionMethods {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Tayx+Graphy+Utils+G_ExtensionMethods")]
impl crate::Tayx::Graphy::Utils::G_ExtensionMethods {
    pub fn SetAllActive_List_1__cordl_bool0(
        gameObjects: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        >,
        active: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetAllActive", (gameObjects, active))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetAllActive_List_1__cordl_bool1(
        images: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
            >,
        >,
        active: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetAllActive", (images, active))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetOneActive(
        images: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
            >,
        >,
        active: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetOneActive", (images, active))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Tayx+Graphy+Utils+G_ExtensionMethods")]
impl quest_hook::libil2cpp::ObjectType
for crate::Tayx::Graphy::Utils::G_ExtensionMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
