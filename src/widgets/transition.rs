use bevy::{platform::time::Instant, prelude::*};
use interpolation::Ease;
use interpolation::EaseFunction;

use crate::prelude::*;

/// A transition easing method.
#[derive(Default, Debug, Copy, Reflect, Clone, PartialEq)]
pub enum TransitionEasing {
    #[default]
    /// Linear
    Linear,
    /// QuadraticIn
    QuadraticIn,
    /// QuadraticOut
    QuadraticOut,
    /// QuadraticInOut
    QuadraticInOut,
    /// CubicIn
    CubicIn,
    /// CubicOut
    CubicOut,
    /// CubicInOut
    CubicInOut,
    /// QuarticIn
    QuarticIn,
    /// QuarticOut
    QuarticOut,
    /// QuarticInOut
    QuarticInOut,
    /// QuinticIn
    QuinticIn,
    /// QuinticOut
    QuinticOut,
    /// QuinticInOut
    QuinticInOut,
    /// SineIn
    SineIn,
    /// SineOut
    SineOut,
    /// SineInOut
    SineInOut,
    /// CircularIn
    CircularIn,
    /// CircularOut
    CircularOut,
    /// CircularInOut
    CircularInOut,
    /// ExponentialIn
    ExponentialIn,
    /// ExponentialOut
    ExponentialOut,
    /// ExponentialInOut
    ExponentialInOut,
    /// ElasticIn
    ElasticIn,
    /// ElasticOut
    ElasticOut,
    /// ElasticInOut
    ElasticInOut,
    /// BackIn
    BackIn,
    /// BackOut
    BackOut,
    /// BackInOut
    BackInOut,
    /// BounceIn
    BounceIn,
    /// BounceOut
    BounceOut,
    /// BounceInOut
    BounceInOut,
}

impl TransitionEasing {
    fn try_into_easing_function(&self) -> Option<EaseFunction> {
        match self {
            TransitionEasing::QuadraticIn => Some(EaseFunction::QuadraticIn),
            TransitionEasing::QuadraticOut => Some(EaseFunction::QuadraticOut),
            TransitionEasing::QuadraticInOut => Some(EaseFunction::QuadraticInOut),
            TransitionEasing::CubicIn => Some(EaseFunction::CubicIn),
            TransitionEasing::CubicOut => Some(EaseFunction::CubicOut),
            TransitionEasing::CubicInOut => Some(EaseFunction::CubicInOut),
            TransitionEasing::QuarticIn => Some(EaseFunction::QuarticIn),
            TransitionEasing::QuarticOut => Some(EaseFunction::QuarticOut),
            TransitionEasing::QuarticInOut => Some(EaseFunction::QuarticInOut),
            TransitionEasing::QuinticIn => Some(EaseFunction::QuinticIn),
            TransitionEasing::QuinticOut => Some(EaseFunction::QuinticOut),
            TransitionEasing::QuinticInOut => Some(EaseFunction::QuinticInOut),
            TransitionEasing::SineIn => Some(EaseFunction::SineIn),
            TransitionEasing::SineOut => Some(EaseFunction::SineOut),
            TransitionEasing::SineInOut => Some(EaseFunction::SineInOut),
            TransitionEasing::CircularIn => Some(EaseFunction::CircularIn),
            TransitionEasing::CircularOut => Some(EaseFunction::CircularOut),
            TransitionEasing::CircularInOut => Some(EaseFunction::CircularInOut),
            TransitionEasing::ExponentialIn => Some(EaseFunction::ExponentialIn),
            TransitionEasing::ExponentialOut => Some(EaseFunction::ExponentialOut),
            TransitionEasing::ExponentialInOut => Some(EaseFunction::ExponentialInOut),
            TransitionEasing::ElasticIn => Some(EaseFunction::ElasticIn),
            TransitionEasing::ElasticOut => Some(EaseFunction::ElasticOut),
            TransitionEasing::ElasticInOut => Some(EaseFunction::ElasticInOut),
            TransitionEasing::BackIn => Some(EaseFunction::BackIn),
            TransitionEasing::BackOut => Some(EaseFunction::BackOut),
            TransitionEasing::BackInOut => Some(EaseFunction::BackInOut),
            TransitionEasing::BounceIn => Some(EaseFunction::BounceIn),
            TransitionEasing::BounceOut => Some(EaseFunction::BounceOut),
            TransitionEasing::BounceInOut => Some(EaseFunction::BounceInOut),
            _ => None,
        }
    }
}

/// A transition bevy component that lets you animate Woodpecker UI widgets.
#[derive(Component, Debug, Reflect, Clone, Copy)]
pub struct Transition {
    /// Indicating the current playing status
    pub playing: bool,
    /// The easing function that dictates the interpolation factor.
    pub easing: TransitionEasing,
    /// Indicates the direction of the animation
    pub reversing: bool,
    /// The start time of the animation.
    pub start: Instant,
    /// The time in milliseconds until the animation completed.
    pub timeout: f32,
    /// Does the animation loop?
    ///
    /// TODO: Change from boolean to enum that allows disabling the reversing animation.
    pub looping: bool,
    /// The starting styles of the widget.
    pub style_a: WoodpeckerStyle,
    /// The ending styles of the widget.
    pub style_b: WoodpeckerStyle,
}

impl PartialEq for Transition {
    fn eq(&self, other: &Self) -> bool {
        self.playing == other.playing
            && self.easing == other.easing
            && self.reversing == other.reversing
            && self.timeout == other.timeout
            && self.looping == other.looping
            && self.style_a == other.style_a
            && self.style_b == other.style_b
    }
}

impl Default for Transition {
    fn default() -> Self {
        Self {
            playing: true,
            easing: Default::default(),
            reversing: Default::default(),
            start: Instant::now(),
            timeout: Default::default(),
            looping: Default::default(),
            style_a: Default::default(),
            style_b: Default::default(),
        }
    }
}

impl Transition {
    pub(crate) fn update(&mut self) -> WoodpeckerStyle {
        // as Milliseconds
        let elapsed_time = self.start.elapsed().as_secs_f32() * 1000.0;
        if (elapsed_time < self.timeout) && self.playing {
            let mut x = if let Some(easing) = self.easing.try_into_easing_function() {
                Ease::calc((elapsed_time / self.timeout).clamp(0.0, 1.0), easing)
            } else {
                (elapsed_time / self.timeout).clamp(0.0, 1.0)
            };
            if self.reversing {
                x = 1.0 - x;
            }
            self.style_a.lerp(&self.style_b, x)
        } else if self.looping && self.playing {
            // Restart animation
            self.start = Instant::now();
            self.reversing = !self.reversing;
            if self.reversing {
                self.style_b
            } else {
                self.style_a
            }
        } else {
            // End of animation
            self.playing = false;
            if self.reversing {
                self.style_a
            } else {
                self.style_b
            }
        }
    }

    /// Is the animation currently playing?
    pub fn is_playing(&self) -> bool {
        self.playing
    }

    /// Starts the animation.
    pub fn start(&mut self) {
        self.reversing = false;
        self.start = Instant::now();
        self.playing = true;
    }

    /// Starts the animation in reverse.
    pub fn start_reverse(&mut self) {
        self.reversing = true;
        self.start = Instant::now();
        self.playing = true;
    }
}

pub(crate) fn update_transitions(mut query: Query<(&mut Transition, &mut WoodpeckerStyle)>) {
    for (mut transition, mut styles) in query.iter_mut() {
        let new_styles = transition.update();
        *styles = new_styles;
    }
}
