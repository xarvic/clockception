use druid::{Widget, LifeCycle, EventCtx, PaintCtx, BoxConstraints, LifeCycleCtx, Size, LayoutCtx, Event, Env, UpdateCtx, RenderContext, Color, Point, Vec2};
use druid::kurbo::{Shape, PathEl};
use druid_widget_nursery::animation::{Animator, AnimationCurve, AnimationId, SimpleCurve, AnimationDirection};
use std::time::Duration;

pub struct Clock {
    animation_current: f64,
    animation_duration: f64,
    start_h1: f64,
    start_h2: f64,
    curr_h1: f64,
    curr_h2: f64,
}

impl Clock {
    pub fn new() -> Self {
        Clock {
            animation_current: 0.0,
            animation_duration: 0.8,
            start_h1: 0.0,
            start_h2: 0.0,
            curr_h1: 0.0,
            curr_h2: 0.0
        }
    }
    fn start_animation(&mut self) {
        self.animation_current = 0.0;
    }
}

impl Widget<(f64, f64)> for Clock {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut (f64, f64), _env: &Env) {
        if let Event::AnimFrame(nanos) = event {
            self.animation_current += *nanos as f64 * 0.000000001 / self.animation_duration;
            let mut curve = AnimationCurve::Simple(SimpleCurve::EaseInOut);
            let progress = curve.translate(self.animation_current).min(1.);

            self.curr_h1 = self.start_h1 + (data.0 - self.start_h1) * progress;
            self.curr_h2 = self.start_h2 + (data.1 - self.start_h2) * progress;

            ctx.request_paint();

            if self.animation_current < 1.0 {
                ctx.request_anim_frame();
            }
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &(f64, f64), _env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            self.start_h1 = data.0;
            self.curr_h1 = data.0;
            self.start_h2 = data.1;
            self.curr_h2 = data.1;
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &(f64, f64), data: &(f64, f64), env: &Env) {
        println!("update value!");
        self.start_h1 = old_data.0;
        self.start_h2 = old_data.1;
        self.start_animation();
        ctx.request_anim_frame();
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &(f64, f64), env: &Env) -> Size {
        bc.constrain_aspect_ratio(1.0, 50.0)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &(f64, f64), env: &Env) {
        let back = ctx.size().to_rect().to_ellipse();

        ctx.fill(back, &Color::GRAY);

        let center = Point::new(ctx.size().width / 2.0, ctx.size().height / 2.0);

        let vec0 = self.curr_h1.sin_cos();
        let vec1 = self.curr_h2.sin_cos();
        let radius = ctx.size().width / 2.0 - 2.0;

        let path = [
            PathEl::MoveTo(Point::new(center.x + vec0.1 * radius, center.y + vec0.0 * radius)),
            PathEl::LineTo(center),
            PathEl::LineTo(Point::new(center.x + vec1.1 * radius, center.y + vec1.0 * radius)),
        ];

        ctx.stroke(&path as &[PathEl], &Color::grey8(0), 2.5);
    }
}