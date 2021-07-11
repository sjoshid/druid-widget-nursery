use druid::widget::prelude::*;
use druid::widget::WidgetExt;
use druid::WindowSizePolicy;
use druid::{
    commands::CLOSE_WINDOW,
    widget::{Scroll, SizedBox},
};
use druid::{Point, Selector, WindowConfig};
use druid::{WindowId, WindowLevel};

pub struct Dropdown<T> {
    header: Box<dyn Widget<T>>,
    drop: Box<dyn Fn(&T, &Env) -> Box<dyn Widget<T>>>,
    window: Option<WindowId>,
}

pub const DROP: Selector<()> = Selector::new("druid-widget-nursery.dropdown.drop");
pub const CLOSE_DROP: Selector<()> = Selector::new("druid-widget-nursery.dropdown.close.drop");

impl<T: Data> Dropdown<T> {
    pub fn new<DW: Widget<T> + 'static>(
        header: impl Widget<T> + 'static,
        make_drop: impl Fn(&T, &Env) -> DW + 'static,
    ) -> Dropdown<T> {
        Dropdown {
            header: header.boxed(),
            drop: Box::new(move |d, e| make_drop(d, e).boxed()),
            window: None,
        }
    }

    pub fn new_sized<DW: Widget<T> + 'static>(
        header: impl Widget<T> + 'static,
        make_drop: impl Fn(&T, &Env) -> DW + 'static,
        size: Size,
    ) -> Dropdown<T> {
        Dropdown {
            header: header.boxed(),
            drop: Box::new(move |d, e| {
                SizedBox::new(Scroll::new(make_drop(d, e)))
                    .width(size.width)
                    .height(size.height)
                    .boxed()
            }),
            window: None,
        }
    }
}

impl<T: Data> Widget<T> for Dropdown<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::Command(n) if n.is(DROP) => {
                let widget = (self.drop)(data, env);
                //let origin = ctx.to_screen(Point::new(0., ctx.size().height));
                let origin = Point::new(
                    ctx.window_origin().x,
                    ctx.window_origin().y + ctx.size().height,
                );
                self.window = Some(
                    ctx.new_sub_window(
                        WindowConfig::default()
                            .set_level(WindowLevel::DropDown)
                            .set_position(origin)
                            .window_size_policy(WindowSizePolicy::Content)
                            .resizable(false)
                            .show_titlebar(false),
                        Dropped { child: widget },
                        data.clone(),
                        env.clone(),
                    ),
                );
                ctx.set_handled();
            }
            Event::Command(n) if n.is(CLOSE_DROP) => {
                if let Some(window) = self.window {
                    ctx.submit_command(CLOSE_WINDOW.to(window));
                    self.window = None;
                }
            }
            _ => {
                self.header.event(ctx, event, data, env);
            }
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.header.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.header.update(ctx, old_data, data, env)
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.header.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.header.paint(ctx, data, env)
    }
}

struct Dropped<T> {
    child: Box<dyn Widget<T>>,
}

impl<T: Data> Widget<T> for Dropped<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.child.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.child.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.child.update(ctx, old_data, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.child.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.child.paint(ctx, data, env)
    }
}
