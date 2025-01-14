#[cfg(feature = "CutScoreBuffer")]
#[repr(C)]
#[derive(Debug)]
pub struct CutScoreBuffer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _saberSwingRatingCounter: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SaberSwingRatingCounter,
    >,
    pub _noteCutInfo: crate::GlobalNamespace::NoteCutInfo,
    pub _noteScoreDefinition: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ScoreModel_NoteScoreDefinition,
    >,
    pub _afterCutScore: i32,
    pub _beforeCutScore: i32,
    pub _centerDistanceCutScore: i32,
    pub _initialized: bool,
    pub _isFinished: bool,
    pub _didFinishEvent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LazyCopyHashSet_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::ICutScoreBufferDidFinishReceiver,
            >,
        >,
    >,
    pub _didChangeEvent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LazyCopyHashSet_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::ICutScoreBufferDidChangeReceiver,
            >,
        >,
    >,
}
#[cfg(feature = "CutScoreBuffer")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::CutScoreBuffer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "CutScoreBuffer";
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
#[cfg(feature = "CutScoreBuffer")]
impl std::ops::Deref for crate::GlobalNamespace::CutScoreBuffer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CutScoreBuffer")]
impl std::ops::DerefMut for crate::GlobalNamespace::CutScoreBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CutScoreBuffer")]
impl crate::GlobalNamespace::CutScoreBuffer {
    pub fn HandleSaberSwingRatingCounterDidChange(
        &mut self,
        swingRatingCounter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ISaberSwingRatingCounter,
        >,
        rating: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::ISaberSwingRatingCounter,
                    >,
                    f32,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("HandleSaberSwingRatingCounterDidChange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleSaberSwingRatingCounterDidChange", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (swingRatingCounter, rating))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleSaberSwingRatingCounterDidFinish(
        &mut self,
        swingRatingCounter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ISaberSwingRatingCounter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ISaberSwingRatingCounter,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleSaberSwingRatingCounterDidFinish")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleSaberSwingRatingCounterDidFinish", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (swingRatingCounter))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        noteCutInfo: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::NoteCutInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::NoteCutInfo>),
                bool,
                1usize,
            >("Init")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Init", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (noteCutInfo)) };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RefreshScores(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("RefreshScores")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RefreshScores", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterDidChangeReceiver(
        &mut self,
        receiver: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ICutScoreBufferDidChangeReceiver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ICutScoreBufferDidChangeReceiver,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RegisterDidChangeReceiver")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterDidChangeReceiver", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (receiver))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterDidFinishReceiver(
        &mut self,
        receiver: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ICutScoreBufferDidFinishReceiver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ICutScoreBufferDidFinishReceiver,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RegisterDidFinishReceiver")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterDidFinishReceiver", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (receiver))
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterDidChangeReceiver(
        &mut self,
        receiver: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ICutScoreBufferDidChangeReceiver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ICutScoreBufferDidChangeReceiver,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("UnregisterDidChangeReceiver")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UnregisterDidChangeReceiver", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (receiver))
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterDidFinishReceiver(
        &mut self,
        receiver: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ICutScoreBufferDidFinishReceiver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ICutScoreBufferDidFinishReceiver,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("UnregisterDidFinishReceiver")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UnregisterDidFinishReceiver", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (receiver))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_afterCutScore(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_afterCutScore")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_afterCutScore", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_afterCutSwingRating(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_afterCutSwingRating")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_afterCutSwingRating", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_beforeCutScore(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_beforeCutScore")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_beforeCutScore", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_beforeCutSwingRating(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_beforeCutSwingRating")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_beforeCutSwingRating", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_centerDistanceCutScore(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_centerDistanceCutScore")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_centerDistanceCutScore", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_cutScore(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_cutScore")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_cutScore", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_executionOrder(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_executionOrder")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_executionOrder", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_isFinished(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isFinished")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_isFinished", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_maxPossibleCutScore(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_maxPossibleCutScore")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_maxPossibleCutScore", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_noteCutInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteCutInfo> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::GlobalNamespace::NoteCutInfo,
                0usize,
            >("get_noteCutInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_noteCutInfo", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::NoteCutInfo = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_noteScoreDefinition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ScoreModel_NoteScoreDefinition>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ScoreModel_NoteScoreDefinition,
                >,
                0usize,
            >("get_noteScoreDefinition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_noteScoreDefinition", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ScoreModel_NoteScoreDefinition,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "CutScoreBuffer")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::CutScoreBuffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "CutScoreBuffer")]
impl AsRef<crate::GlobalNamespace::IReadonlyCutScoreBuffer>
for crate::GlobalNamespace::CutScoreBuffer {
    fn as_ref(&self) -> &crate::GlobalNamespace::IReadonlyCutScoreBuffer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "CutScoreBuffer")]
impl AsMut<crate::GlobalNamespace::IReadonlyCutScoreBuffer>
for crate::GlobalNamespace::CutScoreBuffer {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IReadonlyCutScoreBuffer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "CutScoreBuffer")]
impl AsRef<crate::GlobalNamespace::ISaberSwingRatingCounterDidChangeReceiver>
for crate::GlobalNamespace::CutScoreBuffer {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::ISaberSwingRatingCounterDidChangeReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "CutScoreBuffer")]
impl AsMut<crate::GlobalNamespace::ISaberSwingRatingCounterDidChangeReceiver>
for crate::GlobalNamespace::CutScoreBuffer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::ISaberSwingRatingCounterDidChangeReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "CutScoreBuffer")]
impl AsRef<crate::GlobalNamespace::ISaberSwingRatingCounterDidFinishReceiver>
for crate::GlobalNamespace::CutScoreBuffer {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::ISaberSwingRatingCounterDidFinishReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "CutScoreBuffer")]
impl AsMut<crate::GlobalNamespace::ISaberSwingRatingCounterDidFinishReceiver>
for crate::GlobalNamespace::CutScoreBuffer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::ISaberSwingRatingCounterDidFinishReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
