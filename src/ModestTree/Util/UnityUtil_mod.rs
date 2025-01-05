#[cfg(feature = "ModestTree+Util+UnityUtil")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityUtil {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ModestTree+Util+UnityUtil")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::ModestTree::Util::UnityUtil => "ModestTree.Util"
    ."UnityUtil"
);
#[cfg(feature = "ModestTree+Util+UnityUtil")]
impl std::ops::Deref for crate::ModestTree::Util::UnityUtil {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ModestTree+Util+UnityUtil")]
impl std::ops::DerefMut for crate::ModestTree::Util::UnityUtil {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ModestTree+Util+UnityUtil")]
impl crate::ModestTree::Util::UnityUtil {
    pub fn GetAllGameObjects() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAllGameObjects", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllRootGameObjects() -> quest_hook::libil2cpp::Result<
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
            .invoke("GetAllRootGameObjects", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetComponentsInChildrenBottomUp(
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        includeInactive: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetComponentsInChildrenBottomUp", (gameObject, includeInactive))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetComponentsInChildrenTopDown(
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        includeInactive: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetComponentsInChildrenTopDown", (gameObject, includeInactive))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDepthLevel(
        transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDepthLevel", (transform))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDirectChildren(
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDirectChildren", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDirectChildrenAndSelf(
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDirectChildrenAndSelf", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParents(
        transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetParents", (transform))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParentsAndSelf(
        transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetParentsAndSelf", (transform))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRootParentOrSelf(
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRootParentOrSelf", (gameObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AllLoadedScenes() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::SceneManagement::Scene,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::SceneManagement::Scene,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_AllLoadedScenes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AllScenes() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::SceneManagement::Scene,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::SceneManagement::Scene,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_AllScenes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsAltKeyDown() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsAltKeyDown", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsControlKeyDown() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsControlKeyDown", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsShiftKeyDown() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsShiftKeyDown", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_WasAltKeyJustPressed() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_WasAltKeyJustPressed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_WasShiftKeyJustPressed() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_WasShiftKeyJustPressed", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ModestTree+Util+UnityUtil")]
impl quest_hook::libil2cpp::ObjectType for crate::ModestTree::Util::UnityUtil {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
