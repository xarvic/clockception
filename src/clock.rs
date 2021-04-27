use druid::{Widget, LifeCycle, EventCtx, PaintCtx, BoxConstraints, LifeCycleCtx, Size, LayoutCtx, Event, Env, UpdateCtx, RenderContext, Color, Point};
use druid::kurbo::PathEl;
use druid::piet::{StrokeStyle, LineJoin};
use druid_widget_nursery::animation::{SimpleCurve, Animated};
use std::time::Duration;

pub struct Clock {
    position: Animated<(f64, f64)>,
}

impl Clock {
    pub fn new() -> Self {
        Clock {
            position: Animated::new(
                (0.0, 0.0),
                Duration::from_secs_f64(0.5),
                SimpleCurve::EaseInOut,
                false,
            )
        }
    }
}

impl Default for Clock {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget<(f64, f64)> for Clock {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _: &mut (f64, f64), _env: &Env) {
        if let Event::AnimFrame(nanos) = event {
            self.position.update(*nanos, ctx);
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &(f64, f64), _env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            self.position.jump_to_value(*data);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _: &(f64, f64), data: &(f64, f64), _env: &Env) {
        self.position.animate(*data);
        if self.position.animating() {
            ctx.request_anim_frame();
        }
    }

    fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &(f64, f64), _env: &Env) -> Size {
        bc.constrain_aspect_ratio(1.0, 50.0)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &(f64, f64), _env: &Env) {
        let back = ctx.size().to_rect().to_ellipse();

        ctx.fill(back, &Color::GRAY);

        let center = Point::new(ctx.size().width / 2.0, ctx.size().height / 2.0);

        let vec0 = self.position.get().0.sin_cos();
        let vec1 = self.position.get().1.sin_cos();
        let radius = ctx.size().width / 2.0 - 2.0;

        let path = [
            PathEl::MoveTo(Point::new(center.x + vec0.1 * radius, center.y + vec0.0 * radius)),
            PathEl::LineTo(center),
            PathEl::LineTo(Point::new(center.x + vec1.1 * radius, center.y + vec1.0 * radius)),
        ];

        let style = StrokeStyle::new()
            .line_join(LineJoin::Round);

        ctx.stroke_styled(&path as &[PathEl], &Color::grey8(0), 2.5, &style);
    }
}