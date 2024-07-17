use bevy::prelude::*;

// Tag component used to tag entities added on the splash screen
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct SplashScreen;

// Newtype to use a `Timer` for this screen as a resource
#[derive(Resource, Deref, DerefMut)]
pub struct SplashTimer(pub Timer);


