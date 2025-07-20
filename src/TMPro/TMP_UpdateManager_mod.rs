#[cfg(feature = "TMPro+TMP_UpdateManager")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_UpdateManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_LayoutQueueLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<i32>,
    >,
    pub m_LayoutRebuildQueue: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
        >,
    >,
    pub m_GraphicQueueLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<i32>,
    >,
    pub m_GraphicRebuildQueue: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
        >,
    >,
    pub m_InternalUpdateLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<i32>,
    >,
    pub m_InternalUpdateQueue: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
        >,
    >,
    pub m_CullingUpdateLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<i32>,
    >,
    pub m_CullingUpdateQueue: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
        >,
    >,
}
#[cfg(feature = "TMPro+TMP_UpdateManager")]
unsafe impl quest_hook::libil2cpp::Type for crate::TMPro::TMP_UpdateManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "TMP_UpdateManager";
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
#[cfg(feature = "TMPro+TMP_UpdateManager")]
impl std::ops::Deref for crate::TMPro::TMP_UpdateManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_UpdateManager")]
impl std::ops::DerefMut for crate::TMPro::TMP_UpdateManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_UpdateManager")]
impl crate::TMPro::TMP_UpdateManager {
    pub fn DoRebuilds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("DoRebuilds")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type >
                    ::class(), "DoRebuilds", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalRegisterTextElementForCullingUpdate(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("InternalRegisterTextElementForCullingUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type >
                    ::class(), "InternalRegisterTextElementForCullingUpdate", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (element))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalRegisterTextElementForGraphicRebuild(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("InternalRegisterTextElementForGraphicRebuild")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type >
                    ::class(), "InternalRegisterTextElementForGraphicRebuild", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (element))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalRegisterTextElementForLayoutRebuild(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("InternalRegisterTextElementForLayoutRebuild")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type >
                    ::class(), "InternalRegisterTextElementForLayoutRebuild", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (element))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalRegisterTextObjectForUpdate(
        &mut self,
        textObject: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("InternalRegisterTextObjectForUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type >
                    ::class(), "InternalRegisterTextObjectForUpdate", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (textObject))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalUnRegisterTextElementForGraphicRebuild(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("InternalUnRegisterTextElementForGraphicRebuild")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type >
                    ::class(), "InternalUnRegisterTextElementForGraphicRebuild", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (element))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalUnRegisterTextElementForLayoutRebuild(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("InternalUnRegisterTextElementForLayoutRebuild")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type >
                    ::class(), "InternalUnRegisterTextElementForLayoutRebuild", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (element))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalUnRegisterTextObjectForUpdate(
        &mut self,
        textObject: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("InternalUnRegisterTextObjectForUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type >
                    ::class(), "InternalUnRegisterTextObjectForUpdate", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (textObject))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnCameraPreCull(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnCameraPreCull")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type >
                    ::class(), "OnCameraPreCull", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterTextElementForCullingUpdate(
        element: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RegisterTextElementForCullingUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type >
                    ::class(), "RegisterTextElementForCullingUpdate", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (element))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterTextElementForGraphicRebuild(
        element: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RegisterTextElementForGraphicRebuild")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type >
                    ::class(), "RegisterTextElementForGraphicRebuild", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (element))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterTextElementForLayoutRebuild(
        element: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RegisterTextElementForLayoutRebuild")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type >
                    ::class(), "RegisterTextElementForLayoutRebuild", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (element))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterTextObjectForUpdate(
        textObject: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RegisterTextObjectForUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type >
                    ::class(), "RegisterTextObjectForUpdate", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (textObject))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnRegisterTextElementForRebuild(
        element: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("UnRegisterTextElementForRebuild")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type >
                    ::class(), "UnRegisterTextElementForRebuild", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (element))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnRegisterTextObjectForUpdate(
        textObject: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("UnRegisterTextObjectForUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type >
                    ::class(), "UnRegisterTextObjectForUpdate", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (textObject))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_instance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_UpdateManager>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::TMPro::TMP_UpdateManager>,
                0usize,
            >("get_instance")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::TMP_UpdateManager as quest_hook::libil2cpp::Type >
                    ::class(), "get_instance", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_UpdateManager> = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TMPro+TMP_UpdateManager")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_UpdateManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
