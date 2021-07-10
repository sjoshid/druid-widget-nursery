use druid::im::{vector, Vector};
use druid::lens::InArc;
use druid::piet::TextStorage;
use druid::text::ParseFormatter;
use druid::widget::{
    CrossAxisAlignment, Flex, Label, List, RadioGroup, Scroll, TextBox, ValueTextBox,
};
use druid::{AppLauncher, ArcStr, Color, UnitPoint, Widget, WidgetExt, WindowDesc};
use druid::{Size, WidgetId};
use druid_widget_nursery::{AutoCompleteTextBox, Dropdown, FuzzySearchData};
use std::sync::{Arc, Mutex};

const ID_ONE: WidgetId = WidgetId::reserved(1);

fn main_widget() -> impl Widget<FuzzySearchData> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_spacer(10.)
        .with_child(
            Dropdown::new_sized(
                Flex::row()
                    .with_child(AutoCompleteTextBox::new().with_id(ID_ONE))
                    .with_flex_spacer(1.),
                |_, _| {
                    List::new(|| {
                        Label::new(|item: &ArcStr, _env: &_| {
                            format!("{}", String::from(item.as_str()))
                        })
                        .align_vertical(UnitPoint::LEFT)
                        .padding(10.0)
                        .background(Color::rgb(0.5, 0.5, 0.5))
                    })
                    .lens(FuzzySearchData::suggestions)
                },
                Size::from((100., 70.)),
            )
            .align_left(),
        )
        /*.with_child(
            ValueTextBox::new(TextBox::new(), ParseFormatter::new()).lens(FuzzySearchData::word),
        )*/
        .padding(10.)
        .fix_width(250.)
        .debug_widget_id()
}

pub fn main() {
    let main_window = WindowDesc::new(main_widget())
        .title("Dropdown")
        .window_size((250., 300.));

    let s = String::from("sujit");
    // create the initial app state
    let initial_state = FuzzySearchData {
        word: String::new(),
        suggestions: Arc::new(vec![s.into()]),
        tolerance: 3,
    };

    // start the application
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}
